use crate::backend::AudioBackend;
use crate::{AudioDevice, DeviceType};
use anyhow::{Context, Result};
use std::process::Command;

pub struct PipeWireBackend;

impl PipeWireBackend {
    pub fn new() -> Self {
        Self
    }

    fn run_wpctl(&self, args: &[&str]) -> Result<String> {
        let output = Command::new("wpctl")
            .args(args)
            .output()
            .context("Failed to execute wpctl. Is PipeWire installed?")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("wpctl failed: {}", stderr);
        }

        Ok(String::from_utf8(output.stdout)?)
    }

    fn run_pw_metadata(&self, args: &[&str]) -> Result<String> {
        let output = Command::new("pw-metadata")
            .args(args)
            .output()
            .context("Failed to execute pw-metadata. Is PipeWire installed?")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("pw-metadata failed: {}", stderr);
        }

        Ok(String::from_utf8(output.stdout)?)
    }

    fn run_pw_cli(&self, args: &[&str]) -> Result<String> {
        let output = Command::new("pw-cli")
            .args(args)
            .output()
            .context("Failed to execute pw-cli. Is PipeWire installed?")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("pw-cli failed: {}", stderr);
        }

        Ok(String::from_utf8(output.stdout)?)
    }

    fn parse_devices(&self, output: &str, device_type: DeviceType) -> Result<Vec<AudioDevice>> {
        let section_name = match device_type {
            DeviceType::Sink => "Sinks:",
            DeviceType::Source => "Sources:",
        };

        let mut devices = Vec::new();
        let mut in_audio_section = false;
        let mut in_target_section = false;

        for line in output.lines() {
            if line.trim() == "Audio" {
                in_audio_section = true;
                continue;
            }

            if in_audio_section && (line.trim() == "Video" || line.trim() == "Settings") {
                break;
            }

            if in_audio_section && line.contains(section_name) {
                in_target_section = true;
                continue;
            }

            if in_target_section && line.contains("├─") && !line.contains(section_name) {
                in_target_section = false;
                continue;
            }

            if in_target_section && in_audio_section {
                if let Some(device) = self.parse_device_line(line, device_type) {
                    devices.push(device);
                }
            }
        }

        Ok(devices)
    }

    /// Get the object.serial for a given device ID
    fn get_device_serial(&self, device_id: u32) -> Result<String> {
        let output = self.run_pw_cli(&["info", &device_id.to_string()])?;

        for line in output.lines() {
            if line.contains("object.serial") {
                // Extract value between quotes: *		object.serial = "2062"
                if let Some(start) = line.find('"') {
                    if let Some(end) = line.rfind('"') {
                        if start < end {
                            return Ok(line[start + 1..end].to_string());
                        }
                    }
                }
            }
        }

        anyhow::bail!("Could not find object.serial for device {}", device_id)
    }

    /// Get all active stream IDs for the given device type
    fn get_active_streams(&self, device_type: DeviceType) -> Result<Vec<u32>> {
        let output = self.run_wpctl(&["status"])?;
        let mut stream_ids = Vec::new();
        let mut in_audio_section = false;
        let mut in_streams_section = false;

        for line in output.lines() {
            // Look for the Audio section
            if line.trim() == "Audio" {
                in_audio_section = true;
                continue;
            }

            // Exit Audio section when we hit Video or Settings
            if in_audio_section && (line.trim() == "Video" || line.trim() == "Settings") {
                break;
            }

            // Look for Streams subsection within Audio
            if in_audio_section && line.contains("└─ Streams:") {
                in_streams_section = true;
                continue;
            }

            // Parse stream lines - they're indented and have a number followed by a dot
            if in_streams_section && in_audio_section {
                let trimmed = line.trim_start();

                // Stream lines look like: "        56. Firefox"
                // They don't have │ or ├ or └ characters
                if !trimmed.contains("│") && !trimmed.contains("├") && !trimmed.contains("└") {
                    if let Some(dot_pos) = trimmed.find('.') {
                        let id_part = &trimmed[..dot_pos].trim();
                        if let Ok(id) = id_part.parse::<u32>() {
                            // Verify this is actually a stream by checking device type
                            if self.is_stream_for_device_type(id, device_type).unwrap_or(false) {
                                stream_ids.push(id);
                            }
                        }
                    }
                }
            }
        }

        Ok(stream_ids)
    }

    /// Check if a stream ID is for the given device type (sink vs source)
    fn is_stream_for_device_type(&self, stream_id: u32, device_type: DeviceType) -> Result<bool> {
        let output = self.run_pw_cli(&["info", &stream_id.to_string()])?;

        for line in output.lines() {
            if line.contains("media.class") {
                let line_lower = line.to_lowercase();
                return Ok(match device_type {
                    DeviceType::Sink => line_lower.contains("output"),
                    DeviceType::Source => line_lower.contains("input"),
                });
            }
        }

        Ok(false)
    }

    /// Move a stream to a target device using pw-metadata
    fn move_stream_to_device(&self, stream_id: u32, device_serial: &str) -> Result<()> {
        self.run_pw_metadata(&[
            "-n",
            "default",
            &stream_id.to_string(),
            "target.object",
            device_serial,
        ])
        .context(format!(
            "Failed to move stream {} to device with serial {}",
            stream_id, device_serial
        ))?;
        Ok(())
    }

    /// Parse a single device line from wpctl status
    /// Format: " │  *   58. Family 17h/19h/1ah HD Audio Controller Speaker [vol: 0.54]"
    /// Or:     " │      73. Radeon High Definition Audio Controller [...] [vol: 0.40]"
    fn parse_device_line(&self, line: &str, device_type: DeviceType) -> Option<AudioDevice> {
        if !line.contains("│") || !line.contains(".") {
            return None;
        }

        let is_default = line.contains("*");

        let line = line
            .replace("│", "")
            .replace("├─", "")
            .replace("└─", "")
            .replace("*", "")
            .trim()
            .to_string();

        let parts: Vec<&str> = line.splitn(2, '.').collect();
        if parts.len() != 2 {
            return None;
        }

        let id = parts[0].trim().parse::<u32>().ok()?;

        let name_part = parts[1].trim();
        let description = if let Some(vol_pos) = name_part.rfind("[vol:") {
            name_part[..vol_pos].trim()
        } else {
            name_part
        }
        .to_string();

        Some(AudioDevice {
            id,
            name: description.clone(),
            description,
            is_default,
            device_type,
        })
    }
}

impl Default for PipeWireBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioBackend for PipeWireBackend {
    fn list_devices(&self, device_type: DeviceType) -> Result<Vec<AudioDevice>> {
        let output = self.run_wpctl(&["status"])?;
        self.parse_devices(&output, device_type)
    }

    fn set_default(&self, device_id: u32) -> Result<()> {
        self.run_wpctl(&["set-default", &device_id.to_string()])
            .context("Failed to set default device")?;
        Ok(())
    }

    fn set_default_and_move_streams(
        &self,
        device_id: u32,
        device_type: DeviceType,
        move_streams: bool,
    ) -> Result<()> {
        // First set the default device
        self.set_default(device_id)?;

        // If move_streams is requested, move all active streams
        if move_streams {
            // Get the object.serial for the target device
            let device_serial = self.get_device_serial(device_id)?;

            // Get all active streams for this device type
            let stream_ids = self.get_active_streams(device_type)?;

            // Move each stream to the new device
            for stream_id in stream_ids {
                // Ignore errors when moving individual streams
                // Some streams might not be movable
                let _ = self.move_stream_to_device(stream_id, &device_serial);
            }
        }

        Ok(())
    }

    fn name(&self) -> &str {
        "pipewire"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backend_creation() {
        let backend = PipeWireBackend::new();
        assert_eq!(backend.name(), "pipewire");
    }

    #[test]
    fn test_parse_sink_devices() {
        let backend = PipeWireBackend::new();
        let sample_output = r#"Audio
 ├─ Devices:
 │      44. Radeon High Definition Audio Controller [Rembrandt/Strix] [alsa]
 │      45. Family 17h/19h/1ah HD Audio Controller [alsa]
 │
 ├─ Sinks:
 │  *   58. Family 17h/19h/1ah HD Audio Controller Speaker [vol: 0.54]
 │      73. Radeon High Definition Audio Controller [Rembrandt/Strix] HDMI / DisplayPort 4 Output [vol: 0.40]
 │     112. Radeon High Definition Audio Controller [Rembrandt/Strix] HDMI / DisplayPort 1 Output [vol: 1.00]
 │
 ├─ Sources:
 │      59. Family 17h/19h/1ah HD Audio Controller Headphones Stereo Microphone [vol: 1.00]
 │  *   82. USB Audio Analog Stereo             [vol: 0.69]

Video
 ├─ Devices:
        "#;

        let devices = backend
            .parse_devices(sample_output, DeviceType::Sink)
            .unwrap();

        assert_eq!(devices.len(), 3);

        // Check first device (default)
        assert_eq!(devices[0].id, 58);
        assert_eq!(
            devices[0].description,
            "Family 17h/19h/1ah HD Audio Controller Speaker"
        );
        assert!(devices[0].is_default);

        // Check second device
        assert_eq!(devices[1].id, 73);
        assert!(!devices[1].is_default);

        // Check third device
        assert_eq!(devices[2].id, 112);
        assert!(!devices[2].is_default);
    }

    #[test]
    fn test_parse_source_devices() {
        let backend = PipeWireBackend::new();
        let sample_output = r#"Audio
 ├─ Sinks:
 │  *   58. Family 17h/19h/1ah HD Audio Controller Speaker [vol: 0.54]
 │
 ├─ Sources:
 │      59. Family 17h/19h/1ah HD Audio Controller Headphones Stereo Microphone [vol: 1.00]
 │      60. Family 17h/19h/1ah HD Audio Controller Digital Microphone [vol: 0.96]
 │  *   82. USB Audio Analog Stereo             [vol: 0.69]

Video
        "#;

        let devices = backend
            .parse_devices(sample_output, DeviceType::Source)
            .unwrap();

        assert_eq!(devices.len(), 3);

        assert_eq!(devices[2].id, 82);
        assert_eq!(devices[2].description, "USB Audio Analog Stereo");
        assert!(devices[2].is_default);

        assert!(!devices[0].is_default);
        assert!(!devices[1].is_default);
    }

    #[test]
    fn test_parse_device_line() {
        let backend = PipeWireBackend::new();

        let line = " │  *   58. Family 17h/19h/1ah HD Audio Controller Speaker [vol: 0.54]";
        let device = backend.parse_device_line(line, DeviceType::Sink).unwrap();
        assert_eq!(device.id, 58);
        assert_eq!(
            device.description,
            "Family 17h/19h/1ah HD Audio Controller Speaker"
        );
        assert!(device.is_default);

        let line = " │      73. Radeon High Definition Audio Controller [Rembrandt/Strix] HDMI / DisplayPort 4 Output [vol: 0.40]";
        let device = backend.parse_device_line(line, DeviceType::Sink).unwrap();
        assert_eq!(device.id, 73);
        assert!(!device.is_default);

        let line = " │  ├─ Devices:";
        assert!(backend.parse_device_line(line, DeviceType::Sink).is_none());
    }
}

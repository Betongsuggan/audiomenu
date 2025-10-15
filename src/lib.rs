pub mod backend;
pub mod cli;
pub mod launcher;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceType {
    Sink,
    Source,
}

impl std::fmt::Display for DeviceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeviceType::Sink => write!(f, "sink"),
            DeviceType::Source => write!(f, "source"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AudioDevice {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub is_default: bool,
    pub device_type: DeviceType,
}

impl AudioDevice {
    pub fn format_for_display(&self) -> String {
        let device_emoji = match self.device_type {
            DeviceType::Sink => "🔊",
            DeviceType::Source => "🎤",
        };

        let status = if self.is_default { "✓" } else { " " };

        let hint = self.get_device_hint();
        let hint_str = if !hint.is_empty() {
            format!(" {}", hint)
        } else {
            String::new()
        };

        format!(
            "{} {} {}{} (ID: {})",
            status, device_emoji, self.description, hint_str, self.id
        )
    }

    fn get_device_hint(&self) -> String {
        let name_lower = self.description.to_lowercase();

        if name_lower.contains("hdmi") || name_lower.contains("displayport") {
            "📺".to_string()
        } else if name_lower.contains("usb") || name_lower.contains("arctis") {
            "🎧".to_string()
        } else if name_lower.contains("speaker") {
            "🔈".to_string()
        } else if name_lower.contains("headphone") {
            "🎧".to_string()
        } else if name_lower.contains("digital microphone") || name_lower.contains("mic") {
            "🎙️".to_string()
        } else {
            "".to_string()
        }
    }

    pub fn parse_id_from_selection(selection: &str) -> Option<u32> {
        selection
            .split("(ID: ")
            .nth(1)?
            .split(')')
            .next()?
            .parse()
            .ok()
    }
}

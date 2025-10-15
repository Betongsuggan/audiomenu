pub mod pipewire;

use crate::{AudioDevice, DeviceType};
use anyhow::Result;

pub trait AudioBackend {
    fn list_devices(&self, device_type: DeviceType) -> Result<Vec<AudioDevice>>;

    fn set_default(&self, device_id: u32) -> Result<()>;

    fn set_default_and_move_streams(
        &self,
        device_id: u32,
        device_type: DeviceType,
        move_streams: bool,
    ) -> Result<()>;

    fn name(&self) -> &str;
}

#[derive(Debug, Clone, Copy)]
pub enum Backend {
    PipeWire,
    PulseAudio,
}

impl Backend {
    pub fn create(self) -> Box<dyn AudioBackend> {
        match self {
            Backend::PipeWire => Box::new(pipewire::PipeWireBackend::new()),
            Backend::PulseAudio => unimplemented!("PulseAudio backend not yet implemented"),
        }
    }
}

impl std::str::FromStr for Backend {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "pipewire" => Ok(Backend::PipeWire),
            "pulseaudio" => Ok(Backend::PulseAudio),
            _ => Err(anyhow::anyhow!("Unknown backend: {}", s)),
        }
    }
}

impl std::fmt::Display for Backend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Backend::PipeWire => write!(f, "pipewire"),
            Backend::PulseAudio => write!(f, "pulseaudio"),
        }
    }
}

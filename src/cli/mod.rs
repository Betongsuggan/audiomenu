use crate::backend::Backend;
use crate::launcher::Launcher;
use crate::DeviceType;
use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(name = "audiomenu")]
#[command(author, version, about, long_about = None)]
#[command(about = "Launcher-driven audio device manager for Linux")]
pub struct Cli {
    #[arg(value_enum)]
    pub device_type: CliDeviceType,

    #[arg(short, long, value_enum, default_value = "walker")]
    pub launcher: CliLauncher,

    #[arg(short, long, value_enum, default_value = "pipewire")]
    pub backend: CliBackend,

    #[arg(long, default_value = "2")]
    pub spaces: usize,

    #[arg(short = 'm', long, default_value = "true")]
    pub move_streams: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum CliDeviceType {
    Sink,
    Source,
}

impl From<CliDeviceType> for DeviceType {
    fn from(cli: CliDeviceType) -> Self {
        match cli {
            CliDeviceType::Sink => DeviceType::Sink,
            CliDeviceType::Source => DeviceType::Source,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum CliLauncher {
    Walker,
    Rofi,
    Dmenu,
    Fuzzel,
}

impl From<CliLauncher> for Launcher {
    fn from(cli: CliLauncher) -> Self {
        match cli {
            CliLauncher::Walker => Launcher::Walker,
            CliLauncher::Rofi => Launcher::Rofi,
            CliLauncher::Dmenu => Launcher::Dmenu,
            CliLauncher::Fuzzel => Launcher::Fuzzel,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum CliBackend {
    Pipewire,
    Pulseaudio,
}

impl From<CliBackend> for Backend {
    fn from(cli: CliBackend) -> Self {
        match cli {
            CliBackend::Pipewire => Backend::PipeWire,
            CliBackend::Pulseaudio => Backend::PulseAudio,
        }
    }
}

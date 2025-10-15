/// Example: List all audio devices without launching a menu
use audiomenu::{backend::Backend, DeviceType};

fn main() -> anyhow::Result<()> {
    let backend = Backend::PipeWire.create();

    println!("=== Audio Output Devices (Sinks) ===");
    let sinks = backend.list_devices(DeviceType::Sink)?;
    if sinks.is_empty() {
        println!("  No output devices found");
    } else {
        for device in &sinks {
            println!("{}", device.format_for_display());
        }
    }

    println!("\n=== Audio Input Devices (Sources) ===");
    let sources = backend.list_devices(DeviceType::Source)?;
    if sources.is_empty() {
        println!("  No input devices found");
    } else {
        for device in &sources {
            println!("{}", device.format_for_display());
        }
    }

    Ok(())
}

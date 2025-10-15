/// Test the full device selection flow with debug output
use audiomenu::{backend::Backend, launcher::Launcher, AudioDevice, DeviceType};

fn main() -> anyhow::Result<()> {
    println!("=== Testing Full Flow ===\n");

    // Create backend
    let backend = Backend::PipeWire.create();
    let launcher = Launcher::Walker;
    let device_type = DeviceType::Sink;

    // List devices
    println!("1. Listing devices...");
    let devices = backend.list_devices(device_type)?;
    println!("   Found {} devices\n", devices.len());

    // Format for display
    println!("2. Formatting for launcher:");
    let options: Vec<String> = devices
        .iter()
        .map(|d| d.format_for_display())
        .collect();

    for opt in &options {
        println!("   {}", opt);
    }
    println!();

    // Show menu
    println!("3. Launching Walker menu...");
    println!("   (Select a device)\n");

    let prompt = format!("Select {} device:", device_type);
    let selection = launcher.show_menu(&options, Some(&prompt))?;

    println!("4. User selected: {}", selection);

    // Parse ID
    let device_id = AudioDevice::parse_id_from_selection(&selection)
        .ok_or_else(|| anyhow::anyhow!("Failed to parse device ID from selection"))?;

    println!("   Parsed device ID: {}\n", device_id);

    // Set default
    println!("5. Executing: wpctl set-default {}", device_id);
    backend.set_default(device_id)?;

    println!("   ✓ Command executed\n");

    // Verify
    println!("6. Verifying change...");
    let updated_devices = backend.list_devices(device_type)?;
    if let Some(default) = updated_devices.iter().find(|d| d.is_default) {
        println!("   Current default: {} (ID: {})", default.description, default.id);
        if default.id == device_id {
            println!("   ✓ SUCCESS! Device was changed correctly!");
        } else {
            println!("   ✗ WARNING: Default is still ID {}, not {}", default.id, device_id);
        }
    }

    Ok(())
}

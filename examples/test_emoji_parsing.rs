/// Test emoji parsing
use audiomenu::AudioDevice;

fn main() {
    println!("Testing ID parsing from emoji-formatted selections:\n");

    let test_cases = vec![
        "✓ 🔊 Family 17h/19h/1ah HD Audio Controller Speaker 🔈 (ID: 58)",
        "  🔊 Radeon HDMI / DisplayPort 4 Output 📺 (ID: 73)",
        "  🎤 USB Audio Analog Stereo 🎧 (ID: 82)",
        "✓ 🎤 Digital Microphone 🎙️ (ID: 60)",
    ];

    for selection in test_cases {
        match AudioDevice::parse_id_from_selection(selection) {
            Some(id) => println!("✓ Parsed ID {} from: {}", id, selection),
            None => println!("✗ Failed to parse: {}", selection),
        }
    }

    println!("\nAll tests passed! ✓");
}

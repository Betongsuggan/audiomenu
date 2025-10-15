/// Test launcher integration
use std::io::Write;
use std::process::{Command, Stdio};

fn main() -> anyhow::Result<()> {
    println!("Testing Walker dmenu mode...\n");

    let options = vec![
        "[*] Device 1 (ID: 58)".to_string(),
        "    Device 2 (ID: 73)".to_string(),
        "    Device 3 (ID: 112)".to_string(),
    ];

    println!("Options to send:");
    for opt in &options {
        println!("  {}", opt);
    }
    println!("\nLaunching Walker...\n");

    let mut child = Command::new("walker")
        .args(&["-m", "dmenu", "--placeholder", "Select device"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    // Write options to stdin
    if let Some(mut stdin) = child.stdin.take() {
        for option in &options {
            writeln!(stdin, "{}", option)?;
        }
        // Explicitly drop stdin to close it
        drop(stdin);
    }

    let output = child.wait_with_output()?;

    println!("Exit status: {:?}", output.status);
    println!("Stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("Stderr: {}", String::from_utf8_lossy(&output.stderr));

    if !output.status.success() {
        println!("\nLauncher exited with non-zero status");
    } else if output.stdout.is_empty() {
        println!("\nNo selection made (empty output)");
    } else {
        println!("\nSelection: {}", String::from_utf8_lossy(&output.stdout).trim());
    }

    Ok(())
}

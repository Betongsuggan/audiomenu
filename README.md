# audiomenu

**Launcher-driven audio device manager for Linux**

`audiomenu` is a Rust-based tool that allows you to select and switch audio devices (inputs and outputs) through your favorite launcher (Walker, Rofi, dmenu, Fuzzel).

Inspired by [iwmenu](https://github.com/e-tho/iwmenu) and [bzmenu](https://github.com/e-tho/bzmenu), `audiomenu` brings the same launcher-driven interface to audio device management.

## Features

- 🎧 **Output Device Selection**: Switch between speakers, headphones, HDMI outputs, etc.
- 🎤 **Input Device Selection**: Switch between microphones and other input devices
- 🔄 **Multiple Backends**: Support for PipeWire (PulseAudio coming soon)
- 🚀 **Launcher Integration**: Works with Walker, Rofi, dmenu, and Fuzzel
- ⚡ **Fast & Lightweight**: Written in Rust for performance

## Installation

### Using Nix Flakes

```bash
# Build the project
nix build

# Run directly
nix run . -- sink --launcher walker

# Add to your flake.nix
inputs.audiomenu.url = "path:./audiomenu";
```

### Using Cargo

```bash
cargo build --release
sudo cp target/release/audiomenu /usr/local/bin/
```

## Usage

```bash
# Select output device (sink) with Walker
audiomenu sink --launcher walker

# Select input device (source) with Rofi
audiomenu source --launcher rofi

# Use with dmenu
audiomenu sink --launcher dmenu

# Specify backend explicitly (default is pipewire)
audiomenu sink --backend pipewire
```

## Supported Backends

- ✅ **PipeWire** (via `wpctl`)
- 🚧 **PulseAudio** (planned)

## Supported Launchers

- ✅ Walker
- ✅ Rofi
- ✅ dmenu
- ✅ Fuzzel

## Project Structure

```
audiomenu/
├── src/
│   ├── main.rs           # Entry point
│   ├── lib.rs            # Core types
│   ├── backend/          # Audio backend implementations
│   │   ├── mod.rs        # Backend trait
│   │   └── pipewire.rs   # PipeWire implementation
│   ├── cli/              # CLI argument parsing
│   │   └── mod.rs
│   └── launcher/         # Launcher integrations
│       └── mod.rs
├── Cargo.toml
├── flake.nix            # Nix flake for building
└── README.md
```

## Development

### Prerequisites

- Rust 1.70+
- PipeWire with `wpctl`
- One of: Walker, Rofi, dmenu, or Fuzzel

### Building

```bash
# With cargo
cargo build

# With Nix
nix build
```

### Testing

```bash
cargo test
```

### Development Shell

```bash
nix develop
```

## License

GPL-3.0

## Acknowledgments

- Inspired by [iwmenu](https://github.com/e-tho/iwmenu) and [bzmenu](https://github.com/e-tho/bzmenu)
- Built for use with [Walker launcher](https://github.com/abenz1267/walker)

{ lib, rustPlatform, pkg-config, pipewire, wireplumber }:

rustPlatform.buildRustPackage {
  pname = "audiomenu";
  version = "0.1.0";

  src = ./.;

  cargoLock = { lockFile = ./Cargo.lock; };

  nativeBuildInputs = [ pkg-config ];

  buildInputs = [ pipewire wireplumber ];

  meta = with lib; {
    description = "Launcher-driven audio device manager for Linux";
    homepage = "https://github.com/yourusername/audiomenu";
    license = licenses.gpl3;
    maintainers = [ ];
    platforms = platforms.linux;
  };
}

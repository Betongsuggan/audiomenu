{
  lib,
  rustPlatform,
  makeWrapper,
  pipewire,
  wireplumber,
}:

rustPlatform.buildRustPackage {
  pname = "audiomenu";
  version = "0.1.0";

  src = lib.fileset.toSource {
    root = ./.;
    fileset = lib.fileset.unions [
      ./Cargo.toml
      ./Cargo.lock
      ./src
    ];
  };

  cargoLock.lockFile = ./Cargo.lock;

  nativeBuildInputs = [ makeWrapper ];

  # wpctl, pw-cli and pw-metadata are called at runtime. Suffixed, so the
  # session's own PipeWire tools win and these are only a fallback
  postInstall = ''
    wrapProgram $out/bin/audiomenu \
      --suffix PATH : ${
        lib.makeBinPath [
          pipewire
          wireplumber
        ]
      }
  '';

  meta = {
    description = "Launcher-driven audio device manager for Linux";
    homepage = "https://github.com/Betongsuggan/audiomenu";
    license = lib.licenses.gpl3Only;
    mainProgram = "audiomenu";
    platforms = lib.platforms.linux;
  };
}

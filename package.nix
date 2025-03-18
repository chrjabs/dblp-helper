{
  lib,
  rustPlatform,
  openssl,
  pkg-config,
}: let
  manifest = (lib.importTOML ./Cargo.toml).package;
in
  rustPlatform.buildRustPackage {
    pname = manifest.name;
    version = manifest.version;

    src = lib.fileset.toSource {
      root = ./.;
      fileset = lib.fileset.unions [
        ./Cargo.lock
        ./Cargo.toml
        ./src
      ];
    };
    cargoLock.lockFile = ./Cargo.lock;

    useNextest = true;

    buildInputs = [openssl];
    nativeBuildInputs = [pkg-config];

    meta = with lib; {
      description = manifest.description;
      homepage = manifest.homepage;
      license = licenses.mit;
      platforms = platforms.all;
      mainProgram = "dblp";
    };
  }

{
  description = "dblp-tools";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";

    flake-parts.url = "github:hercules-ci/flake-parts";

    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs =
    inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } (_: {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
      perSystem =
        {
          config,
          system,
          pkgs,
          ...
        }:
        let
          lib = pkgs.lib;
        in
        {
          _module.args.pkgs = import inputs.nixpkgs {
            inherit system;
            overlays =
              let
                toolchain-overlay = _: super: {
                  rust-toolchain = super.symlinkJoin {
                    name = "rust-toolchain";
                    paths = [
                      ((pkgs.extend (import inputs.rust-overlay)).rust-bin.fromRustupToolchainFile ./rust-toolchain.toml)
                    ];
                    buildInputs = [ super.makeWrapper ];
                    postBuild = ''
                      wrapProgram $out/bin/cargo --set LIBCLANG_PATH ${super.libclang.lib}/lib
                    '';
                  };
                };
              in
              [
                toolchain-overlay
              ];
          };

          packages.default = pkgs.callPackage ./package.nix {
            rustPlatform = pkgs.makeRustPlatform {
              cargo = pkgs.rust-toolchain;
              rustc = pkgs.rust-toolchain;
            };
          };

          devShells.default =
            let
              libs = with pkgs; [
                openssl
                xz
                bzip2
              ];
            in
            pkgs.mkShell.override { stdenv = pkgs.clangStdenv; } rec {
              nativeBuildInputs = with pkgs; [
                pkg-config
                rust-toolchain
                cargo-nextest
              ];
              buildInputs = libs;
              LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
              LD_LIBRARY_PATH = lib.makeLibraryPath libs;
              PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig/";
            };
        };
    });
}

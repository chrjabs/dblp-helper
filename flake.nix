{
  description = "dblp-tools";

  inputs = {
    nix-config.url = "github:chrjabs/nix-config";

    nixpkgs.url = "github:nixos/nixpkgs";
    nixpkgs.follows = "nix-config/nixpkgs";
    systems.url = "github:nix-systems/default-linux";

    nixpkgs-edge.url = "github:nixos/nixpkgs";

    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = inputs @ {
    self,
    nixpkgs,
    nixpkgs-edge,
    systems,
    rust-overlay,
    nix-config,
  }: let
    lib = nixpkgs.lib;
    pkgsFor = lib.genAttrs (import systems) (system: (import nixpkgs {
      inherit system;
      overlays =
        [
          (import rust-overlay)
          (final: _: {cargo-auditable = nixpkgs-edge.legacyPackages.${system}.cargo-auditable;})
        ]
        ++ builtins.attrValues nix-config.overlays;
    }));
    forEachSystem = f: lib.genAttrs (import systems) (system: f pkgsFor.${system});
  in {
    devShells = forEachSystem (pkgs: {
      default = let
        libs = with pkgs; [openssl];
      in
        pkgs.mkShell.override {stdenv = pkgs.clangStdenv;} rec {
          nativeBuildInputs = with pkgs; [
            pkg-config
            (rust-bin.fromRustupToolchainFile ./rust-toolchain.toml)
            cargo-nextest
          ];
          buildInputs = libs;
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath libs;
          LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
          PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig/";
        };
    });

    formatter = forEachSystem (pkgs: pkgs.alejandra);

    packages = forEachSystem (pkgs: {
      default = pkgs.callPackage ./package.nix {
        rustPlatform = pkgs.makeRustPlatform {
          cargo = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
          rustc = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        };
      };
    });
  };
}

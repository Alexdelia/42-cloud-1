{
  description = "cloud-1 frontend flake";

  # FIXME: use treefmt for formatting https://github.com/numtide/treefmt-nix
  # TODO: look into https://github.com/srid/leptos-fullstack/tree/master
  # TODO: look into jenkins
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    flake-utils.url = "github:numtide/flake-utils";

    rust-overlay.url = "github:oxalica/rust-overlay";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    # treefmt-nix.url = "github:numtide/treefmt-nix";

    pre-commit-hooks = {
      url = "github:cachix/git-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    nixpkgs,
    rust-overlay,
    flake-utils,
    ...
  } @ inputs:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        inherit (pkgs) lib;

        base = rec {
          rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
          cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);

          craneLib = (inputs.crane.mkLib pkgs).overrideToolchain rustToolchain;

          common = {
            pname = cargoToml.package.name;
            version = cargoToml.package.version;

            src = lib.fileset.toSource {
              root = ./.;
              fileset = lib.fileset.unions [
                (craneLib.fileset.commonCargoSources ./.)
                (
                  lib.fileset.fileFilter
                  (file: lib.any file.hasExt ["html" "scss"])
                  ./.
                )
                (lib.fileset.maybeMissing ./public)
              ];
            };

            nativeBuildInputs = with pkgs; [
              openssl
              pkg-config

              cargo-leptos
              dart-sass
              binaryen # provides wasm-opt
            ];
          };

          cargoArtifacts = craneLib.buildDepsOnly common;
        };
      in {
        # nix build
        packages.default = pkgs.callPackage ./default.nix {inherit pkgs base;};

        # nix flake check
        check = base.craneLib.cargoClippy (base.common
          // {
            inherit (base) cargoArtifacts;
            cargoClippyExtraArgs = "--all-targets --all-features -- --deny warnings";
          });

        # nix develop
        devShells.default = pkgs.callPackage ./shell.nix {inherit pkgs inputs base;};
      }
    );
}

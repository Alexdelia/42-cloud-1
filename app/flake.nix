{
  description = "cloud-1 frontend flake";

  # FIXME: use treefmt for formatting https://github.com/numtide/treefmt-nix
  # TODO: look into https://github.com/srid/leptos-fullstack/tree/master
  # TODO: look into jenkins
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    flake-utils.url = "github:numtide/flake-utils";

    rust-overlay.url = "github:oxalica/rust-overlay";

    crane.url = "github:ipetkov/crane";

    treefmt-nix.url = "github:numtide/treefmt-nix";

    pre-commit-hooks = {
      url = "github:cachix/git-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
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
            inherit (cargoToml.package) version;

            src = lib.fileset.toSource {
              root = ./.;
              fileset = lib.fileset.unions [
                (craneLib.fileset.commonCargoSources ./.)
                (
                  lib.fileset.fileFilter
                  (file: lib.any file.hasExt ["html" "scss"])
                  ./.
                )
                (
                  lib.fileset.fileFilter
                  (file: lib.any file.hasExt ["json"])
                  ./.sqlx
                )
                (
                  lib.fileset.fileFilter
                  (file: lib.any file.hasExt ["sql"])
                  ./migrations
                )
                (lib.fileset.maybeMissing ./public)
                (lib.fileset.maybeMissing ./.env)
              ];
            };

            nativeBuildInputs = with pkgs; [
              openssl
              pkg-config

              rustToolchain

              cargo-leptos
              dart-sass
              binaryen # provides wasm-opt
            ];
          };

          cargoArtifacts = craneLib.buildDepsOnly common;
        };

        treefmtEval = inputs.treefmt-nix.lib.evalModule pkgs ./treefmt.nix;
      in {
        # nix build
        packages.default = pkgs.callPackage ./default.nix {inherit pkgs base;};

        # nix flake check
        checks = {
          cargo-clippy = base.craneLib.cargoClippy (base.common
            // {
              inherit (base) cargoArtifacts;
              cargoClippyExtraArgs = "--all-targets --all-features -- --deny warnings";
            });

          formatting = treefmtEval.config.build.check self;
        };

        # nix develop
        devShells.default = pkgs.callPackage ./shell.nix {inherit pkgs inputs base;};

        # nix fmt
        formatter = treefmtEval.config.build.wrapper;
      }
    );
}

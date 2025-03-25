{
  pkgs,
  inputs,
}: let
  inherit (pkgs) lib;

  rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

  craneLib = (inputs.crane.mkLib pkgs).overrideToolchain rustToolchain;

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
in
  craneLib.buildPackage {
    inherit src;
  }

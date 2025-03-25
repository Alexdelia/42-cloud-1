{
  pkgs,
  inputs,
}: let
  lib = pkgs.lib;
  craneLib = inputs.crane.mkLib pkgs;

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

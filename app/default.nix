{
  pkgs,
  inputs,
}: let
  craneLib = inputs.crane.mkLib pkgs;
in
  craneLib.buildPackage {
    src = craneLib.cleanCargoSource ./.;
  }

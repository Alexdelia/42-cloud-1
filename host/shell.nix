{
  pkgs,
  inputs,
}:
pkgs.mkShell {
  packages = [
    inputs.deploy-rs.packages.${pkgs.system}.default
  ];
}

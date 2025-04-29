{
  modulesPath,
  lib,
  pkgs,
  ...
}: {
  imports = [
    (modulesPath + "/installer/scan/not-detected.nix")
    (modulesPath + "/profiles/qemu-guest.nix")
    ./disk-config.nix

    ./hardware-configuration.nix

    ./bootloader.nix
    ./openssh.nix
  ];

  environment.systemPackages = map lib.lowPrio (with pkgs; [
    curl
    gitMinimal
  ]);

  system.stateVersion = "24.05";
}

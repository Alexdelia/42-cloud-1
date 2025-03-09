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
  ];
  boot.loader.grub = {
    # no need to set devices, disko will add all devices that have a EF02 partition to the list already
    # devices = [ ];
    efiSupport = true;
    efiInstallAsRemovable = true;
  };
  services.openssh.enable = true;

  environment.systemPackages = map lib.lowPrio [
    pkgs.curl
    pkgs.gitMinimal
  ];

  users.users.root.openssh.authorizedKeys.keys = [
    "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIMIfTLm+mUFmMfydIkJioTceAtrN9BEXkbbhqGFjBUeL alex@jiruo"
    "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIMpZkrT0NGEbVnXwVRtuAngLlIQQkEqDHypDzDOdyOKa alex@qemu"
  ];

  system.stateVersion = "24.05";
}

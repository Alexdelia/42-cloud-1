{
  lib,
  pkgs,
  hostname,
  user,
  ...
}: {
  imports = [
    ./hardware-configuration.nix

    ./openssh.nix

    ./user.nix

    ./docker.nix
    ./nginx.nix
  ];

  nix = {
    settings = {
      experimental-features = ["nix-command" "flakes"];

      trusted-users = [user];
    };
  };

  networking.hostName = hostname;

  environment.systemPackages = map lib.lowPrio (with pkgs; [
    curl
    gitMinimal
  ]);

  security.sudo.wheelNeedsPassword = false;

  system.stateVersion = "24.11";
}

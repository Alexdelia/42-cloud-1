{
  lib,
  pkgs,
  hostname,
  ...
}: {
  imports = [
    ./hardware-configuration.nix

    ./openssh.nix

    ./user.nix
  ];

  nix = {
    settings = {
      experimental-features = ["nix-command" "flakes"];
    };
  };

  networking.hostName = hostname;

  environment.systemPackages = map lib.lowPrio (with pkgs; [
    curl
    gitMinimal
  ]);

  system.stateVersion = "24.11";
}

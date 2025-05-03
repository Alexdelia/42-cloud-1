{
  lib,
  pkgs,
  ...
}: {
  imports = [
    ./hardware-configuration.nix

    ./openssh.nix
  ];

  nix = {
    settings = {
      experimental-features = ["nix-command" "flakes"];
    };
  };

  environment.systemPackages = map lib.lowPrio (with pkgs; [
    curl
    gitMinimal
  ]);

  system.stateVersion = "24.11";
}

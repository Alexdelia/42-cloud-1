{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    deploy-rs = {
      url = "github:serokell/deploy-rs";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    deploy-rs,
    ...
  }: let
    system = "x86_64-linux";
    hostname = "cloud-1";
    user = "alex";
  in {
    nixosConfigurations.cloud-1 = nixpkgs.lib.nixosSystem {
      inherit system;

      modules = [
        ./src
      ];

      specialArgs = {
        inherit hostname user;
      };
    };

    deploy.nodes.${hostname} = {
      inherit hostname;

      profiles.system = {
        inherit user;
        path = deploy-rs.lib.${system}.activate.nixos self.nixosConfigurations.${hostname};
      };
    };

    checks = builtins.mapAttrs (system: deployLib: deployLib.deployChecks self.deploy) deploy-rs.lib;
  };
}

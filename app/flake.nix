{
  description = "cloud-1 frontend flake";

  # FIXME: use treefmt for formatting https://github.com/numtide/treefmt-nix
  # TODO: look into https://github.com/srid/leptos-fullstack/tree/master
  # TODO: look into jenkins
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    flake-utils.url = "github:numtide/flake-utils";

    rust-overlay.url = "github:oxalica/rust-overlay";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    # treefmt-nix.url = "github:numtide/treefmt-nix";

    pre-commit-hooks = {
      url = "github:cachix/git-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    nixpkgs,
    rust-overlay,
    flake-utils,
    ...
  } @ inputs:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in {
        # build
        packages.default = pkgs.callPackage ./default.nix {inherit pkgs inputs;};

        # dev shell
        devShells.default = pkgs.callPackage ./shell.nix {inherit pkgs inputs;};
      }
    );
}

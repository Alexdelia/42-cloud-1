{
  pkgs,
  inputs,
  base,
}: let
  pre-commit-check = inputs.pre-commit-hooks.lib.${pkgs.system}.run {
    src = ./.;
    hooks = {
      typos = {
        enable = true;
        stages = ["pre-commit"];
      };

      check-toml = {
        enable = true;
        stages = ["pre-commit"];
      };

      /*
      clippy = {
        enable = false;
        stages = ["pre-push"];
        settings = {
          extraArgs = "--manifest-path=./app/Cargo.toml";
          # allFeatures = true;
        };
      };
      */

      nix-check = {
        enable = true;

        name = "nix flake check";
        entry = "bash -c 'cd ./app && nix flake check \"$@\"'";

        pass_filenames = false;
        always_run = true;
        stages = ["pre-push"];
      };

      nix-fmt = {
        enable = true;

        name = "nix fmt";
        entry = "bash -c 'cd ./app && nix fmt \"$@\"'";

        pass_filenames = false;
        stages = ["pre-commit"];
      };
    };
  };
in
  pkgs.mkShell {
    buildInputs =
      base.common.nativeBuildInputs
      ++ pre-commit-check.buildInputs;

    # for rust-analyzer 'hover' tooltips to work.
    RUST_SRC_PATH = "${base.rustToolchain}/lib/rustlib/src/rust/library";

    shellHook =
      /*
      bash
      */
      ''
        ${pre-commit-check.shellHook}

        if [ ! -f .env ]; then
        	cp .env.example .env
        	printf "\n\n\t\033[1mplease look at the \033[35m.env\033[39m file\033[0m\n\n"
        fi

        alias serve="cargo leptos serve"
        alias watch="cargo leptos watch"

        alias fmt="nix fmt"
      '';
  }

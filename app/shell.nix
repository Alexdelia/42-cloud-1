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

      nix-check = {
        enable = false; # is using .env which is not in the repo

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
      ++ pre-commit-check.buildInputs
      ++ (with pkgs; [sqlx-cli]);

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
        	printf "\n\n\t\033[1mplease look at the \033[35m.env\033[39m file and \033[32medit\033[39m as needed\033[0m\n"
        	printf "\t\033[1myou can then run \033[32msource .env\033[0m\n\n"
        else
        	source .env
        fi

        alias serve="cargo leptos serve"
        alias watch="cargo leptos watch"

        alias fmt="nix fmt"

        alias deploy="DOCKER_HOST='ssh://alex@cloud-1' docker compose up --detach"
      '';
  }

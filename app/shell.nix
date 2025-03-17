{
  pkgs,
  inputs,
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

      clippy = {
        enable = false;
        stages = ["pre-push"];
      };
      rustfmt = {
        enable = true;
        stages = ["pre-commit"];
        settings = {
          manifest-path = "./app/Cargo.toml";
        };
      };

      prettier = {
        enable = true;
        stages = ["pre-commit"];
      };
    };
  };
in
  pkgs.mkShell {
    buildInputs = with pkgs;
      [
        openssl
        pkg-config
        (rust-bin.stable.latest.default.override {
          # extensions = ["rust-src"];
          targets = ["wasm32-unknown-unknown"];
        })
      ]
      ++ pre-commit-check.buildInputs;

    packages = with pkgs; [
      cargo-leptos
      dart-sass

      leptosfmt
      nodePackages.prettier
    ];

    shellHook =
      /*
      bash
      */
      ''
        ${pre-commit-check.shellHook}

        alias serve="cargo leptos serve"
        alias watch="cargo leptos watch"

        alias fmt="leptosfmt ./src/**/*.rs && prettier --write ./style/**/*css --cache --log-level=warn"
      '';
  }

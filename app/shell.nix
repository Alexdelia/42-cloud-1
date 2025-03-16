{
  pkgs,
  inputs,
}: let
  pre-commit-check = inputs.pre-commit-hooks.lib.${pkgs.system}.run {
    src = ./.;
    hooks = {
      typos.enable = true;

      # check-toml.enable = true;
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
      sass

      leptosfmt
    ];

    shellHook =
      /*
      bash
      */
      ''
        ${pre-commit-check.shellHook}

        alias serve="cargo leptos serve"
        alias watch="cargo leptos watch"
      '';
  }

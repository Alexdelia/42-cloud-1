{pkgs ? import <nixpkgs> {}}:
pkgs.mkShell {
  buildInputs = with pkgs; [
    openssl
    pkg-config
    (rust-bin.stable.latest.default.override {
      # extensions = ["rust-src"];
      targets = ["wasm32-unknown-unknown"];
    })
  ];

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
      alias serve="cargo leptos serve"
      alias watch="cargo leptos watch"
    '';
}

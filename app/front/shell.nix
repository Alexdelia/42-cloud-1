{pkgs ? import <nixpkgs> {}}:
pkgs.mkShell {
  buildInputs = with pkgs; [
    # openssl
    # pkg-config
    (rust-bin.stable.latest.default.override {
      # extensions = ["rust-src"];
      targets = ["wasm32-unknown-unknown"];
    })
  ];

  packages = with pkgs; [
    wasm-pack

    httplz
  ];

  shellHook = ''
    alias build="wasm-pack build --target web"

    alias serve="${pkgs.httplz}/bin/httplz"
  '';
}

{pkgs ? import <nixpkgs> {}}:
pkgs.mkShell {
  buildInputs = with pkgs; [
    # openssl
    # pkg-config
    (rust-bin.stable.latest.default.override {
      # extensions = ["rust-src"];
      targets = ["wasm32-unknown-unknown"];
    })
    wasm-pack
    cargo-generate
  ];

  # shellHook = ''
  # '';
}

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
    trunk
    stylance-cli

    leptosfmt
  ];

  shellHook = let
    serve = "${pkgs.trunk}/bin/trunk serve";
  in
    /*
    bash
    */
    ''
      alias serve="${serve}"
      alias open="${serve} --open"
    '';
}

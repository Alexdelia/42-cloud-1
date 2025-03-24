{pkgs ? import <nixpkgs> {}}:
pkgs.rustPlatform.buildRustPackage {
  pname = "cloud-1";
  version = "0.3.0";

  src = pkgs.lib.cleanSource ./.;

  cargoSha256 = pkgs.lib.fakeSha256;
  cargoLock = {
    lockFile = ./Cargo.lock;

    outputHashes = {
    };
  };

  postInstall = ''
    ls -lahR $out | tee $out/ls.txt
  '';
}

{
  pkgs,
  base,
}:
base.craneLib.buildPackage {
  inherit (base.common) src pname version;
  inherit (base) cargoArtifacts;

  buildPhaseCargoCommand = ''
    cargoBuildLog=$(mktemp cargoBuildLogXXXX.json)
    SQLX_OFFLINE=true cargo leptos build --release -vvv
  '';
  cargoTestCommand = "SQLX_OFFLINE=true cargo leptos test --release -vvv #";

  nativeBuildInputs = with pkgs;
    [
      makeWrapper
    ]
    ++ base.common.nativeBuildInputs;

  installPhaseCommand =
    /*
    bash
    */
    ''
      mkdir -p $out/bin

      cp target/release/${base.common.pname} $out/bin/
      cp -r target/site $out/bin/

      wrapProgram $out/bin/${base.common.pname} \
      	--set LEPTOS_SITE_ROOT $out/bin/site
    '';
}

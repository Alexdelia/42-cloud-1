_: {
  projectRootFile = "rust-toolchain.toml";

  programs = {
    rustfmt = {
      enable = true;
      edition = "2024";
    };
    leptosfmt.enable = true;

    alejandra.enable = true;
    deadnix.enable = true;
    statix.enable = true;

    prettier = {
      enable = true;
      settings = {
        useTabs = true;
      };
    };
  };

  settings.global.excludes = [
    ".gitingore"

    "*.lock"

    ".env*"

    "*.png"
    "*.ico"

    "*.toml"
    "Dockerfile"
  ];
}

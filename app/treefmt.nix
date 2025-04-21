_: {
  projectRootFile = "rust-toolchain.toml";

  programs = {
    rustfmt = {
      enable = true;
      edition = "2024";
    };
    leptosfmt.enable = true;

    sqlfluff = {
      enable = true;
      dialect = "postgres";
    };
    sqlfluff-lint.enable = true; # inherit dialect from sqlfluff

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
    ".dockerignore"

    "*.lock"

    ".env*"

    "*.png"
    "*.ico"

    "*.toml"
    "Dockerfile"
  ];
}

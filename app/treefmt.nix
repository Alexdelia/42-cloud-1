_: {
  projectRootFile = "rust-toolchain.toml";

  programs = {
    rustfmt = {
      enable = true;
      edition = "2024";
    };

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
}

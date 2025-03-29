{...}: {
  projectRootFile = "rust-toolchain.toml";

  programs = {
    alejandra = {
      enable = true;
    };
  };
}

{ ... }:

{
  languages.rust = {
    enable = true;
    toolchainFile = ./rust-toolchain.toml;
  };

  git-hooks.hooks = {
    rustfmt.enable = true;

    clippy = {
      enable = true;
      settings.denyWarnings = true;
    };
  };
}

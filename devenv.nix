{ pkgs, ... }:
{
  cachix.enable = false;

  languages.rust = {
    enable = true;
    channel = "stable";
    version = "1.85.0";
  };

  packages = [
    pkgs.pkg-config
    pkgs.openssl
    pkgs.bun
  ];
}

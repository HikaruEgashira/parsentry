{ pkgs, ... }:
{
  languages.rust = {
    enable = true;
    channel = "stable";
  };

  packages = [
    pkgs.pkg-config
    pkgs.openssl
    pkgs.bun
  ];
}

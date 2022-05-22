{ pkgs ? <nixpkgs> }:
pkgs.mkShell {
  buildInputs = with pkgs;
    (pkgs.lib.optionals pkgs.stdenv.isLinux ([ udev ])) ++ [
      # solana-basic
      cargo-deps
      cargo-udeps
      cargo-outdated

      goki-cli

      # sdk
      yarn
      nodejs
      python3

      pkgconfig
      openssl
      jq

      libiconv
    ] ++ (pkgs.lib.optionals pkgs.stdenv.isDarwin
      (with pkgs.darwin.apple_sdk.frameworks; [ AppKit IOKit Foundation ]));
}

{ pkgs }:
pkgs.mkShell {
  buildInputs = with pkgs;
    (pkgs.lib.optionals pkgs.stdenv.isLinux ([ libudev ])) ++ [
      solana-cli
      cargo-deps
      cargo-watch
      cargo-udeps

      # sdk
      (yarn.override { nodejs = nodejs-14_x; })
      nodejs-14_x
      python3

      pkgconfig
      openssl
      jq

      libiconv
    ] ++ (pkgs.lib.optionals pkgs.stdenv.isDarwin
      (with pkgs.darwin.apple_sdk.frameworks; [ AppKit IOKit Foundation ]));
}

{ src, rustPlatform, solana-cli, osSpecificPackages, pkgconfig, openssl }:

rustPlatform.buildRustPackage rec {
  pname = "goki-cli";
  version = "0.1.3";
  inherit src;

  cargoLock = { lockFile = ./Cargo.lock; };
  verifyCargoDeps = true;
  strictDeps = true;

  nativeBuildInputs = [ pkgconfig ];
  buildInputs = osSpecificPackages ++ [ openssl solana-cli ];
}

{ src, rustPlatform, solana-cli, osSpecificPackages }:

rustPlatform.buildRustPackage rec {
  pname = "goki-cli";
  version = "0.1.3";
  inherit src;

  cargoLock = { lockFile = ./Cargo.lock; };
  verifyCargoDeps = true;
  strictDeps = true;

  buildInputs = osSpecificPackages ++ [ solana-cli ];
}

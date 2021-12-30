{ lib, version, src, rustPlatform, solana-cli, osSpecificPackages, pkgconfig
, openssl }:

rustPlatform.buildRustPackage rec {
  pname = "goki-cli";
  inherit version src;

  cargoLock = { lockFile = ./Cargo.lock; };
  verifyCargoDeps = true;
  strictDeps = true;

  nativeBuildInputs = [ pkgconfig ];
  buildInputs = osSpecificPackages ++ [ openssl solana-cli ];

  meta = with lib; {
    homepage = "https://goki.so";
    description = "Goki command line tools.";

    license = licenses.agpl3;
    platforms = platforms.unix;
  };
}

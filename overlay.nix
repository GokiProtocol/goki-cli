final: prev:
let
  osSpecificPackages = with prev;
    (lib.optionals stdenv.isDarwin (with darwin.apple_sdk.frameworks;
    ([ IOKit Security CoreFoundation AppKit ]
    ++ (lib.optionals stdenv.isAarch64 [ System ]))))
    ++ (lib.optionals stdenv.isLinux [ udev ]);
in
{
  goki-cli = import ./default.nix {
    inherit (prev) lib rustPlatform pkgconfig openssl;
    solana-basic = prev.solana.solana-basic;
    inherit osSpecificPackages;
    version = "0.2.3";
    src = builtins.path
      {
        path = ./.;
        name = "goki-cli";
        filter = path: type:
          let
            includeFiles = [
              "Cargo.toml"
              "Cargo.lock"
              "src"
            ];
          in
          # Only include the above files in the generated derivation
          (prev.lib.lists.any (p: (prev.lib.hasInfix p path)) includeFiles);
      };
  };
}

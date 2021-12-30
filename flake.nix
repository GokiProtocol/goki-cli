{
  description = "Goki CLI build and development environment.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    saber-overlay.url = "github:saber-hq/saber-overlay";
    flake-utils.url = "github:numtide/flake-utils";
    gitignore = {
      url = "github:hercules-ci/gitignore.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, saber-overlay, flake-utils, gitignore }:
    flake-utils.lib.eachSystem [
      "aarch64-darwin"
      "x86_64-darwin"
      "x86_64-linux"
    ] (system:
      let
        pkgs = import nixpkgs { inherit system; }
          // saber-overlay.packages.${system};

        osSpecificPackages = with pkgs;
          (lib.optionals stdenv.isDarwin (with darwin.apple_sdk.frameworks;
            ([ IOKit Security CoreFoundation AppKit ]
              ++ (lib.optionals stdenv.isAarch64 [ System ]))))
          ++ (lib.optionals stdenv.isLinux [ libudev ]);

        goki-cli = import ./default.nix {
          inherit (pkgs) lib solana-cli rustPlatform pkgconfig openssl;
          inherit osSpecificPackages;
          version = "0.1.4";
          src = gitignore.lib.gitignoreSource ./.;
        };
      in {
        packages.goki-cli = goki-cli;
        defaultPackage = goki-cli;
        devShell = import ./shell.nix { inherit pkgs; };
      });
}

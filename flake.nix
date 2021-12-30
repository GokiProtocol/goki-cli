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
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; }
          // saber-overlay.packages.${system};

        osSpecificPackages = (pkgs.lib.optionals pkgs.stdenv.isDarwin
          (with pkgs.darwin.apple_sdk.frameworks;
            ([ IOKit Security CoreFoundation AppKit ]
              ++ (pkgs.lib.optionals pkgs.stdenv.isAarch64 [ System ]))))
          ++ (pkgs.lib.optionals pkgs.stdenv.isLinux [ pkgs.libudev ]);

        goki-cli = import ./default.nix {
          inherit (pkgs) solana-cli rustPlatform;
          inherit osSpecificPackages;
          src = gitignore.lib.gitignoreSource ./.;
        };
      in {
        packages.goki-cli = goki-cli;
        defaultPackage = goki-cli;
        devShell = import ./shell.nix { inherit pkgs; };
      });
}

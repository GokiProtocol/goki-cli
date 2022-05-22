{
  description = "Goki CLI build and development environment.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    saber-overlay.url = "github:saber-hq/saber-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, saber-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlay = import ./overlay.nix;
        pkgs = import nixpkgs
          {
            inherit system;
            overlays = [
              saber-overlay.overlay
              overlay
            ];
          };
      in
      {
        packages = with pkgs; {
          inherit goki-cli;
          default = goki-cli;
        };
        devShells = {
          default = import ./shell.nix { inherit pkgs; };
        };
      });
}

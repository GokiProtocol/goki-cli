{
  description = "Goki CLI build and development environment.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    saber-overlay.url = "github:saber-hq/saber-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, saber-overlay, flake-utils }:
    let
      overlay = import ./overlay.nix;
      defaultOverlay = final: prev:
        (nixpkgs.lib.composeExtensions saber-overlay.overlays.default overlay) final prev;
    in
    {
      overlays = {
        default = defaultOverlay;
        basic = overlay;
      };
    } // flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs
          {
            inherit system;
            overlays = [
              saber-overlay.overlays.default
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

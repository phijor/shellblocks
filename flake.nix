{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crate2nix.url = "github:nix-community/crate2nix";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      crate2nix,
      flake-utils,
      ...
    }:
    {
      overlays.default = import ./overlay.nix;
    }
    // flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ self.overlays.default ];
        };
      in
      {
        packages = rec {
          inherit (pkgs) shellblocks;
          default = shellblocks;
        };
        devShells = {
          default = pkgs.mkShell { packages = [ crate2nix.packages.${system}.default ]; };
        };
        formatter = pkgs.nixfmt-rfc-style;
      }
    );
}

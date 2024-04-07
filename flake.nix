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
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        cargo = import ./Cargo.nix { inherit pkgs; };
        shellblocks = cargo.rootCrate.build;
      in
      {
        packages = {
          inherit shellblocks;
          default = shellblocks;
        };
        devShells = {
          default = pkgs.mkShell { packages = [ crate2nix.packages.${system}.default ]; };
        };
        formatter = pkgs.nixfmt-rfc-style;
      }
    );
}

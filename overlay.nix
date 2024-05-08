final: prev:
let
  cargo = import ./Cargo.nix { pkgs = final; };
in
{
  shellblocks = cargo.rootCrate.build;
}

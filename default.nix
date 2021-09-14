{ pkgs ? import ./nixpkgs.nix {} }:

with pkgs;

let
  inherit (rust.packages.stable) rustPlatform;
in

{
  devhub = stdenv.mkDerivation rec {
    name = "devhub";
    src = gitignoreSource ./.;

    buildInputs = [
      holochain
      hc
      lair-keystore
      cargo
      jq
    ];

    nativeBuildInputs = [
    ];
  };
}

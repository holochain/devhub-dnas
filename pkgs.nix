{ pkgs, system }:

import (pkgs.fetchFromGitHub {
  owner = "spartan-holochain-counsel";
  repo = "nix-overlay";
  rev = "a023aa5b8babb5bacf9284a29cf14911a33592d2";
  sha256 = "Dv8CJgOjRdPYOiBJ7mTGL5g9WRzPY5qQByu7D0EW01U=";
}) {
  inherit pkgs;
  inherit system;
}

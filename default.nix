let
  holonixPath = builtins.fetchTarball {
    url = "https://github.com/holochain/holonix/archive/6ae8ffb8e5c1a1faa4f4e1af8a9f7139b2ce0f3c.tar.gz";
    sha256 = "0ksvy2m7gpxf5r7l1rznvjwajypk7iii3h5d8nx762kah55nyliq";
  };
  holonix = import (holonixPath) {
    include = {
      holochainBinaries = true;
      node = false;
      happs = false;
    };

    holochainVersionId = "custom";
    holochainVersion = { # v0.0.106
      rev = "b11908875a9f6a09e8939fbf6f45ff658e3d10a6"; # Sep 16, 2021
      sha256 = "074jdpr0dmzp2wl7flal1rdcnhd86bbf9g8dmzq4wl616v3ibqzs";
      cargoSha256 = "0k1bbh7cklnhzkj06icbi7wamq6hl6q77d51k43qil3mrvddb7j0";

      lairKeystoreHashes = { # v0.0.4
        sha256 = "0khg5w5fgdp1sg22vqyzsb2ri7znbxiwl7vr2zx6bwn744wy2cyv";
        cargoSha256 = "1lm8vrxh7fw7gcir9lq85frfd0rdcca9p7883nikjfbn21ac4sn4";
      };

      bins = {
        holochain = "holochain";
        hc = "hc";
      };
    };
  };
  nixpkgs = holonix.pkgs;
in nixpkgs.mkShell {
  inputsFrom = [ holonix.main ];
}

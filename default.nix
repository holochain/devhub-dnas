let
  holonixPath = builtins.fetchTarball { # main as of Mar 15, 2022
    url = "https://github.com/holochain/holonix/archive/391557dc5b3065b0d357ea9f9a2bc77e7347be8e.tar.gz";
    sha256 = "10dnbd3s8gm4bl7my7c168vyvi3358s1lb5yjnw3fwnp9z62vy09";
  };
  holonix = import (holonixPath) {
    include = {
      holochainBinaries = true;
      node = false;
      scaffolding = false;
      happs = false;
    };

    holochainVersionId = "custom";
    holochainVersion = {
      url = "https://github.com/holochain/holochain";
      rev = "holochain-0.0.143"; # Jun 8, 2022 - 7f204047c56a2c165b1442cd480828a03caadde2
      sha256 = "1qcsj76i8ig1lavm1nyr50w2grnqnmxar01hmky1zsdz3a2blzsx";
      cargoLock = {
        outputHashes = {
        };
      };

      binsFilter = [
        "holochain"
        "hc"
        "kitsune-p2p-tx2-proxy"
      ];

      rustVersion = "1.58.1";

      lair = {
        url = "https://github.com/holochain/lair";
        rev = "lair_keystore-v0.1.3"; # May 5, 2022 - 27e3a4e305e2a5d48ba625aa3bfac9516d2583ed
        sha256 = "0xisp3rqdnjsypxpjcin94qwsgvb99vwisq49jjl6x7qxl2s3afm";

        binsFilter = [
          "lair-keystore"
        ];

        rustVersion = "1.58.1";

        cargoLock = {
          outputHashes = {
          };
        };
      };
    };
  };
  nixpkgs = holonix.pkgs;
in nixpkgs.mkShell {
  inputsFrom = [ holonix.main ];
}

let
  holonixPath = builtins.fetchTarball { # main as of Sep 1, 2022
    url = "https://github.com/holochain/holonix/archive/a983ff292331d7553efadc5ab3916d5c2197dcee.tar.gz";
    sha256 = "0zpkw7wppdxl3pznkb39i7svfhg8pc0ly87n89sxsczj1fb17028";
  };
  holonix = import (holonixPath) {
    include = {
      holochainBinaries = true;
      node = false;
      scaffolding = false;
      happs = false;
    };

    rustVersion = {
      track = "stable";
      version = "1.63.0";
    };

    holochainVersionId = "custom";
    holochainVersion = {
      url = "https://github.com/holochain/holochain";
      rev = "holochain-0.0.175"; # Nov 30, 2022 - 64286bf7aa10e9e93032361a69843daa354fdb1b
      sha256 = "0djcycqp2g2ci9gr03v5bh2r7419bcgp06fwb9hh2asa6qm18909";
      cargoLock = {
        outputHashes = {
        };
      };

      binsFilter = [
        "holochain"
        "hc"
        # "kitsune-p2p-tx2-proxy"
      ];

      lair = {
        url = "https://github.com/holochain/lair";
        rev = "lair_keystore-v0.2.2"; # Nov 2, 2022 - 004d15c05f51bb625cb5c8db6e71f4a58ff4d3fe
        sha256 = "1kh2p6lyw3yi9ah8ci6y0815w6fnmwplrkl2wsdc535qp7wmqm3y";

        binsFilter = [
          "lair-keystore"
        ];

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

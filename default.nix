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
      rev = "holochain-0.0.153"; # Aug 10, 2022 - 005c15c06404f41baa77e4533a32cef93a69ed3b
      sha256 = "19d2g8p1n9iskc10yamivn326mrlcs0q1qyvfvlm577g6cb5zdyi";
      cargoLock = {
        outputHashes = {
        };
      };

      binsFilter = [
        "holochain"
        "hc"
        "kitsune-p2p-tx2-proxy"
      ];

      rustVersion = "1.59.0";

      lair = {
        url = "https://github.com/holochain/lair";
        rev = "lair_keystore-v0.2.0"; # Jun 20, 2022 - 20b18781d217f172187f16a0ef86b78eb1fcd3bd
        sha256 = "1j3a8sgcg0dki65cqda2dn5wn85m8ljlvnzyglaayhvljk4xkfcz";

        binsFilter = [
          "lair-keystore"
        ];

        rustVersion = "1.59.0";

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

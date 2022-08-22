let
  holonixPath = builtins.fetchTarball { # main as of Aug 21, 2022
    url = "https://github.com/holochain/holonix/archive/9fedfe36a0fbe5046227ba8cf506676a34550832.tar.gz";
    sha256 = "0g7mhhp2a7nw3nq8iybm81hia7lx4avv8nqkq4g3lm5m3lzxc88s";
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
      rev = "holochain-0.0.155"; # Aug 20, 2022 - ab8c5552111da73971554ea3d80e473c97b5c650
      sha256 = "1nh1z6vnvi88fp481b4782mxqxwvg1cxz19n8y7dpn35s8jgwraa";
      cargoLock = {
        outputHashes = {
        };
      };

      binsFilter = [
        "holochain"
        "hc"
      ];

      rustVersion = "1.63.0";

      lair = {
        url = "https://github.com/holochain/lair";
        rev = "lair_keystore-v0.2.0"; # Jun 20, 2022 - 20b18781d217f172187f16a0ef86b78eb1fcd3bd
        sha256 = "1j3a8sgcg0dki65cqda2dn5wn85m8ljlvnzyglaayhvljk4xkfcz";

        binsFilter = [
          "lair-keystore"
        ];

        rustVersion = "1.63.0";

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

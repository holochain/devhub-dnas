let
  holonixPath = builtins.fetchTarball { # Oct 6, 2021
    url = "https://github.com/holochain/holonix/archive/de9d6d1e820f4e3beeb20c24005c17f565a24453.tar.gz";
    sha256 = "1z7w0hbncm375ns04021ka6li9qpchx0qn13v5xycd8p3sq0x14n";
  };
  holonix = import (holonixPath) {
    include = {
      holochainBinaries = true;
      node = false;
      happs = false;
    };

    holochainVersionId = "custom";
    holochainVersion = { # v0.0.124
      url = "https://github.com/holochain/holochain";

      rev = "4a449ee985053f5b8ee0f0688b949bd26bfcb3fc"; # Jan 26, 2022
      sha256 = "0mbamx8njs0gs28s5rbfni3p4jdf6jrhaqj1b2aqbzl7ah2nb02c";
      cargoLock = {
        outputHashes = {
          "cargo-test-macro-0.1.0" = "1yy1y1d523xdzwg1gc77pigbcwsbawmy4b7vw8v21m7q957sk0c4";
        };
      };

      binsFilter = [
        "holochain"
        "hc"
        # "kitsune-p2p-proxy"
      ];

      lair = { # v0.1.0
        url = "https://github.com/holochain/lair";

        rev = "0343621e0bfa2a941ecf53363003d1f28b7ef0e6";
        sha256 = "0jvk4dd42axwp5pawxayg2jnjx05ic0f6k8f793z8dwwwbvmqsqi";

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

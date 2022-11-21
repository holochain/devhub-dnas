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

    holochainVersionId = "custom";
    holochainVersion = {
      url = "https://github.com/holochain/holochain";
      rev = "holochain-0.0.172"; # Nov 9, 2022 - c39dac04fc87bc1325a8bb6fce275caedaa07eb3
      sha256 = "13hksk7agh1w2rzmwx5rlh3m7arrw09hqp53i2wkf2acz3qdj4hm";
      cargoLock = {
        outputHashes = {
        };
      };

      binsFilter = [
        "holochain"
        "hc"
        # "kitsune-p2p-tx2-proxy"
      ];

      rustVersion = "1.63.0";

      lair = {
        url = "https://github.com/holochain/lair";
        rev = "lair_keystore-v0.2.1"; # Sep 20, 2022 - 840999730ff2a5bacea8a31ed8fbacc954291b5c
        sha256 = "0pzvsm4jq1w8k5n9b949fyi7dd4d54sz86graxq948apiwc60bmp";

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

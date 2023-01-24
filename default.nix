let
  holonixPath = builtins.fetchTarball { # main as of Jan 19, 2022
    url = "https://github.com/holochain/holonix/archive/ca3211a9cf3c7a8bdd9229d5be8af2f5888469b6.tar.gz";
    sha256 = "1cqq274z7gg4s0bhrf188nbsn46zv6i46xych20afaj636d44llf";
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
      rev = "holochain-0.1.0-beta-rc.4"; # Jan 20, 2023 - 2730fc80a4c87144fc0a755862628af5448b96fd
      sha256 = "Rr66+kZf5GTnXlhyBfM3U0uXJU2k3l4xSMcH23x0Wz4=";
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
        rev = "lair_keystore-v0.2.3"; # Dec 13, 2022 - cbfbefefe43073904a914c8181a450209a74167b
        sha256 = "011c0cng4h1vjb9wkjplhnpl6vnc41c8h8l4k6ldgc5k4ppap8vj";

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

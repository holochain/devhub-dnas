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
      rev = "holochain-0.1.0"; # Jan 26, 2023 - 41150668b18a57f4dc801a0b3439c1c76e149064
      sha256 = "sha256-jSEWpZut7OgdBVPOpLoLPGAZT0+pnSngvc5oEsNy67M=";
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

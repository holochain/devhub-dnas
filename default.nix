let
  holonixPath = builtins.fetchTarball { # Oct 6, 2021
    url = "https://github.com/holochain/holonix/archive/48a75e79b1713334ab0086767a214e5b1619d38d.tar.gz";
    sha256 = "0r8ph5l00g70lr7lfcipnsv5vcagq5b51in232gdbglw9ngk8048";
  };
  holonix = import (holonixPath) {
    include = {
      holochainBinaries = true;
      node = false;
      happs = false;
    };

    holochainVersionId = "custom";
    holochainVersion = { # v0.0.109 (not official tag)
      rev = "e5a480ce735beaa8ae7434abdb1b6dc03d487ffa"; # Oct 7, 2021
      sha256 = "12xg1qmxxcp252z9dsk3hf2c3zl3m74891iiwg6ks08pql39cjsb";
      cargoSha256 = "08a72d7nqpakml657z9vla739cbg8y046av4pwisdgj1ykyzyi60";

      lairKeystoreHashes = { # v0.0.4
        sha256 = "12n1h94b1r410lbdg4waj5jsx3rafscnw5qnhz3ky98lkdc1mnl3";
        cargoSha256 = "0axr1b2hc0hhik0vrs6sm412cfndk358grfnax9wv4vdpm8bq33m";
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

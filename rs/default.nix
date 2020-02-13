{ callPackage, fetchgit, makeRustPlatform, rustChannelOf, nix, boost, postgresql95, hydraSrc }:

let
  naersk = callPackage (
    fetchgit {
      url = "https://github.com/nmattia/naersk";
      rev = "9a71dbed3c33b2faa57b2fb05ae9cfd563bd6c2c";
      sha256 = "1bgkmfdkf3vsxk16vfv2j14xb1sk7idswqd8iai5lwqq7lxq3cna";
    }
  ) {
    inherit (channel) cargo;
    rustc = channel.rust;
  };
  channel = rustChannelOf {
    date = "2020-02-06";
    channel = "nightly";
  };

in naersk.buildPackage {
  name = "hydra-scripts";
  version = "0.1.0";
  src = hydraSrc;
  doCheck = false;

  buildInputs = [ boost nix.dev postgresql95 ];
}

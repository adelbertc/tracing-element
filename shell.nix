let
  rust-version = "1.47.0";

  nixpkgs = fetchGit {
    url = "https://github.com/NixOS/nixpkgs.git";
    rev = "f26d6639f278bb03bf8dcde3b8fccc855f313154";
  };

  mozilla-overlay =
    import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);

  pkgs = import nixpkgs {
    overlays = [ mozilla-overlay ];
  };

  rust-channel = pkgs.rustChannelOf {
    channel = rust-version;
  };

  rust = rust-channel.rust.override {
    extensions = [ "rust-src" ];
  };

  cargo = rust-channel.cargo;
in
  pkgs.mkShell {
    name = "rust-dev";
    buildInputs = [ rust cargo pkgs.rust-analyzer ];
  }

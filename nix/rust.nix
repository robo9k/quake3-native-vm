{ sources ? import ./sources.nix }:

let
  pkgs =
    import sources.nixpkgs { overlays = [ (import sources.nixpkgs-mozilla) ]; };
  chan = (pkgs.rustChannelOf { rustToolchain = ../rust-toolchain.toml; }).rust;
in chan

let
    moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
    nixpkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
    aoc2024Rust = nixpkgs.rustChannelOf {
        date = "2024-11-28";
        channel = "stable";
    };
in
with nixpkgs;
stdenv.mkDerivation {
    name = "moz_overlay_shell";
    buildInputs = [
        aoc2024Rust.rust
        aoc2024Rust.rust-std
        aoc2024Rust.rust-analysis
        lldb_18
    ];
    nativeBuildInputs = [
        nixpkgs.openssl.dev
    ];
}

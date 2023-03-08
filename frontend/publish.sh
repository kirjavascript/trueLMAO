#!/bin/sh
# cargo install cross
# cargo install cargo-appimage

# windows
CROSS_CONTAINER_ENGINE=podman
cross build --release --target x86_64-pc-windows-gnu
cross build --release --target i686-pc-windows-gnu
mv ../target/x86_64-pc-windows-gnu/release/frontend_bin.exe
cross clean

# appimage
cargo appimage

# web
sh build.sh

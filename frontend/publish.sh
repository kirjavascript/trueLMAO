#!/bin/sh
# cargo install cross
# cargo install cargo-appimage

# windows
CROSS_CONTAINER_ENGINE=podman
cross build --release --target x86_64-pc-windows-gnu

# appimage
cargo appimage

# web
sh build.sh

# cross clean

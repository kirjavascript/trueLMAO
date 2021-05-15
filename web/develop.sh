#!/bin/sh
# TODO: release mode

# https://www.npmjs.com/package/local-web-server
trap 'kill %1; kill %2' SIGINT
cargo watch -s 'wasm-pack build --target web && cp pkg/web_bg.wasm static/emu.wasm' -w src -w ../emu &
ws --spa index.html --directory static &

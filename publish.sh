#!/usr/bin/env bash

rm -rf ./pkg
wasm-pack build --target=nodejs
wasm-pack pack
wasm-pack publish

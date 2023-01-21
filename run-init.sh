#!/bin/sh
cargo build
rm -rf .dotstrap-init
mkdir -p .dotstrap-init
cd .dotstrap-init
~/Projects/dotstrap/target/debug/dotstrap init
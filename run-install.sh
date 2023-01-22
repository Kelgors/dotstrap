#!/bin/sh
cargo build
cd .dotstrap
~/Projects/dotstrap/target/debug/dotstrap install --dry
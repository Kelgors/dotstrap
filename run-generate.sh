#!/bin/sh
cargo build
cd .dotstrap
~/Projects/dotstrap/target/debug/dotstrap generate Kelgors-Desktop

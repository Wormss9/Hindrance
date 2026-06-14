#!/usr/bin/env bash
set -e

mkdir -p target/dist/linux
mkdir -p target/dist/windows

cargo build --release
cargo build --release --target x86_64-pc-windows-gnu

cp target/release/Hindrance \
   target/dist/linux/

cp target/x86_64-pc-windows-gnu/release/Hindrance.exe \
   target/dist/windows/

cp "$(find . -name libsteam_api.so | head -n1)" \
   target/dist/linux/

cp "$(find . -name steam_api64.dll | head -n1)" \
   target/dist/windows/
#!/usr/bin/env bash
set -e

rm -rf target/dist

mkdir -p target/dist/linux
mkdir -p target/dist/windows

cargo build --release --no-default-features
cargo build --release --target x86_64-pc-windows-gnu --no-default-features

cp target/release/Hindrance target/dist/linux/
cp target/x86_64-pc-windows-gnu/release/Hindrance.exe target/dist/windows/

cp -r assets target/dist/linux/assets
cp -r assets target/dist/windows/assets

cp dist/libsteam_api.so target/dist/linux/
cp dist/steam_api64.dll target/dist/windows/
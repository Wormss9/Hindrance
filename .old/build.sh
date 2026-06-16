#!/usr/bin/env bash
set -e

rm -r target/dist

mkdir -p target/dist/linux/assets
mkdir -p target/dist/windows/assets

cargo build --release --no-default-features
cargo build --release --target x86_64-pc-windows-gnu --no-default-features

cp target/release/Hindrance target/dist/linux/
cp target/x86_64-pc-windows-gnu/release/Hindrance.exe target/dist/windows/

cp assets/icon.png target/dist/linux/assets/
cp assets/icon.png target/dist/windows/assets/

cp assets/Jost-SemiBold.ttf target/dist/linux/assets/
cp assets/Jost-SemiBold.ttf target/dist/windows/assets/

cp assets/libsteam_api.so target/dist/linux/
cp assets/steam_api64.dll target/dist/windows/
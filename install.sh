#!/bin/bash

echo
echo "crawl"
echo "by ShadowNetter"
echo
echo "cloning into repo..."
git clone https://github.com/ShadowNetter-Official/dictate
cd dictate
echo "done"
echo "installing..."
cargo build --release
cp target/release/crawl ~/.cargo/bin/
echo "done"
echo
echo "to uninstall do: "
echo "rm ~/.cargo/bin/crawl"
echo

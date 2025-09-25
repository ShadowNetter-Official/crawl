#!/bin/bash

echo
echo "crawl"
echo "by ShadowNetter"
echo "installing..."
cargo build --release
cp target/release/crawl ~/.cargo/bin/
echo "done"
echo
echo "to uninstall do: "
echo "rm ~/.cargo/bin/crawl"
echo

#!/usr/bin/env sh

cd nfp_client/
cargo build
sudo cp ./target/debug/nfp_client /usr/bin
cd ..
cd nfp_server/
cargo build
sudo cp ./target/debug/nfp_server /usr/bin

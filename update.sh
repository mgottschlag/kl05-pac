#!/usr/bin/env bash

cargo install --version 0.19.0 svd2rust
cargo install --version 0.7.0  form
rustup component add rustfmt
pip3 install --upgrade --user "svdtools>=0.1.20"

rm -rf src
mkdir src

svd2rust -i MKL05Z4.svd
form -i lib.rs -o src
rm lib.rs

cargo fmt

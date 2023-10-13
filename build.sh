#!/bin/bash

# EXAMPLE USAGE:
# build.sh andromeda-contract some-category
# Builds "andromeda-contract" contract and "some-category" category

# LOG all the contracts compiled with there compressed file size
local FILE_LOG=""

build_contract () {
    echo "Building contract...";
    cargo wasm
    # Get the version of the contract processed
    for wasm in target/wasm32-unknown-unknown/release/*.wasm; do
        local CONTRACT=`basename $wasm`;
        local CONTRACT=${CONTRACT//_/-}
        local CONTRACT=${CONTRACT//.wasm/}
        # Get the version of the contract processed
        local BUILD_VERSION=$(cargo pkgid $CONTRACT | cut -d# -f2 | cut -d: -f2)
        local OUT_FILE="./artifacts/$CONTRACT@$BUILD_VERSION.wasm"
        wasm-opt -Os $wasm -o $OUT_FILE
    done;
}


export RUSTFLAGS="-C link-arg=-s"

#Clear current builds
rm -rf ./target
rm -rf ./artifacts
mkdir artifacts

build_contract
echo -e "$FILE_LOG"
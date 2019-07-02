#!/bin/bash

set -e

OUTPUT=src/elastic/src/genned/mod.rs

if [ ! -f $OUTPUT ]; then
    echo "this script must be run from the repository root"
    exit
fi

cargo run -p generate_requests > $OUTPUT
cargo fmt

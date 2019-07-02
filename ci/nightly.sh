#!/bin/bash

set -o errexit -o nounset -o xtrace

if [ "$KIND" == "build" ]; then
    cargo test
elif [ "$KIND" == "integration" ]; then
    ELASTIC_LOG=debug cargo run -p integration -- default sniffed_node
fi

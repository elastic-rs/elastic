#!/bin/bash

set -o errexit -o nounset -o xtrace

if [ "$KIND" == "build" ]; then
    cargo test --verbose --all

    BRANCH=$(if [ "$TRAVIS_PULL_REQUEST" == "false" ]; then echo $TRAVIS_BRANCH; else echo $TRAVIS_PULL_REQUEST_BRANCH; fi)

    if [ "$BRANCH" == "vNext" ]; then
        echo "uploading crate docs"

        cargo doc --no-deps --all

        REV=$(git rev-parse --short HEAD)
        cd target/doc
        rm -rf .git || true
        git init
        git remote add upstream "https://$GH_TOKEN@github.com/elastic-rs/elastic.git"
        git config user.name "elastic-rs"
        git config user.email "travis@elastic.rs"
        git add -A .
        git commit -qm "Build docs at ${TRAVIS_REPO_SLUG}@${REV}"

        echo "Pushing gh-pages to GitHub"
        git push -q upstream HEAD:refs/heads/gh-pages --force
    fi
elif [ "$KIND" == "bench" ]; then
    cd benches
    cargo build --all
elif [ "$KIND" == "integration" ]; then 
    cd tests/run
    cargo build

    export RUST_LOG=info
    ../../target/debug/run default sniffed_node
fi

set -ex

cargo test --verbose --all
cargo bench --verbose --all

cd benches
cargo build --all

cd ../tests/run
cargo run

if [ "$TRAVIS_BRANCH" = "master" ]; then
    cd ../../
    cargo doc --all

    REV=$(git rev-parse --short HEAD)
    cd target/doc
    git init
    git remote add upstream "https://$GH_TOKEN@github.com/elastic-rs/elastic.git"
    git config user.name "elastic-rs"
    git config user.email "travis@elastic.rs"
    git add -A .
    git commit -qm "Build docs at ${TRAVIS_REPO_SLUG}@${REV}"

    echo "Pushing gh-pages to GitHub"
    git push -q upstream HEAD:refs/heads/gh-pages --force
fi

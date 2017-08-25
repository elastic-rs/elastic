cargo test --verbose --all

cd benches
cargo build --all

cd ../tests/run
cargo run

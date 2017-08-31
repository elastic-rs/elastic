cargo test --verbose --all
cargo bench --verbose --all

cd benches
cargo build --all

cd ../tests/run
cargo run

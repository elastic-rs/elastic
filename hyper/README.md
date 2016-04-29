# elasticsearch-rs hyper client
Yet another work in progress Elasticsearch client for Rust.

A lightweight implementation of the Elasticsearch API based on Hyper.

Each API endpoint is represented as its own function, so each possible http route gets its own function.
The functions are also designed to work well with the `elastic_types` and `json_str` crates, but deserialisation is the responsibility of the caller.

Tests and benches require an Elasticsearch instance is available at `localhost:9200`, and can be run with `cargo bench --features test-integration`.
Running without the `--features` flag will run tests that don't depend on Elasticsearch itself, if any.
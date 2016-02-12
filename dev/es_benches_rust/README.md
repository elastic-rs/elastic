# elasticsearch-rs
Yet another work in progress Elasticsearch client for Rust.

## Dev Scratchpad

This crate is just a place to mock out possible client implementations and bench them. The two main client efforts are:

- [hyper]() for synchronous io. See issue: [#2](https://github.com/KodrAus/elasticsearch-rs/issues/2)
- [rotor](https://github.com/tailhook/rotor-http) for asynchronous io. See issue: [#31](https://github.com/KodrAus/elasticsearch-rs/issues/31)

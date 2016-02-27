# elasticsearch-rs hyper client
Yet another work in progress Elasticsearch client for Rust.

This is the utility that takes the Elasticsearch API spec and generates `rust` source for `hyper`.
To generate source, build this crate and run:

```
target/debug/elastic_hyper_codegen ../../codegen/spec/api ../src/api
```
# `elastic_hyper` codegen

This is the utility that takes the Elasticsearch API spec and generates `rust` source for `hyper`.
To generate source, build this crate and run:

```
target/release/elastic_hyper_codegen ../../codegen/spec/api ../src/api
```

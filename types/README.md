# elasticsearch-rs core types
[Docs](http://kodraus.github.io/rustdoc/elastic_types/)

Yet another work in progress Elasticsearch client for Rust.

A high-level implementation of the core types in Elasticsearch documents.

Types within this crate are self-contained and handle their own serialisation/deserialisation requirements.
Each type also supplies a `struct` for its [Put Mapping API](https://www.elastic.co/guide/en/elasticsearch/reference/current/indices-put-mapping.html) properties.

The types in this crate are backed by compiler plugins in `elastic_types_codegen`.

# Links
- [Elasticsearch Doc](https://www.elastic.co/guide/en/elasticsearch/guide/current/mapping.html)
- [Github](https://github.com/KodrAus/elasticsearch-rs)

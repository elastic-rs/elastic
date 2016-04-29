# elasticsearch macros
Yet another work in progress Elasticsearch client for Rust.

A collection of compiler-plugin macros for the various other Elasticsearch crates:

- `elastic_types_macros` for custom derive attributes on mapping types
- `elastic_date_macros` for date-specific compiler plugins
- `json_str` for building json string literals (probably moving to a separate repo in the future)

# Links
- [Compiler Plugins](https://doc.rust-lang.org/book/compiler-plugins.html)
- [Github](https://github.com/KodrAus/elasticsearch-rs)

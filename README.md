# elasticsearch-rs
Elasticsearch client for Rust

# Goals

To provide a strongly-typed, full-featured and efficient Elasticsearch client for Rust over asynchronous io.

I'd like to follow a similar pattern to the Query DSL as the C# client does, where type info is always available when constructing queries. With Rust, it should be possible to produce a synax that's closer to the actual JSON spec, possibly using macros.


# Roadmap

- Codegen API endpoints
- Implement core Elasticsearch types
- io layer
- Doc APIs
- Query DSL proof-of-concept to test design
- Everything else

# [`elastic_responses`](https://docs.rs/elastic_responses/*/elastic_responses/) [![Latest Version](https://img.shields.io/crates/v/elastic_responses.svg)](https://crates.io/crates/elastic_responses)

A crate to handle parsing and handling Elasticsearch search results which provides convenient iterators to step through the results returned. It is designed to work with [`elastic-reqwest`](https://github.com/elastic-rs/elastic-reqwest/).

## Example
 
`Cargo.toml`
```
[dependencies]
elastic_reqwest = "*"
elastic_responses = "*" 
```

### Search

Query your Elasticsearch Cluster, then iterate through the results:

```rust
// Send a request (omitted, see `samples/basic`, and read the response.
let mut res = client.elastic_req(&params, SearchRequest::for_index("_all", body)).unwrap();

// Parse body to JSON
let response = parse::<SearchResponse<Value>>().from_reader(res.status().to_u16(), res).unwrap();

// Use hits() or aggs() iterators
// Hits
for hit in response.hits() {
    println!("{:?}", hit);
}

// Agregations
for agg in response.aggs() {
    println!("{:?}", agg);
}
```

### Bulk

Bulk response operations are split by whether they succeeded or failed:

```rust
// Send a request (omitted, see `samples/bulk`, and read the response.
let mut res = client.elastic_req(&params, BulkRequest::new(body)).unwrap();

// Parse body to JSON
let response = parse::<BulkResponse>().from_reader(res.status().to_u16(), res).unwrap();

for op in response.into_iter() {
    match op {
        Ok(ok) => {
            // Do something with successful operations
            println!("ok: {:?}", op);
        },
        Err(err) => {
            // Do something with failed operations
            println!("{:?}", op);
        }
    }
}
```
 
## License

Licensed under either of these:
 
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

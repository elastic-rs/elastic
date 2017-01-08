# Elasticsearch Response Iterators

A crate to handle parsing and handling Elasticsearch search results which provides
convenient iterators to step through the results returned. It is designed to work
with [`elastic-reqwest`](https://github.com/elastic-rs/elastic-hyper/).

## Usage
 
`Cargo.toml`
```
[dependencies]
elastic_reqwest = "*"
elastic_requests = "*"
elastic_responses = "*" 
```

Query your Elasticsearch Cluster, then iterate through the results:

 ```rust
 // Send a request (omitted, see `samples/basic`, and read the response.
 let mut res = client.elastic_req(&params, SearchRequest::for_index("_all", body)).unwrap();

 // Parse body to JSON as an elastic_responses::Response object
 let body_as_json: EsResponse = res.json().unwrap();

 // Use hits() or aggs() iterators
 // Hits
 for hit in body_as_json.hits() {
     println!("{:?}", hit);
 }

 // Agregations
 for agg in body_as_json.aggs() {
     println!("{:?}", agg);
 }
 ```
 
## License

Licensed under either of these:
 
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
 

# [`elastic`](https://docs.rs/elastic/*/elastic/) [![Latest Version](https://img.shields.io/crates/v/elastic.svg)](https://crates.io/crates/elastic) [![Gitter](https://img.shields.io/gitter/room/nwjs/nw.js.svg)](https://gitter.im/elastic-rs/Lobby)

`elastic` is an efficient, modular API client for [Elasticsearch](https://github.com/elastic/elasticsearch) written in [Rust](https://www.rust-lang.org).
The API is targeting the Elastic Stack `6.x`.

`elastic` provides strongly-typed documents and weakly-typed queries.

## Stability

This crate is still quite unstable and is likely to continue to churn breaking releases over the near future with not-so-detailed changelogs.

If you run into any problems upgrading in your own open source projects feel free to [open up an issue](https://github.com/elastic-rs/elastic/issues) and we'll give you a hand. The goal is definitely to offer a stable API eventually.

## Example

Add `elastic` to your `Cargo.toml`:

```toml
[dependencies]
elastic = "*"
elastic_derive = "*"
serde_json = "*"
```

Create a `SyncClient` and start making requests:

```rust
#[macro_use]
extern crate elastic_derive;
#[macro_use]
extern crate serde_json;
extern crate elastic;

use serde_json::Value;
use elastic::prelude::*;

// A reqwest HTTP client and default parameters.
// The builder includes the base node url (http://localhost:9200).
let client = SyncClientBuilder::new().build()?;

let query = "some query string";

// A search request with a freeform body.
let res = client.search::<Value>()
                .index("_all")
                .body(json!({
                    "query": {
                        "query_string": {
                            "query": query
                        }
                    }
                }))
                .send()?;

// Iterate through the hits in the response.
for hit in res.hits() {
    println!("{:?}", hit);
}
```

`elastic` also offers an `AsyncClient` for use with the `tokio` asynchronous io stack.
See the [examples](https://github.com/elastic-rs/elastic/tree/master/examples) folder for complete samples.

### Building documents

Document mapping is derived at compile-time from your _Plain Old Rust Structures_. Just add a `#[derive(ElasticType)]` attribute:

```rust
#[derive(ElasticType, Serialize, Deserialize)]
struct MyDocument {
	pub id: String,
	pub title: String,
	pub timestamp: Date<DefaultDateMapping<EpochMillis>>,
	pub content: Text<DefaultTextMapping>,
}
```

And you can start using `MyDocument` in `Client` request methods.

See the [docs](https://docs.rs/elastic/*/elastic/types/index.html) for more details.

## Alternatives

If you'd like to use a strongly-typed Query DSL builder see [`rs-es`](https://github.com/benashford/rs-es). This client does the hard work of providing an idiomatic Rust API for interacting with Elasticsearch. It has the advantage of letting you know your queries will parse at compile-time instead of runtime.

## License

Licensed under either of these:
 
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

# Developing `elastic`

This document outlines some thoughts on the development of `elatic`.

## Getting started

Some common changes to `elastic`.
This list should grow over time, and be updated as things change.

### Adding a new client method

1. The raw request type should already exist in `src/elastic/genned/mod.rs` (the types are pre-generated).
1. Add the response type to `src/elastic/client/responses`. The name should align with the generated name for the request, like `IndicesExistsRequest` and `IndicesExistsResponse`. The name of the module should align with the high-level request, like `src/client/requests/index_exists` and `src/client/responses/index_exists`.
1. Add a client method to `elastic` in a new module under the `src/elastic/src/client/requests` folder. The naming convention is `document_*` for document methods like `document_get`, `index_*` for index methods like `index_exists` and no prefix for other methods like `search`. You can copy one of the existing methods as a template. They all follow a similar pattern of defining a synchronous and asynchronous implementation.
1. Add the method to the table in the client docs under `src/elastic/src/client/mod.rs`.
1. Add an integration test to `tests/integration`. You can copy one of the existing integration tests as a template. You can also run just your integration test by calling `cargo run -p integration -- --filter integration::tests::{group}::{case}`.

Some other considerations:

- We should avoid adding a `DocumentType` bound to generics unless the method has a `document_` prefix. For example the `search` method takes a `T: DeserializeOwned`, where `document_get` takes a `T: DocumentType + DeserializeOwned`.

## Development

`elastic` targets the _latest_ `stable` channel, so it doesn't use any unstable features, but we'd like to track where improvements can be made by unstable features once they stabilise.

`elastic` sits on a stack with hard dependencies on the following libraries:

- `reqwest`/`hyper` for HTTP transport
- `tokio` for async IO
- `serde` for serialisation

There hasn't been much effort put into abstracting these dependencies at this stage, and `elastic` can't stabilise until these libraries and a few others do.

### Methodology

The following is a simple set of guidelines that the codebase should follow. It's mostly a reminder to ourselves and not a hard set of rules.

#### Usability

- Make it difficult for callers to mess up invariants
- Avoid duplicating effort between crates
- Keep caller code obvious and auditable
- Avoid unnecessary dependencies so callers don't end up depending on stuff they don't want to
- Avoid panicking

#### Performance

- Minimise heap allocations and copies where possible while staying ergonomic
- Cover features with micro-benchmarks to support performance investigation and catch unexpected regressions

#### Testing

- Try cover the whole public API surface so breaking changes result in broken tests/examples
- Features should fail to compile if used incorrectly rather than having surprising behaviour at runtime

#### Documentation

- Types should have detailed docs with general examples
- Type methods should have examples and document any panics/error cases
- Modules should have general guidance for the types they contain
- Make it easy to navigate between related types. `elastic` uses a lot of generic code that can be hard to follow, so we need to work hard to help the user follow what's happening

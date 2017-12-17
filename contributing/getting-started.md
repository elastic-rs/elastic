# Developing `elastic`

This document outlines some thoughts on the development of `elatic`.

## Getting started

Some common changes to `elastic`.
This list should grow over time, and be updated as things change.

### Adding a new client method

1. The raw request type should already exist in `elastic_requests` (the types are pre-generated).
1. Add the response type to `elastic_responses`. The name should align with the generated name for the request, like `IndicesExistsRequest` and `IndicesExistsResponse`.
1. Add a client method to `elastic` in a new module under the `src/elastic/client/requests` folder. The naming convention is `document_*` for document methods like `document_get`, `index_*` for index methods like `index_exists` and no prefix for other methods like `search`.
1. Add the method to the table in the client docs under `src/elastic/client/mod.rs`.
1. Add an integration test to `tests/run`.

Some other considerations:

- We should avoid adding a `DocumentType` bound to generics unless the method has a `document_` prefix. For example the `search` method takes a `T: DeserializeOwned`, where `document_get` takes a `T: DocumentType + DeserializeOwned`.

## Development

`elastic` targets the `stable` channel, so it doesn't use any unstable features, but we'd like to track where improvements can be made by unstable features once they stabilise. There is another [GitHub Project](https://github.com/orgs/elastic-rs/projects/8) to record these possible enhancements.

The `elastic` crate brings a few independent crates together into a cohesive API. It aims to provide the glue between them and offer some good defaults. If you have a more specialised use-case, you can pick and choose the crates that will best support it. See the [crates](#crates) section for a full list.

`elastic` sits on a stack with hard dependencies on the following libraries:

- `reqwest`/`hyper` for HTTP transport
- `serde` for serialisation

There hasn't been much effort put into abstracting these dependencies at this stage, and `elastic` can't stabilise until these libraries and a few others do.

### Branches

The `master` branch should always be just about current with what's released on `crates.io`. Any non-breaking changes will be merged straight into `master` and released.

The `vNext` branch is where breaking changes for upcoming releases are collected. Once we're ready for a new breaking release, we'll merge `vNext` into `master` and push out a new release.

If you'd like to work on a new feature, base off `master`, unless you depend on code that's already in `vNext`. If the feature can be implemented in a non-breaking way then we'll merge it in to `master` for you. If it can't then we'll merge it in to `vNext`.

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

## Navigating the repository

`elastic` bundles up a couple of crates into a single client. This might make it difficult to find your way around the codebase when following items or finding out where a change should live.

### `elastic`

This is the main crate that bundles up `elastic_requests`, `elastic_types`, `elastic_requests` and `elastic_responses`.

### `elastic_reqwest`

A synchronous [`reqwest`](https://github.com/seanmonstar/reqwest) implementation of the Elasticsearch REST API.

### `elastic_requests`

Zero-copy request types for the REST API endpoints. These are automatically generated from the official spec.

### `elastic_responses`

Idiomatic support for inspecting Elasticsearch responses and iterating over hits.

### `elastic_types`

A library for building Elasticsearch types in Rust. Define your Elasticsearch types as PORS (Plain Old Rust Structures) and generate an equivalent Elasticsearch mapping from them, where correctness is enforced by Rust's type system.

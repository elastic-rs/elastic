# Async Sample

This is a sample that demonstrates how you can use individual crates that make up `elastic` to build your own specific client. It's also a bit of a playground for ideas that may find their way into the `elastic` client.

It uses the [`tokio`](https://tokio.rs) branch of [`hyper`](https://hyper.rs) for asynchronously streaming a bulk request from a file. The file is memory mapped so it doesn't need a buffer allocated to hold all the bits at once. There are a few other optimisations this sample uses:

- The response body isn't buffered into a single contiguous slice for deserialisation. This trades memory efficiency for speed and complexity
- The allocated field type for the `BulkResponse` is an interned string. That means duplicated values like the `index` and `type` on bulk operations are more efficient

`elastic` will offer a first-class async API once `reqwest` does.

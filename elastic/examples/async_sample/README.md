# Async Sample

This is a sample that demonstrates how you can use individual crates that make up `elastic` to build your own specific client.

It uses the [`tokio`](https://tokio.rs) branch of [`hyper`](https://hyper.rs) for asynchronously streaming a bulk request from a file. The file is memory mapped so it doesn't need a buffer allocated to hold all the bits at once. This approach could be useful for a [filebeat](https://www.elastic.co/products/beats/filebeat) like service that maps the new portion of a data file and upserts it.

`elastic` will offer a first-class async API once `reqwest` does.
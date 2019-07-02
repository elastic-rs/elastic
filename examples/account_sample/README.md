# Account Sample

This is a more complete sample for `elastic`, using the fake [bank account sample data](https://www.elastic.co/guide/en/elasticsearch/reference/current/getting-started-explore-data.html).

The code contains inline tests to show you what the result of some operations like document mapping will be.

Some of the key pieces include:

- The [`Account` document type](src/model/account.rs)
- The [command to create the index](src/ops/commands/ensure_bank_index_exists.rs)
- The [query to search `Account`s](src/ops/queries/simple_search.rs)

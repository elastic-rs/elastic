# Client Micro-benchmark Tests

This repo is an **unscientific** attempt to benchmark the performance of a few Elasticsearch clients. The idea is just to keep a ballpark figure of `elastic` compared to a few similar clients to make sure it doesn't deviate too far.

## Process

The test data is built with the following Sense script:

```
PUT bench_index

PUT bench_index/bench_doc/_mapping 
{
  "properties": {
    "id": {
      "type": "integer"
    },
    "title": {
      "type": "string"
    },
    "timestamp": {
      "type": "date",
      "format": "epoch_millis"
    }
  }
}

PUT bench_index/bench_doc/1
{
  "id": 1,
  "title": "Document",
  "timestamp": 1460002100704
}

PUT bench_index/bench_doc/2
{
  "id": 2,
  "title": "Document",
  "timestamp": 1460002100704
}

PUT bench_index/bench_doc/3
{
  "id": 3,
  "title": "Document",
  "timestamp": 1460002100704
}

PUT bench_index/bench_doc/4
{
  "id": 4,
  "title": "Document",
  "timestamp": 1460002100704
}

PUT bench_index/bench_doc/5
{
  "id": 5,
  "title": "Document",
  "timestamp": 1460002100704
}

PUT bench_index/bench_doc/6
{
  "id": 6,
  "title": "Document",
  "timestamp": 1460002100704
}

PUT bench_index/bench_doc/7
{
  "id": 7,
  "title": "Document",
  "timestamp": 1460002100704
}

PUT bench_index/bench_doc/8
{
  "id": 8,
  "title": "Document",
  "timestamp": 1460002100704
}

PUT bench_index/bench_doc/9
{
  "id": 9,
  "title": "Document",
  "timestamp": 1460002100704
}

PUT bench_index/bench_doc/10
{
  "id": 10,
  "title": "Document",
  "timestamp": 1460002100704
}
```

The test request is as follows:

```
POST bench_index/bench_doc/_search
{
  "query": {
    "query_string": {
      "query": "*"
    }
  },
  "size": 10
}
```

A baseline that gives the approximate time taken to get a response from Elasticsearch is produced
using a raw `elastic_hyper` query.

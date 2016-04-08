# Client Micro-benchmark Tests

This repo is an unscientific attempt to benchmark the performance of a few Elasticsearch clients.
Results are really just an indication of the amount of work a particular client on a particular
platform needs to do to send a single search request to Elasticsearch and deserialise the results into some native form.

This is only benchmarking the time it takes to send a single request with `keep_alive` enabled.

It would be great to have a _real_ test dataset that can be used for a more realistic set of
benchmarks including concurrent queries and doc indexing. This will do for now though.

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

## Results

### Request/Response Time (Mean ns)

![alt text](http://kodraus.github.io/query_mean.png)

### Request/Response Time (Percentile ns)

![alt text](http://kodraus.github.io/query_percentiles.png)

### Rust (elastic_hyper + elastic_types)

```
target/release/elastic_hyper_bench 1000

Time per request:       365466 [ns] (mean)

Percentage of the requests served within a certain time (ns)
  50%      359611
  66%      363613
  75%      366488
  80%      368328
  90%      373225
  95%      379301
  98%      391994
  99%      419145
```

### Rust (elastic_hyper + custom)

*NOTE:* This implementation breaks the rules by specifying a `filter_path` on the return set. So there are actually fewer bits being returned.

The point of this one is to show that `elastic_hyper` basically sits right on top of the minimum request time possible, and any work you do to the response is up to you.

```
target/release/elastic_hyper_fltrd_bench 1000

Time per request:       338909 [ns] (mean)

Percentage of the requests served within a certain time (ns)
  50%      327194
  66%      331722
  75%      335546
  80%      338625
  90%      345013
  95%      352359
  98%      386021
  99%      463996
```

### Rust (rs-es)

```
target/release/rs-es_bench 1000

Time per request:       366679 [ns] (mean)

Percentage of the requests served within a certain time (ns)
  50%      359821
  66%      364438
  75%      367154
  80%      369198
  90%      375461
  95%      381692
  98%      393719
  99%      402724
```

### Go (elastic)

```
go run main.go --runs=1000

Time per request:       454216 [ns] (mean)

Percentage of the requests served within a certain time (ns)
  50%      436441
  66%      442593
  75%      447330
  80%      451547
  90%      465539
  95%      489054
  98%      762119
  99%      918507
```

### CSharp (Elasticsearch.NET)

_TODO: Update with keep alive_

```
dnx --configuration Release run 1000

Time per request:       2298637 [ns] (mean)

Percentage of the requests served within a certain time (ns)
  50%      1183200
  66%      1207600
  75%      1224700
  80%      1231900
  90%      1269600
  95%      1388000
  98%      3153400
  99%      5986600
```

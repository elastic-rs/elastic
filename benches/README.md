# Client Micro-benchmark Tests

This repo is an unscientific attempt to benchmark the performance of a few Elasticsearch clients.
Results are really just an indication of the amount of work a particular client on a particular
platform needs to do to send a single search request to Elasticsearch and deserialise the results into some native form.

This is only benchmarking the time it takes to send a single request, so things like connection pools don't come into the equation, even though they're super-important.

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

### Rust (elastic_hyper raw) (baseline)

```
target/release/elastic_hyper_raw 1000

Time per request:       1139549 [ns] (mean)

Percentage of the requests served within a certain time (ns)
  50%      1098626
  66%      1110958
  75%      1121555
  80%      1128979
  90%      1146547
  95%      1179442
  98%      1772267
  99%      2193342
```

### Rust (elastic_hyper + elastic_types)

```
target/release/elastic_hyper_bench 1000

Time per request:       1436260 [ns] (mean)

Percentage of the requests served within a certain time (ns)
  50%      1388881
  66%      1420875
  75%      1443839
  80%      1458862
  90%      1499976
  95%      1565245
  98%      1839258
  99%      2627653
```

### Rust (elastic_hyper + custom)

*NOTE:* This implementation breaks the rules by specifying a `filter_path` on the return set. So there are actually fewer bits being returned.

The point of this one is to show that `elastic_hyper` basically sits right on top of the minimum request time possible, and any work you do to the response is up to you.

```
target/release/elastic_hyper_fltrd_bench 1000

Time per request:       1242905 [ns] (mean)

Percentage of the requests served within a certain time (ns)
  50%      1188268
  66%      1204799
  75%      1217937
  80%      1226329
  90%      1261835
  95%      1311765
  98%      1805281
  99%      2300044
```

### Rust (rs-es)

```
target/release/rs-es_bench 1000

Time per request:       1266050 [ns] (mean)

Percentage of the requests served within a certain time (ns)
  50%      1223386
  66%      1238574
  75%      1251329
  80%      1263147
  90%      1291514
  95%      1325730
  98%      1443468
  99%      2083648
```

### CSharp (Elasticsearch.NET)

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

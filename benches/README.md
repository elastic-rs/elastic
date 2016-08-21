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

PUT bench_index/bench_doc/_mapping {
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

### Rust (elastic_hyper + elastic_types)

```
target/release/elastic_hyper_bench 1000

Time per request:       443930ns (mean)

Percentage of the requests served within a certain time (ns)
  50%      440680ns
  66%      449437ns
  75%      453430ns
  80%      456648ns
  90%      468064ns
  95%      480051ns
  98%      491004ns
  99%      498912ns
```

### Rust (elastic_hyper + custom)

*NOTE:* This implementation breaks the rules by specifying a `filter_path` on the return set. So there are actually fewer bits being returned.

The point of this one is to show that `elastic_hyper` basically sits right on top of the minimum request time possible, and any work you do to the response is up to you.

```
target/release/elastic_hyper_fltrd_bench 1000

Time per request:       418474ns (mean)

Percentage of the requests served within a certain time (ns)
  50%      408622ns
  66%      417205ns
  75%      424182ns
  80%      428961ns
  90%      439819ns
  95%      451993ns
  98%      473822ns
  99%      496964ns
```

### Rust (rs-es)

```
target/release/rs-es_bench 1000

Time per request:       441224ns (mean)

Percentage of the requests served within a certain time (ns)
  50%      430000ns
  66%      443157ns
  75%      450988ns
  80%      458268ns
  90%      481121ns
  95%      498099ns
  98%      523963ns
  99%      533303ns
```

### Go (elastic)

```
go run main.go --runs=1000

Time per request:       591804ns (mean)

Percentage of the requests served within a certain time (ns)
  50%      577550ns
  66%      592822ns
  75%      604616ns
  80%      612085ns
  90%      634805ns
  95%      666371ns
  98%      732796ns
  99%      1106103ns
```

### CSharp (Elasticsearch.NET)

```
dnx --configuration Release run 1000

Time per request:       1503586ns (mean)

Percentage of the requests served within a certain time (ns)
  50%      1209800ns
  66%      1246300ns
  75%      1271100ns
  80%      1292800ns
  90%      1341900ns
  95%      1400400ns
  98%      2310900ns
  99%      5948600ns
```

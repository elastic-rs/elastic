# Client Performance Tests

This repo is an unscientific attempt to benchmark the performance of a few Elasticsearch clients.
Results are really just an indication of the amount of work a particular client on a particular
platform needs to do to send a single search request to Elasticsearch and deserialise the results
into some native form.

It would be great to have a _real_ test dataset that can be used for a more realistic set of
benchmarks. This will do for now though.

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
using `ab` as follows:

```
ab -n 200 -c 1 -T 'application/json' -p postdata.txt http://localhost:9200/bench_index/bench_doc/_search
This is ApacheBench, Version 2.3 <$Revision: 1706008 $>
Copyright 1996 Adam Twiss, Zeus Technology Ltd, http://www.zeustech.net/
Licensed to The Apache Software Foundation, http://www.apache.org/

Benchmarking localhost (be patient)
Completed 100 requests
Completed 200 requests
Finished 200 requests


Server Software:        
Server Hostname:        localhost
Server Port:            9200

Document Path:          /bench_index/bench_doc/_search
Document Length:        1583 bytes

Concurrency Level:      1
Time taken for tests:   0.260 seconds
Complete requests:      200
Failed requests:        0
Total transferred:      334200 bytes
Total body sent:        43800
HTML transferred:       316600 bytes
Requests per second:    770.26 [#/sec] (mean)
Time per request:       1.298 [ms] (mean)
Time per request:       1.298 [ms] (mean, across all concurrent requests)
Transfer rate:          1256.94 [Kbytes/sec] received
                        164.73 kb/s sent
                        1421.67 kb/s total

Connection Times (ms)
              min  mean[+/-sd] median   max
Connect:        0    0   0.1      0       1
Processing:     1    1   1.0      1      10
Waiting:        1    1   1.0      1      10
Total:          1    1   1.0      1      10

Percentage of the requests served within a certain time (ms)
  50%      1
  66%      1
  75%      1
  80%      1
  90%      2
  95%      2
  98%      4
  99%      9
 100%     10 (longest request)
```

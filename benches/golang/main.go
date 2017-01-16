package main

import (
  "fmt"
  "flag"
  "sort"
  "golang.org/x/net/context"
  "github.com/chrisport/go-stopwatch/stopwatch"
  "gopkg.in/olivere/elastic.v5"
)

type BenchDoc struct {
  Id int `json:"id"`
  Title string `json:"title"`
  Timestamp int64 `json:"timestamp"`
}

func main() {
    var runs = flag.Int("runs", 1000, "number of times to run the benchmark")

    results := make([]float64, *runs)

    for i := 0; i < *runs; i++ {
        client, err := elastic.NewClient()

        if err != nil {
          panic(err)
        }

        stopwatch := stopwatch.NewStopwatch()

        query := elastic.NewQueryStringQuery("*")
        query_result, err := client.
            Search("bench_index").
            Type("bench_doc").
            Query(query).
            Size(10).
            Do(context.TODO())

        took := stopwatch.Get().Nanoseconds()
        results[i] = float64(took)

        if query_result == nil {
            panic("Result was nil")
        }

        if err != nil {
            panic(err)
        }
    }

    t := 0.0
	for _, value := range results {
		t += value
	}

    mean := t / float64(*runs)

    fmt.Printf("took mean %fns\n", mean)

    sort.Sort(sort.Float64Slice(results))

    percentiles := [...]float64{ 0.5, 0.66, 0.75, 0.80, 0.90, 0.95, 0.98, 0.99, 1.00 }

    for _,pval := range percentiles {
        val := results[GetIndex(pval, *runs)]
        fmt.Printf("Percentile %f : %f ns\n", pval, val)
    }
}

func GetIndex(p float64, r int) int {
    return int((p * float64(r)) - 1.0)
}

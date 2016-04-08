using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;

namespace Elasticsearch.Net.Bench
{
    public class BenchDoc
    {
        public int Id { get; set; }
        public string Title { get; set; }
        public DateTime Timestamp { get; set; }
    }

    public class Program
    {
        public static void Main(string[] args)
        {
            int runs = args.Count() > 0 ? Convert.ToInt32(args.First()) : 200;

            var config = new ConnectionConfiguration(new Uri("http://localhost:9200"));
            config.DisablePing();

            var client = new ElasticLowLevelClient(config);

            var results = new List<long>();
            for (int i = 0; i < runs; i++)
            {
                var watch = Stopwatch.StartNew();
                var req = new {
                  query = new {
                    query_string = new {
                      query = "*"
                      }
                    },
                    size = 10
                  };
                var data = client.Search<BenchDoc>(
                    "bench_index", "bench_type",
                    new PostData<object>(req)
                );
                watch.Stop();

                results.Add(watch.Elapsed.Ticks * 100);
            }

            var sorted = results.OrderBy(r => r).ToArray();

            var percentiles = new List<Tuple<double, long>>
            {
                new Tuple<double, long>(0.5, sorted[GetIndex(0.5, runs)]),
                new Tuple<double, long>(0.66, sorted[GetIndex(0.66, runs)]),
                new Tuple<double, long>(0.75, sorted[GetIndex(0.75, runs)]),
                new Tuple<double, long>(0.80, sorted[GetIndex(0.80, runs)]),
                new Tuple<double, long>(0.90, sorted[GetIndex(0.90, runs)]),
                new Tuple<double, long>(0.95, sorted[GetIndex(0.95, runs)]),
                new Tuple<double, long>(0.98, sorted[GetIndex(0.98, runs)]),
                new Tuple<double, long>(0.99, sorted[GetIndex(0.99, runs)]),
                new Tuple<double, long>(1.0, sorted[GetIndex(1.0, runs)])
            };

            Console.WriteLine("took mean {0}ns", results.Sum() / (float)runs);

            foreach (var p in percentiles)
            {
                Console.WriteLine("percentile {0}%: {1}ns", p.Item1 * 100, p.Item2);
            }
        }

        static int GetIndex(double percentile, int runs)
        {
            return (int)((percentile * runs) - 1);
        }
    }
}

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
        public void Main(string[] args)
        {
            var config = new ConnectionConfiguration(new Uri("http://localhost:9200"));
            config.DisablePing();
            var client = new ElasticLowLevelClient(config);

            var results = new List<long>();

            for (int i = 0; i < 200; i++)
            {
                var watch = Stopwatch.StartNew();
                var data = client.Search<BenchDoc>(new PostData<object>("{'query':{'query_string':{'query':'*'}},'size':10}"));
                watch.Stop();

                results.Add((watch.ElapsedTicks / Stopwatch.Frequency) * 1000000000);
            }

            var sorted = results.OrderBy(r => r).ToArray();

            var percentiles = new List<Tuple<double, long>>
            {
                new Tuple<double, long>(0.5, sorted[GetIndex(0.5)]),
                new Tuple<double, long>(0.66, sorted[GetIndex(0.66)]),
                new Tuple<double, long>(0.75, sorted[GetIndex(0.75)]),
                new Tuple<double, long>(0.80, sorted[GetIndex(0.80)]),
                new Tuple<double, long>(0.90, sorted[GetIndex(0.90)]),
                new Tuple<double, long>(0.95, sorted[GetIndex(0.95)]),
                new Tuple<double, long>(0.98, sorted[GetIndex(0.98)]),
                new Tuple<double, long>(0.99, sorted[GetIndex(0.99)]),
                new Tuple<double, long>(1.0, sorted[GetIndex(1.0)])
            };

            Console.WriteLine("took mean {0}ns", sorted.Sum() / 200.0);

            foreach (var p in percentiles)
            {
                Console.WriteLine("percentile {}%: {}ns", p.Item1 * 100, p.Item2);
            }
        }

        static int GetIndex(double percentile)
        {
            return (int)((percentile * 200) - 1);
        }
    }
}

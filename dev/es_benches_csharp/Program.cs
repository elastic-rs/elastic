using System;
using System.Diagnostics;
using System.IO;

namespace ElasticsearchNetTest
{
    class Program
    {
        static void Main(string[] args)
        {
            //ElasticsearchNet
            var netwatch = Stopwatch.StartNew();
                var netclient = new Elasticsearch.Net.ElasticLowLevelClient();
                netclient.Search<Stream>("bench_index", "docs", new Elasticsearch.Net.PostData<object>(
                    @"{
                      'query': {
                        'query_string': {
                          'default_field': 'title',
                          'query': 'doc'
                        }
                      }
                    }"
                ));
            netwatch.Stop();
            Console.WriteLine("Elasticsearch.NET: {0}ns", (netwatch.Elapsed.TotalMilliseconds * 1000000).ToString("n0"));

            //NEST
            var nestwatch = Stopwatch.StartNew();
                var nestclient = new Nest.ElasticClient();
                nestclient.Search<object>(body => body
                    .Query(query => query
                        .QueryString(qrystr => qrystr
                            .Query("doc")
                            .DefaultField("title")
                        )
                    )
                );
            nestwatch.Stop();
            Console.WriteLine("NEST: {0}ns", (nestwatch.Elapsed.TotalMilliseconds * 1000000).ToString("n0"));
        }
    }
}

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
            double total = 0;
            int c = 0;
            for (int i = 0; i < 100; i++)
            {
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
                total += netwatch.Elapsed.TotalMilliseconds;
                c++;
            }
            Console.WriteLine("Elasticsearch.NET: {0}ns", ((total / c) * 1000000).ToString("n0"));

            //NEST
            total = 0;
            c = 0;
            for (int i = 0; i < 100; i++)
            {
                var nestwatch = Stopwatch.StartNew();
                    var nestclient = new Nest.ElasticClient();
                    nestclient.Search<object>(body => body
                        .Index("bench_index")
                        .Type("docs")
                        .Query(query => query
                            .QueryString(qrystr => qrystr
                                .Query("doc")
                                .DefaultField("title")
                            )
                        )
                    );
                nestwatch.Stop();
                total += nestwatch.Elapsed.TotalMilliseconds;
                c++;
            }
            Console.WriteLine("NEST: {0}ns", ((total / c) * 1000000).ToString("n0"));
        }
    }
}

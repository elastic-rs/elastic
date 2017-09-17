extern crate serde;
#[macro_use]
extern crate serde_derive;

extern crate elastic;
#[macro_use]
extern crate elastic_derive;

use elastic::prelude::*;

#[derive(Debug, Serialize, ElasticType)]
#[elastic(mapping = "BenchDocMapping")]
struct BenchDoc {
    pub id: i32,
    pub title: String,
    pub timestamp: Date<DefaultDateMapping<EpochMillis>>,
}

#[derive(Default)]
struct BenchDocMapping;
impl DocumentMapping for BenchDocMapping {
    fn name() -> &'static str {
        "bench_doc"
    }
}

fn index() -> Index<'static> {
    "bench_index".into()
}

fn main() {
    let client = SyncClientBuilder::new().build().unwrap();

    client.index_create(index()).send().unwrap();

    client
        .document_put_mapping::<BenchDoc>(index())
        .send()
        .unwrap();

    for i in 0..10 {
        let doc = BenchDoc {
            id: i,
            title: "Document".into(),
            timestamp: Date::now(),
        };

        client.document_index(index(), id(i), &doc).send().unwrap();
    }
}

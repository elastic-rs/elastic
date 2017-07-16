#[macro_use]
extern crate serde_derive;
extern crate serde;

#[macro_use]
extern crate elastic_derive;
extern crate elastic;

use elastic::prelude::*;

#[derive(Debug, Serialize, ElasticType)]
#[elastic(mapping = "BenchDocMapping")]
struct BenchDoc {
    pub id: i32,
    pub title: String,
    pub timestamp: Date<EpochMillis>,
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
    let client = ClientBuilder::new().build().unwrap();

    client.create_index(index()).send().unwrap();

    client.put_mapping::<BenchDoc>(index()).send().unwrap();

    for i in 0..10 {
        let doc = BenchDoc {
            id: i,
            title: "Document".into(),
            timestamp: Date::now(),
        };

        client.index_document(index(), id(i), &doc).send().unwrap();
    }
}

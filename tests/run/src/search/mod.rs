use run_tests::{
    test,
    Test,
};

mod empty_query;
mod no_index;
mod raw_query_string;

pub fn tests() -> Vec<Test> {
    vec![
        Box::new(|client| test(client, no_index::NoIndex)),
        Box::new(|client| test(client, empty_query::EmptyQuery)),
        Box::new(|client| test(client, raw_query_string::RawQueryString)),
    ]
}

use run_tests::{test, Test};

mod no_index;

pub fn tests() -> Vec<Test> {
    vec![
        Box::new(|client| test(client, no_index::NoIndex))
    ]
}

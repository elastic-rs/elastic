use run_tests::{test, Test};

mod delete;
mod index_create;

pub fn tests() -> Vec<Test> {
    vec![
        Box::new(|client| test(client, delete::Delete)),
        Box::new(|client| test(client, index_create::IndexCreate)),
    ]
}

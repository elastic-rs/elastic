use run_tests::{test, Test};

mod exists;
mod does_not_exist;

pub fn tests() -> Vec<Test> {
    vec![
        Box::new(|client| test(client, exists::Exists)),
        Box::new(|client| test(client, does_not_exist::DoesNotExist)),
    ]
}

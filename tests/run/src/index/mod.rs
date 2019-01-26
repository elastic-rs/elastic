use run_tests::{
    test,
    Test,
};

mod does_not_exist;
mod exists;

pub fn tests() -> Vec<Test> {
    vec![
        Box::new(|client| test(client, exists::Exists)),
        Box::new(|client| test(client, does_not_exist::DoesNotExist)),
    ]
}

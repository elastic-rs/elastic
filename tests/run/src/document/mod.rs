use run_tests::{
    test,
    Test,
};

mod compile_test;

mod delete;
mod simple_index_get;
mod simple_mapping;
mod update_no_index;
mod update_with_doc;
mod update_with_inline_script;
mod update_with_script;

pub fn tests() -> Vec<Test> {
    vec![
        Box::new(|client| test(client, simple_index_get::SimpleIndexGet)),
        Box::new(|client| test(client, simple_mapping::SimpleMapping)),
        Box::new(|client| test(client, update_with_doc::UpdateWithDoc)),
        Box::new(|client| test(client, update_with_script::UpdateWithScript)),
        Box::new(|client| test(client, update_with_inline_script::UpdateWithInlineScript)),
        Box::new(|client| test(client, update_no_index::UpdateNoIndex)),
        Box::new(|client| test(client, delete::Delete)),
    ]
}

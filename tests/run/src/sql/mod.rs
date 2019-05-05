use run_tests::{
    test,
    Test,
};

mod invalid_query;
mod invalid_syntax;
mod select_all;

pub fn tests() -> Vec<Test> {
    vec![
        Box::new(|client| test(client, select_all::SelectAll)),
        Box::new(|client| test(client, invalid_syntax::InvalidSyntax)),
        Box::new(|client| test(client, invalid_query::InvalidQuery)),
    ]
}

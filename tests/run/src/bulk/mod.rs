use run_tests::{
    test,
    Test,
};

mod delete;
mod index_get;
mod index_create;
mod stream;
mod stream_tiny_size_limit;
mod stream_zero_size_limit;
mod stream_tiny_timeout;

pub fn tests() -> Vec<Test> {
    vec![
        Box::new(|client| test(client, delete::Delete)),
        Box::new(|client| test(client, index_get::IndexGet)),
        Box::new(|client| test(client, index_create::IndexCreate)),
        Box::new(|client| test(client, stream::BulkStream)),
        Box::new(|client| test(client, stream_tiny_size_limit::BulkStreamTinySize)),
        Box::new(|client| test(client, stream_zero_size_limit::BulkStreamZeroSize)),
        Box::new(|client| test(client, stream_tiny_timeout::BulkStreamTinyTimeout)),
    ]
}

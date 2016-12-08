mod genned;

pub use genned::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn it_works() {
        let req = SearchRequestParams::index_ty("test_index", "test_ty", vec![]);

        fn http<'a, I: Into<HttpRequest<'a>>>(_: I) {}

        http(&req);
    }
}

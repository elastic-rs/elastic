mod genned;

pub use genned::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn it_works() {
        let req =
            SearchRequestParams::index_ty("test_index", "test_ty", "{'query': { 'match_all': {}}}");

        fn do_something_with_request<'a, I: Into<HttpRequest<'a>>>(_: I) {}

        do_something_with_request(&req);
    }
}

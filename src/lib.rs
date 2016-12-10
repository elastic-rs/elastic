mod genned;

pub use genned::*;

#[cfg(test)]
mod tests {
    use std::thread;
    use super::*;

    fn do_something_with_request<'a, I: Into<HttpRequest<'a>>>(_: I) {}

    fn do_something_with_static_request<I: Into<HttpRequest<'static>>>
        (req: I)
         -> thread::JoinHandle<()> {
        let req = req.into();
        thread::spawn(move || {
            assert_eq!("/test_index/test_ty/_search", **req.url);
        })
    }

    fn do_something_with_into_static_request<'a, I: Into<HttpRequest<'a>>>
        (req: I)
         -> thread::JoinHandle<()> {
        let req = req.into().into_static();
        thread::spawn(move || {
            assert_eq!("/test_index/test_ty/_search", **req.url);
        })
    }

    #[test]
    fn it_works() {
        let req = SearchRequest::index_ty("test_index", "test_ty", "{'query': { 'match_all': {}}}");

        assert_eq!("/test_index/test_ty/_search", *req.url);

        do_something_with_request(&req);
        do_something_with_request(req);
    }

    #[test]
    fn it_works_static() {
        let req = SearchRequest::index_ty("test_index", "test_ty", "{'query': { 'match_all': {}}}");

        do_something_with_static_request(req).join().unwrap();
    }

    #[test]
    fn it_works_into_static() {
        let idx = String::from("test_index");

        let req = SearchRequest::index_ty(idx, "test_ty", "{'query': { 'match_all': {}}}");

        do_something_with_into_static_request(req).join().unwrap();
    }
}

//TODO: Work on a basic persistent HTTP connection pool
//Figure out what the role of the API should be. What exactly are we requesting?
//Should we just be constructing a request object to send to the loop?

// elastic::search::post_index_type(headers, "http://localhost:9200", "my_index", "my_type", json_str!({})) -> ElasticRequest
// Send the request to the loop. Do this automagically?
// Sending the request should return a promise that deserialises the response. See old kafka client work

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
    }
}

use elastic::client::requests::IndicesCreateRequest;

pub fn name() -> &'static str {
    INDEX
}

pub fn request() -> IndicesCreateRequest<'static> {
    IndicesCreateRequest::for_index(name(), bank_index())
}

const INDEX: &'static str = "bank-sample";

fn bank_index() -> String {
    let get_index = json_fn!(|filters, analysers| {
       "settings" : {
          "analysis" : {
             "filter" : $filters,
             "analyzer" : $analysers
          }
       }
    });

    get_index(&bank_filters(), &bank_analysers())
}

fn bank_filters() -> String {
    json_str!({
        "email": {
            "type": "pattern_capture",
            "preserve_original": 1,
            "patterns": [
                "([^@]+)",
                "(\\p{L}+)",
                "(\\d+)",
                "@(.+)"
            ]
        }
    })
}

fn bank_analysers() -> String {
    json_str!({
        "email": {
            "tokenizer": "uax_url_email",
            "filter": [
                "email",
                "lowercase",
                "unique"
            ]
        }
    })
}

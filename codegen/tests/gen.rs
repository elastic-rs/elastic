extern crate elastic_codegen;

/// See http://www.elastic.co/guide/en/elasticsearch/reference/master/docs-bulk.html
struct BulkReq {
	/// Explicit write consistency setting for the operation
	pub consistency: Consistency,
	/// Refresh the index after performing the operation
	pub refresh: bool,
	/// Specific routing value
	pub routing: String
}

enum Consistency {
	One,
	Quorum,
	All
}

impl BulkReq {
	pub fn get_url(index: Option<String>, es_type: Option<String>) -> &'static str {
		match (index, es_type) {
			(Some(index), Some(es_type)) => "/{index}/{es_type}/_bulk",
			(Some(index), None) => "/{index}/_bulk",
			_ => "/_bulk"
		}
	}
}
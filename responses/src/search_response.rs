use serde::Deserialize;

/// A successful response from a Query DSL query.
#[derive(Debug, Deserialize)]
pub struct SearchResponse<T> where
T: Deserialize {
    /// The time taken to complete a query in ms.
    pub took: u64,
    /// Whether or not the query timed out.
    pub timed_out: bool,
    /// Metadata on shard activity.
    #[serde(rename="_shards")]
    pub shards: Shards,
    /// Document results.
    pub hits: SearchHits<T>
}

/// Metadata on shard activity for a Query DSL query.
#[derive(Debug, Deserialize)]
pub struct Shards {
    /// The total number of shards involved in this query.
    pub total: u64,
    /// The total number of shards that successfully executed the query.
    pub successful: u64,
    /// The total number of shards that failed to execute the query.
    pub failed: u64
}

/// A collection of hits for a Query DSL query.
#[derive(Debug, Deserialize)]
pub struct SearchHits<T> where
T: Deserialize {
    /// The total number of hits.
    pub total: u64,
    /// Document results.
    pub hits:  Vec<Hit<T>>
}

/// An individual hit for a Query DSL query.
#[derive(Debug, Deserialize)]
pub struct Hit<T> where
T: Deserialize {
    /// The index of the hit.
    #[serde(rename="_index")]
    pub index: String,
    /// The type of the hit.
    #[serde(rename="_type")]
    pub doc_type: String,
    /// The id of the hit.
    #[serde(rename="_id")]
    pub id: String,
    /// The relevance score of the hit.
    #[serde(rename="_score")]
    pub score: Option<f64>,
    /// The source document data.
    #[serde(rename="_source")]
    pub source: Option<T>,
    /// The index timestamp of the hit.
    #[serde(rename="_timestamp")]
    pub timestamp: Option<f64>,
    /// The routing value of the hit.
    #[serde(rename="_routing")]
    pub routing: Option<String>
}
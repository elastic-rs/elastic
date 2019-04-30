/*!
Response types for a [sql request](https://www.elastic.co/guide/en/elasticsearch/reference/current/sql-rest.html)
*/

use parsing::IsOkOnSuccess;
use serde_json::Value;

/** Response for a [sql request][sql-request]. */
#[derive(Deserialize, Debug)]
pub struct SqlResponse {
    columns: Vec<SqlColumn>,
    rows: Vec<Vec<Value>>,
}

impl SqlResponse {
    /** Gets a reference to the result columns. */
    pub fn columns(&self) -> &Vec<SqlColumn> {
        &self.columns
    }

    /** Gets a reference to a vector of rows each a vector of a values. */
    pub fn rows(&self) -> &Vec<Vec<Value>> {
        &self.rows
    }
}

#[derive(Deserialize, Debug)]
pub struct SqlColumn {
    name: String,
    #[serde(rename = "type")]
    ty: String,
}

impl SqlColumn {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn ty(&self) -> &str {
        &self.ty
    }
}

impl IsOkOnSuccess for SqlResponse {}

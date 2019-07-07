/*!
Response types for a [sql request](https://www.elastic.co/guide/en/elasticsearch/reference/current/sql-rest.html).
*/

use crate::http::receiver::IsOkOnSuccess;
use serde_json::Value;

/** Response for a [sql request](https://www.elastic.co/guide/en/elasticsearch/reference/current/sql-rest.html). */
#[derive(Deserialize, Debug)]
pub struct SqlQueryResponse {
    columns: Vec<SqlColumn>,
    rows: Vec<SqlRow>,
}

impl SqlQueryResponse {
    /** Gets a reference to the result columns. */
    pub fn columns(&self) -> &[SqlColumn] {
        &self.columns
    }

    /** Gets a reference to the result rows. */
    pub fn rows(&self) -> &[SqlRow] {
        &self.rows
    }
}

/** A row in the result set. */
#[derive(Deserialize, Debug)]
pub struct SqlRow(Vec<Value>);

impl SqlRow {
    /** Gets a reference to the result columns. */
    pub fn columns(&self) -> &[Value] {
        &self.0
    }
}

/** A column in the result set. */
#[derive(Deserialize, Debug)]
pub struct SqlColumn {
    name: String,
    #[serde(rename = "type")]
    ty: String,
}

impl SqlColumn {
    /** The name of the column. */
    pub fn name(&self) -> &str {
        &self.name
    }

    /** The type of the column. */
    pub fn ty(&self) -> &str {
        &self.ty
    }
}

impl IsOkOnSuccess for SqlQueryResponse {}

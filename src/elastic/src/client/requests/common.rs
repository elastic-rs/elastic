/*!
Types that are common between requests.
*/

use std::ops::Not;

use serde::ser::{
    Serialize,
    Serializer,
};
use serde_json::{
    Map,
    Value,
};

/** Update an indexed document using a new document. */
#[derive(Serialize)]
pub struct Doc<TDocument> {
    doc: DocInner<TDocument>,
    #[serde(skip_serializing_if = "Not::not")]
    doc_as_upsert: bool,
    #[serde(rename = "_source")]
    source: Value,
}

impl<TDocument> Doc<TDocument> {
    pub(crate) fn empty() -> Self {
        Doc {
            doc: DocInner { inner: None },
            doc_as_upsert: false,
            source: false.into(),
        }
    }

    pub(crate) fn value(doc: TDocument) -> Self {
        Doc {
            doc: DocInner { inner: Some(doc) },
            doc_as_upsert: false,
            source: false.into(),
        }
    }

    pub(crate) fn doc_as_upsert(mut self) -> Self {
        self.doc_as_upsert = true;

        self
    }

    pub(crate) fn source(mut self, value: impl Into<Value>) -> Self {
        self.source = value.into();

        self
    }
}

struct DocInner<TDocument> {
    inner: Option<TDocument>,
}

impl<TDocument> Serialize for DocInner<TDocument>
where
    TDocument: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.inner {
            Some(ref doc) => doc.serialize(serializer),
            None => Value::Object(Map::new()).serialize(serializer),
        }
    }
}

/** A default set of script parameters. */
pub type DefaultParams = Map<String, Value>;

/** Update an indexed document using a script. */
#[derive(Serialize)]
pub struct Script<TParams> {
    script: ScriptInner<TParams>,
}

impl Script<DefaultParams> {
    /** Create a new script builder using the given source. */
    pub(crate) fn new<TScript>(source: TScript) -> Self
    where
        TScript: ToString,
    {
        ScriptBuilder::new(source).build()
    }
}

#[derive(Serialize)]
struct ScriptInner<TParams> {
    #[serde(rename = "inline")]
    source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    lang: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<TParams>,
}

/** A builder for an update script that can be configured before sending. */
pub struct ScriptBuilder<TParams> {
    source: String,
    lang: Option<String>,
    params: Option<TParams>,
}

impl ScriptBuilder<DefaultParams> {
    /** Create a new script builder using the given source. */
    pub fn new<TScript>(source: TScript) -> Self
    where
        TScript: ToString,
    {
        ScriptBuilder {
            source: source.to_string(),
            params: None,
            lang: None,
        }
    }

    /** Set a script parameter. */
    pub fn param<TKey, TValue>(mut self, key: TKey, value: TValue) -> Self
    where
        TKey: ToString,
        TValue: Into<Value>,
    {
        let mut params = self.params.unwrap_or_else(DefaultParams::new);
        params.insert(key.to_string(), value.into());

        self.params = Some(params);
        self
    }
}

impl<TParams> ScriptBuilder<TParams> {
    pub(crate) fn from_script(script: Script<TParams>) -> Self {
        let script = script.script;

        ScriptBuilder {
            source: script.source,
            lang: script.lang,
            params: script.params,
        }
    }

    /** Set the language for the update script. */
    pub fn lang<TLang>(mut self, lang: Option<TLang>) -> Self
    where
        TLang: ToString,
    {
        self.lang = lang.map(|lang| lang.to_string());
        self
    }

    /** Specify a new set of parameters for the update script. */
    pub fn params<TNewParams>(self, params: TNewParams) -> ScriptBuilder<TNewParams> {
        ScriptBuilder {
            source: self.source,
            lang: self.lang,
            params: Some(params),
        }
    }

    pub(crate) fn build(self) -> Script<TParams> {
        Script {
            script: ScriptInner {
                source: self.source,
                params: self.params,
                lang: self.lang,
            },
        }
    }
}

impl From<String> for ScriptBuilder<DefaultParams> {
    fn from(source: String) -> Self {
        ScriptBuilder::new(source)
    }
}

impl<'a> From<&'a str> for ScriptBuilder<DefaultParams> {
    fn from(source: &'a str) -> Self {
        ScriptBuilder::new(source)
    }
}

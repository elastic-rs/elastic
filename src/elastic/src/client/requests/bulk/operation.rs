use std::io::{self, Write};
use std::ops::Deref;

use serde::ser::{Serialize, Serializer, SerializeMap};
use serde_json;

use client::requests::params::{Index, Type, Id};
use client::requests::common::{Doc, Script, ScriptBuilder, DefaultParams};

pub use client::responses::bulk::Action;

/**
A bulk operation.
*/
pub struct BulkOperation<TValue> {
    action: Action,
    header: BulkHeader,
    inner: Option<TValue>,
}

#[derive(Serialize)]
struct BulkHeader {
    #[serde(rename = "_index", serialize_with = "serialize_param", skip_serializing_if = "Option::is_none")]
    index: Option<Index<'static>>,
    #[serde(rename = "_type", serialize_with = "serialize_param", skip_serializing_if = "Option::is_none")]
    ty: Option<Type<'static>>,
    #[serde(rename = "_id", serialize_with = "serialize_param", skip_serializing_if = "Option::is_none")]
    id: Option<Id<'static>>,
}

fn serialize_param<S, T>(field: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Deref<Target = str>,
{
    serializer.serialize_str(&*field.as_ref().expect("serialize `None` value"))
}

impl<TDocument> BulkOperation<Doc<TDocument>> {
    /**
    A convenient method for creating an index bulk operation.
    */
    pub fn new_index(doc: TDocument) -> Self {
        BulkOperation::new(Action::Index, Some(Doc::value(doc)))
    }

    /**
    A convenient method for creating an update bulk operation.
    */
    pub fn new_update(doc: TDocument) -> Self {
        BulkOperation::new(Action::Update, Some(Doc::value(doc)))
    }

    /**
    A convenient method for creating a create bulk operation.
    */
    pub fn new_create(doc: TDocument) -> Self {
        BulkOperation::new(Action::Create, Some(Doc::value(doc)))
    }
}

impl BulkOperation<()> {
    /**
    A convenient method for creating a delete bulk operation.
    */
    pub fn new_delete() -> Self {
        BulkOperation::new(Action::Delete, None)
    }
}

impl BulkOperation<Script<DefaultParams>> {
    /**
    A convenient method for creating an update bulk operation.
    */
    pub fn new_update_script<TScript>(script: TScript) -> Self
    where
        TScript: ToString,
    {
        BulkOperation::new(Action::Update, Some(Script::new(script)))
    }
}

impl<TParams> BulkOperation<Script<TParams>> {
    /**
    Set the script for this bulk operation.
    */
    pub fn script_fluent<TBuilder, TNewParams>(self, builder: TBuilder) -> BulkOperation<Script<TNewParams>>
    where
        TBuilder: Fn(ScriptBuilder<TParams>) -> ScriptBuilder<TNewParams>,
    {
        let inner = self.inner.map(|script| builder(ScriptBuilder::from_script(script)).build());

        BulkOperation {
            action: self.action,
            header: self.header,
            inner,
        }
    }
}

impl<TValue> BulkOperation<TValue> {
    /**
    Create a new operation for the given action.
    */
    fn new(action: Action, value: Option<TValue>) -> Self {
        BulkOperation {
            action,
            header: BulkHeader {
                index: None,
                ty: None,
                id: None,
            },
            inner: value,
        }
    }
    
    /**
    Set the index for this bulk operation.
    */
    pub fn index<I>(mut self, index: I) -> Self
    where
        I: Into<Index<'static>>,
    {
        self.header.index = Some(index.into());
        self
    }

    /**
    Set the type for this bulk operation.
    */
    pub fn ty<I>(mut self, ty: I) -> Self
    where
        I: Into<Type<'static>>,
    {
        self.header.ty = Some(ty.into());
        self
    }

    /**
    Set the id for this bulk operation.
    */
    pub fn id<I>(mut self, id: I) -> Self
    where
        I: Into<Id<'static>>,
    {
        self.header.id = Some(id.into());
        self
    }
}

impl<TDocument> BulkOperation<TDocument>
where
    TDocument: Serialize
{
    /**
    Write the operation to the given writer.

    Bulk operations have a particular line-delimited format.
    This method will write a json header, then a newline, then the document body.
    */
    pub fn write<W>(&self, mut writer: W) -> io::Result<()>
        where
            W: Write,
    {
        struct Header<'a> {
            action: Action,
            inner: &'a BulkHeader,
        }

        impl<'a> Serialize for Header<'a> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where S: Serializer
            {
                let mut map = serializer.serialize_map(Some(1))?;

                let k = match self.action {
                    Action::Create => "create",
                    Action::Delete => "delete",
                    Action::Index => "index",
                    Action::Update => "update",
                };

                map.serialize_entry(k, &self.inner)?;

                map.end()
            }
        }

        serde_json::to_writer(&mut writer, &Header { action: self.action, inner: &self.header })?;
        write!(&mut writer, "\n")?;

        if let Some(ref inner) = self.inner {
            serde_json::to_writer(&mut writer, inner)?;
            write!(&mut writer, "\n")?;
        }

        Ok(())
    }
}

/**
A convenient method for creating an index bulk operation.
*/
pub fn bulk_index<TDocument>(doc: TDocument) -> BulkOperation<Doc<TDocument>> {
    BulkOperation::new_index(doc)
}

/**
A convenient method for creating an update bulk operation.
*/
pub fn bulk_update<TDocument>(doc: TDocument) -> BulkOperation<Doc<TDocument>> {
    BulkOperation::new_update(doc)
}

/**
A convenient method for creating a create bulk operation.
*/
pub fn bulk_create<TDocument>(doc: TDocument) -> BulkOperation<Doc<TDocument>> {
    BulkOperation::new_create(doc)
}

/**
A convenient method for creating an update bulk operation.
*/
pub fn bulk_update_script<TScript>(script: TScript) -> BulkOperation<Script<DefaultParams>>
where
    TScript: ToString,
{
    BulkOperation::new_update_script(script)
}

/**
A convenient method for creating an update bulk operation.
*/
pub fn bulk_update_script_fluent<TScript, TBuilder, TParams>(script: TScript, builder: TBuilder) -> BulkOperation<Script<TParams>>
    where
        TScript: ToString,
        TBuilder: Fn(ScriptBuilder<DefaultParams>) -> ScriptBuilder<TParams>,
{
    BulkOperation::new_update_script(script).script_fluent(builder)
}

/**
A convenient method for creating a delete bulk operation.
*/
pub fn bulk_delete() -> BulkOperation<()> {
    BulkOperation::new(Action::Delete, None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_doc() {
        unimplemented!()
    }

    #[test]
    fn index_doc() {
        unimplemented!()
    }

    #[test]
    fn update_doc() {
        unimplemented!()
    }

    #[test]
    fn update_script() {
        unimplemented!()
    }

    #[test]
    fn delete() {
        unimplemented!()
    }
}
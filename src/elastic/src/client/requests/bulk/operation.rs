use std::{
    io::{
        self,
        Write,
    },
    marker::PhantomData,
    ops::Deref,
};

use serde::ser::{
    Serialize,
    SerializeMap,
    Serializer,
};
use serde_json;

use crate::{
    client::requests::common::{
        DefaultParams,
        Doc,
        Script,
        ScriptBuilder,
    },
    params::{
        Id,
        Index,
        Type,
    },
    types::document::DocumentType,
};

pub use crate::client::responses::bulk::Action;

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
    #[serde(
        rename = "_index",
        serialize_with = "serialize_param",
        skip_serializing_if = "Option::is_none"
    )]
    index: Option<Index<'static>>,
    #[serde(
        rename = "_type",
        serialize_with = "serialize_param",
        skip_serializing_if = "Option::is_none"
    )]
    ty: Option<Type<'static>>,
    #[serde(
        rename = "_id",
        serialize_with = "serialize_param",
        skip_serializing_if = "Option::is_none"
    )]
    id: Option<Id<'static>>,
}

fn serialize_param<S, T>(field: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Deref<Target = str>,
{
    serializer.serialize_str(&*field.as_ref().expect("serialize `None` value"))
}

impl<TParams> BulkOperation<Script<TParams>> {
    /**
    Set the script for this bulk operation.
    */
    pub fn script_fluent<TBuilder, TNewParams>(
        self,
        builder: TBuilder,
    ) -> BulkOperation<Script<TNewParams>>
    where
        TBuilder: Fn(ScriptBuilder<TParams>) -> ScriptBuilder<TNewParams>,
    {
        let inner = self
            .inner
            .map(|script| builder(ScriptBuilder::from_script(script)).build());

        BulkOperation {
            action: self.action,
            header: self.header,
            inner,
        }
    }
}

impl<TValue> BulkOperation<TValue> {
    /**
    Set the index for this bulk operation.
    */
    pub fn index(mut self, index: impl Into<Index<'static>>) -> Self {
        self.header.index = Some(index.into());
        self
    }

    /**
    Set the type for this bulk operation.
    */
    pub fn ty(mut self, ty: impl Into<Type<'static>>) -> Self {
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

impl<TDocument> BulkOperation<Doc<TDocument>>
where
    TDocument: Serialize,
{
    pub fn doc_as_upsert(mut self) -> Self {
        if let Some(inner) = self.inner {
            self.inner = Some(inner.doc_as_upsert());
        };

        self
    }
}

impl<TDocument> BulkOperation<TDocument>
where
    TDocument: Serialize,
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
            where
                S: Serializer,
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

        serde_json::to_writer(
            &mut writer,
            &Header {
                action: self.action,
                inner: &self.header,
            },
        )?;
        write!(&mut writer, "\n")?;

        if let Some(ref inner) = self.inner {
            serde_json::to_writer(&mut writer, inner)?;
            write!(&mut writer, "\n")?;
        }

        Ok(())
    }
}

pub struct BulkDocumentOperation<TDocument> {
    _marker: PhantomData<TDocument>,
}

impl<TDocument> BulkDocumentOperation<TDocument>
where
    TDocument: DocumentType,
{
    pub fn new() -> Self {
        BulkDocumentOperation {
            _marker: PhantomData,
        }
    }

    pub fn index(self, doc: TDocument) -> BulkOperation<TDocument> {
        BulkOperation {
            action: Action::Index,
            header: BulkHeader {
                index: Some(doc.index().to_owned()),
                ty: Some(doc.ty().to_owned()),
                id: doc.partial_id().map(|id| id.to_owned()),
            },
            inner: Some(doc),
        }
    }

    pub fn update(self, doc: TDocument) -> BulkOperation<Doc<TDocument>> {
        BulkOperation {
            action: Action::Update,
            header: BulkHeader {
                index: Some(doc.index().to_owned()),
                ty: Some(doc.ty().to_owned()),
                id: doc.partial_id().map(|id| id.to_owned()),
            },
            inner: Some(Doc::value(doc)),
        }
    }

    pub fn update_script<TId, TScript>(
        self,
        id: TId,
        script: TScript,
    ) -> BulkOperation<Script<DefaultParams>>
    where
        TId: Into<Id<'static>>,
        TScript: ToString,
    {
        BulkOperation {
            action: Action::Update,
            header: BulkHeader {
                index: TDocument::partial_static_index(),
                ty: TDocument::partial_static_ty(),
                id: Some(id.into()),
            },
            inner: Some(Script::new(script)),
        }
    }

    pub fn update_script_fluent<TId, TScript, TBuilder, TParams>(
        self,
        id: TId,
        script: TScript,
        builder: TBuilder,
    ) -> BulkOperation<Script<TParams>>
    where
        TId: Into<Id<'static>>,
        TScript: ToString,
        TBuilder: Fn(ScriptBuilder<DefaultParams>) -> ScriptBuilder<TParams>,
    {
        BulkOperation {
            action: Action::Update,
            header: BulkHeader {
                index: TDocument::partial_static_index().map(Into::into),
                ty: TDocument::partial_static_ty().map(Into::into),
                id: Some(id.into()),
            },
            inner: Some(Script::new(script)),
        }
        .script_fluent(builder)
    }

    pub fn create(self, doc: TDocument) -> BulkOperation<TDocument> {
        BulkOperation {
            action: Action::Create,
            header: BulkHeader {
                index: Some(doc.index().to_owned()),
                ty: Some(doc.ty().to_owned()),
                id: doc.partial_id().map(|id| id.to_owned()),
            },
            inner: Some(doc),
        }
    }

    pub fn delete<TId>(self, id: TId) -> BulkOperation<()>
    where
        TId: Into<Id<'static>>,
    {
        BulkOperation {
            action: Action::Delete,
            header: BulkHeader {
                index: TDocument::partial_static_index(),
                ty: TDocument::partial_static_ty(),
                id: Some(id.into()),
            },
            inner: None,
        }
    }
}

pub fn bulk<TDocument>() -> BulkDocumentOperation<TDocument>
where
    TDocument: DocumentType,
{
    BulkDocumentOperation::new()
}

pub struct BulkRawOperation {
    _private: (),
}

impl BulkRawOperation {
    pub fn new() -> Self {
        BulkRawOperation { _private: () }
    }

    pub fn index<TDocument>(self, doc: TDocument) -> BulkOperation<TDocument> {
        BulkOperation {
            action: Action::Index,
            header: BulkHeader {
                index: None,
                ty: None,
                id: None,
            },
            inner: Some(doc),
        }
    }

    pub fn update<TDocument>(self, doc: TDocument) -> BulkOperation<Doc<TDocument>> {
        BulkOperation {
            action: Action::Update,
            header: BulkHeader {
                index: None,
                ty: None,
                id: None,
            },
            inner: Some(Doc::value(doc)),
        }
    }

    pub fn update_script<TScript>(self, script: TScript) -> BulkOperation<Script<DefaultParams>>
    where
        TScript: ToString,
    {
        BulkOperation {
            action: Action::Update,
            header: BulkHeader {
                index: None,
                ty: None,
                id: None,
            },
            inner: Some(Script::new(script)),
        }
    }

    pub fn update_script_fluent<TId, TScript, TBuilder, TParams>(
        self,
        script: TScript,
        builder: TBuilder,
    ) -> BulkOperation<Script<TParams>>
    where
        TScript: ToString,
        TBuilder: Fn(ScriptBuilder<DefaultParams>) -> ScriptBuilder<TParams>,
    {
        BulkOperation {
            action: Action::Update,
            header: BulkHeader {
                index: None,
                ty: None,
                id: None,
            },
            inner: Some(Script::new(script)),
        }
        .script_fluent(builder)
    }

    pub fn create<TDocument>(self, doc: TDocument) -> BulkOperation<TDocument> {
        BulkOperation {
            action: Action::Create,
            header: BulkHeader {
                index: None,
                ty: None,
                id: None,
            },
            inner: Some(doc),
        }
    }

    pub fn delete(self) -> BulkOperation<()> {
        BulkOperation {
            action: Action::Delete,
            header: BulkHeader {
                index: None,
                ty: None,
                id: None,
            },
            inner: None,
        }
    }
}

pub fn bulk_raw() -> BulkRawOperation {
    BulkRawOperation::new()
}

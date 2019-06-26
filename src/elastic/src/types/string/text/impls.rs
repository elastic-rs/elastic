use super::mapping::{
    TextFieldType,
    TextMapping,
};
use crate::types::string::mapping::DefaultStringMapping;
use serde::{
    de::{
        Error,
        Visitor,
    },
    Deserialize,
    Deserializer,
    Serialize,
    Serializer,
};
use std::{
    borrow::Borrow,
    marker::PhantomData,
};

impl TextFieldType<DefaultStringMapping> for String {}
impl<'a> TextFieldType<DefaultStringMapping> for &'a str {}

/**
An Elasticsearch `text` field with a mapping.

Where the mapping isn't custom, you can use the standard library `String` instead.

# Examples

Defining a `text` field with a mapping:

```
use elastic::types::string::text::mapping::DefaultTextMapping;
use elastic::types::string::text::Text;

let string = Text::<DefaultTextMapping>::new("my string value");
```
*/
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Text<TMapping>
where
    TMapping: TextMapping,
{
    value: String,
    _m: PhantomData<TMapping>,
}

impl<TMapping> Text<TMapping>
where
    TMapping: TextMapping,
{
    /**
    Creates a new `Text` with the given mapping.

    # Examples

    Create a new `Text` from a `String`:

    ```
    use elastic::types::string::text::mapping::DefaultTextMapping;
    use elastic::types::string::text::Text;

    let string = Text::<DefaultTextMapping>::new("my string");
    ```
    */
    pub fn new<I>(string: I) -> Text<TMapping>
    where
        I: Into<String>,
    {
        Text {
            value: string.into(),
            _m: PhantomData,
        }
    }

    /** Change the mapping of this string. */
    pub fn remap<TNewMapping>(text: Text<TMapping>) -> Text<TNewMapping>
    where
        TNewMapping: TextMapping,
    {
        Text::new(text.value)
    }
}

impl_string_type!(Text, TextMapping, TextFieldType);

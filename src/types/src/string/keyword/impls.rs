use super::mapping::{
    KeywordFieldType,
    KeywordMapping,
};
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

/**
An Elasticsearch `keyword` with a mapping.

Where the mapping isn't custom, you can use the standard library `String` instead.

# Examples

Defining a `keyword` with a mapping:

```
use elastic_types::string::keyword::mapping::DefaultKeywordMapping;
use elastic_types::string::keyword::Keyword;

let string = Keyword::<DefaultKeywordMapping>::new("my string value");
```
*/
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Keyword<TMapping>
where
    TMapping: KeywordMapping,
{
    value: String,
    _m: PhantomData<TMapping>,
}

impl<TMapping> Keyword<TMapping>
where
    TMapping: KeywordMapping,
{
    /**
    Creates a new `Keyword` with the given mapping.

    # Examples

    Create a new `Keyword` from a `String`:

    ```
    use elastic_types::string::keyword::mapping::DefaultKeywordMapping;
    use elastic_types::string::keyword::Keyword;

    let string = Keyword::<DefaultKeywordMapping>::new("my string");
    ```
    */
    pub fn new<I>(string: I) -> Keyword<TMapping>
    where
        I: Into<String>,
    {
        Keyword {
            value: string.into(),
            _m: PhantomData,
        }
    }

    /** Change the mapping of this string. */
    pub fn remap<TNewMapping>(keyword: Keyword<TMapping>) -> Keyword<TNewMapping>
    where
        TNewMapping: KeywordMapping,
    {
        Keyword::new(keyword.value)
    }
}

impl_string_type!(Keyword, KeywordMapping, KeywordFieldType);

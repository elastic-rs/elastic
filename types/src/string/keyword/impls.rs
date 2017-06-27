use std::marker::PhantomData;
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde::de::{Visitor, Error};
use super::mapping::{KeywordFieldType, KeywordMapping};

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
pub struct Keyword<M>  where M: KeywordMapping {
    value: String,
    _m: PhantomData<M>,
}

impl<M> Keyword<M>
    where M: KeywordMapping
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
    pub fn new<I>(string: I) -> Keyword<M>
        where I: Into<String>
    {
        Keyword {
            value: string.into(),
            _m: PhantomData,
        }
    }

    /** Change the mapping of this string. */
    pub fn remap<MInto>(self) -> Keyword<MInto>
        where MInto: KeywordMapping
    {
        Keyword::<MInto>::new(self.value)
    }
}

impl_string_type!(Keyword, KeywordMapping, KeywordFieldType);

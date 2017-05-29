use std::marker::PhantomData;
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde::de::{Visitor, Error};
use super::mapping::{TextFieldType, TextMapping};
use string::mapping::DefaultStringMapping;

impl TextFieldType<DefaultStringMapping> for String {}
impl TextFieldType<DefaultStringMapping> for &'static str {}

/**
An Elasticsearch `text` field with a mapping.

Where the mapping isn't custom, you can use the standard library `String` instead.

# Examples

Defining a `text` field with a mapping:

```
use elastic_types::string::text::mapping::DefaultTextMapping;
use elastic_types::string::text::Text;

let string = Text::<DefaultTextMapping>::new("my string value");
```
*/
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Text<M>
    where M: TextMapping
{
    value: String,
    _m: PhantomData<M>,
}
impl<M> Text<M>
    where M: TextMapping
{
    /**
    Creates a new `Text` with the given mapping.
    
    # Examples
    
    Create a new `Text` from a `String`:
    
    ```
    use elastic_types::string::text::mapping::DefaultTextMapping;
    use elastic_types::string::text::Text;
    
    let string = Text::<DefaultTextMapping>::new("my string");
    ```
    */
    pub fn new<I>(string: I) -> Text<M>
        where I: Into<String>
    {
        Text {
            value: string.into(),
            _m: PhantomData,
        }
    }

    /** Change the mapping of this string. */
    pub fn remap<MInto>(self) -> Text<MInto>
        where MInto: TextMapping
    {
        Text::<MInto>::new(self.value)
    }
}

impl_string_type!(Text, TextMapping, TextFieldType);

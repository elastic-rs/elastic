/*!
Implementation of the Elasticsearch `keyword` and `text` types.

Keyword fields are stored as a raw string of tokens, and aren't analysed when querying.
They're useful for data that only has meaning when considered as a whole, like an id
or single word.

Text fields are stored as a sequence of tokens, constructed based on the given `analyzer`.
They're useful for blobs of content that can be sliced in various ways, like prose.

As far as serialisation is concerned, `keyword` and `text` are equivalent.

# Examples

For defining your own string mapping, see:

- [keyword mapping details](keyword/mapping/trait.KeywordMapping.html#derive-mapping)
- [text mapping details](text/mapping/trait.TextMapping.html#derive-mapping).

Map with a default `string` (follows the semantics for legacy `string` mapping):

```
struct MyType {
    pub field: String
}
```

Map a `keyword`:

```
# fn main() {
# use elastic::types::prelude::*;
# use elastic::types::string::prelude::*;
struct MyType {
    pub field: Keyword<DefaultKeywordMapping>
}
# }
```

Map `text`:

```
# fn main() {
# use elastic::types::prelude::*;
# use elastic::types::string::prelude::*;
struct MyType {
    pub field: Text<DefaultTextMapping>
}
# }
```

Map a custom type as a `keyword` field.
This is especially useful for simple `enum`s:

```
#[macro_use] extern crate serde_derive;
# fn main() {
# use elastic::types::prelude::*;
#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
enum MyKeywordField {
    VariantA,
    VariantB,
    VariantC,
}

impl KeywordFieldType<DefaultKeywordMapping> for MyKeywordField {}
# }
```

# Links

- [Elasticsearch Doc](https://www.elastic.co/guide/en/elasticsearch/reference/master/string.html)
*/

#[macro_use]
mod macros;

pub mod keyword;
pub mod text;

pub mod mapping;

pub use self::{
    keyword::Keyword,
    text::Text,
};

pub mod prelude {
    /*!
    Includes all types for the `string` types.

    This is a convenience module to make it easy to build mappings for multiple types without too many `use` statements.
    */

    pub use super::{
        keyword::prelude::*,
        mapping::*,
        text::prelude::*,
    };
}

#[cfg(test)]
mod tests {
    use serde_json;

    use crate::types::prelude::*;

    #[derive(Default)]
    struct MyKeywordMapping;
    impl KeywordMapping for MyKeywordMapping {}

    #[derive(Default)]
    struct MyTextMapping;
    impl TextMapping for MyTextMapping {}

    #[test]
    fn can_change_keyword_mapping() {
        fn takes_custom_mapping(_: Keyword<MyKeywordMapping>) -> bool {
            true
        }

        let string: Keyword<DefaultKeywordMapping> = Keyword::new("stuff");

        assert!(takes_custom_mapping(Keyword::remap(string)));
    }

    #[test]
    fn serialise_elastic_keyword() {
        let string: Keyword<DefaultKeywordMapping> = Keyword::new("my string");

        let ser = serde_json::to_string(&string).unwrap();

        assert_eq!(r#""my string""#, ser);
    }

    #[test]
    fn deserialise_elastic_keyword() {
        let string: Keyword<DefaultKeywordMapping> =
            serde_json::from_str(r#""my string""#).unwrap();

        assert_eq!("my string", string);
    }

    #[test]
    fn can_change_text_mapping() {
        fn takes_custom_mapping(_: Text<MyTextMapping>) -> bool {
            true
        }

        let string: Text<DefaultTextMapping> = Text::new("stuff");

        assert!(takes_custom_mapping(Text::remap(string)));
    }

    #[test]
    fn serialise_elastic_text() {
        let string: Text<DefaultTextMapping> = Text::new("my string");

        let ser = serde_json::to_string(&string).unwrap();

        assert_eq!(r#""my string""#, ser);
    }

    #[test]
    fn deserialise_elastic_text() {
        let string: Text<DefaultTextMapping> = serde_json::from_str(r#""my string""#).unwrap();

        assert_eq!("my string", string);
    }
}

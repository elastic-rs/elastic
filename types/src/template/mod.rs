//! Requirements for creating an index template for predefined mapping types.
//!
//! The [`IndexTemplate`](struct.IndexTemplate.html) struct allows you to build a request to the [Put Index Template API](https://www.elastic.co/guide/en/elasticsearch/reference/master/indices-templates.html),
//! using strongly typed `ObjectMapping`s.
//!
//! > NOTE: Only the `mappings` property on the index template is supported here,
//! if you want to customise `aliases` and other index settings
//! you can create multiple index templates in Elasticsearch with the same `template` pattern.
//! The properties of all matching templates will be merged when an index is created.
//!
//! # Links
//!
//! - [Elasticsearch Doc](https://www.elastic.co/guide/en/elasticsearch/reference/master/indices-templates.html)

use std::collections::BTreeMap;
use serde::{Serialize, Serializer};
use serde_json::{Value, Error};
use ::object::ObjectMapping;
use ::mappers::TypeMapper;

/// A structure for predefining mappings with an index template.
///
/// > NOTE: This structure is quite slow to serialise, so you should use it sparingly.
///
/// # Examples
///
/// Build an index template with predefined mappings.
/// In this example we have two mapping types: `MyTypeMapping` and `MyOtherTypeMapping`:
///
/// ```
/// # #![feature(proc_macro)]
/// # #[macro_use]
/// # extern crate json_str;
/// # #[macro_use]
/// # extern crate serde_derive;
/// # #[macro_use]
/// # extern crate elastic_types_derive;
/// # #[macro_use]
/// # extern crate elastic_types;
/// # extern crate serde;
/// # extern crate serde_json;
/// # use elastic_types::prelude::*;
/// # #[derive(Serialize, Deserialize, ElasticType)]
/// # pub struct MyType {
/// # 	pub my_date: Date<DefaultDateFormat>
/// # }
/// # #[derive(Serialize, Deserialize, ElasticType)]
/// # pub struct MyOtherType {
/// # 	pub my_num: i32
/// # }
/// # fn main() {
/// let mut template = IndexTemplate::new("template_name", "data-*", MyTypeMapping).unwrap();
///
/// template.add_mapping(MyOtherTypeMapping).unwrap();
///
/// let ser = serde_json::to_string(&template);
/// # }
/// ```
///
/// Which will produce the following result:
///
/// ```
/// # #![feature(proc_macro)]
/// # #[macro_use]
/// # extern crate json_str;
/// # #[macro_use]
/// # extern crate serde_derive;
/// # #[macro_use]
/// # extern crate elastic_types_derive;
/// # #[macro_use]
/// # extern crate elastic_types;
/// # extern crate serde;
/// # extern crate serde_json;
/// # use elastic_types::prelude::*;
/// # #[derive(Serialize, Deserialize, ElasticType)]
/// # pub struct MyType {
/// # 	pub my_date: Date<DefaultDateFormat>
/// # }
/// # #[derive(Serialize, Deserialize, ElasticType)]
/// # pub struct MyOtherType {
/// # 	pub my_num: i32
/// # }
/// # fn main() {
/// # let mut template = IndexTemplate::new("template_name", "data-*", MyTypeMapping).unwrap();
/// # template.add_mapping(MyOtherTypeMapping).unwrap();
/// # let mapping = serde_json::to_string(&template).unwrap();
/// # let json = json_str!(
/// {
/// 	"template": "data-*",
/// 	"order": 0,
/// 	"mappings": {
/// 		"myothertype": {
/// 			"properties": {
/// 				"my_num": {
/// 					"type": "integer"
/// 				}
/// 			}
/// 		},
/// 		"mytype": {
/// 			"properties": {
/// 				"my_date": {
/// 					"format": "basic_date_time",
/// 					"type": "date"
/// 				}
/// 			}
/// 		}
/// 	}
/// }
/// # );
/// # assert_eq!(json, mapping);
/// # }
/// ```
///
/// # Links
///
/// - [Elasticsearch Doc](https://www.elastic.co/guide/en/elasticsearch/reference/master/indices-templates.html)
pub struct IndexTemplate {
    /// The name of the index template.
    pub name: &'static str,
    /// The index name pattern to match.
    pub template: &'static str,
    /// An order used to resolve property value conflicts when multiple templates match an index.
    pub order: i32,
    mappings: BTreeMap<&'static str, Value>,
}

impl IndexTemplate {
    /// Create a new `IndexTemplate` for the given pattern and an initial mapping.
    ///
    /// The `IndexTemplate` requires at least one type mapping, so this method
    /// takes one to start off with.
    pub fn new<M>(name: &'static str, template: &'static str, t: M) -> Result<Self, Error>
        where M: ObjectMapping
    {
        let mut tmpl = IndexTemplate {
            name: name,
            template: template,
            order: 0,
            mappings: BTreeMap::new(),
        };

        try!(tmpl.add_mapping(t));

        Ok(tmpl)
    }

    /// Add the given mapping type to the template if it doesn't exist already.
    pub fn add_mapping<M>(&mut self, t: M) -> Result<(), Error>
        where M: ObjectMapping
    {
        if !self.mappings.contains_key(M::name()) {
            let m = try!(TypeMapper::to_value(t));
            self.mappings.insert(M::name(), m);
        }

        Ok(())
    }
}

impl Serialize for IndexTemplate {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        let mut state = try!(serializer.serialize_struct("mapping", 3));

        try!(serializer.serialize_struct_elt(&mut state, "template", self.template));
        try!(serializer.serialize_struct_elt(&mut state, "order", self.order));
        try!(serializer.serialize_struct_elt(&mut state, "mappings", &self.mappings));

        serializer.serialize_struct_end(state)
    }
}

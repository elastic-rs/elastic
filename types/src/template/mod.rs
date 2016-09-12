//! Requirements for creating an index template for predefined mapping types.

use std::collections::BTreeMap;
use serde::{ Serialize, Serializer };
use serde_json::{ Value, Error };
use ::object::ObjectMapping;
use ::mappers::TypeMapper;

/// A structure for predefining mappings with an [index template](https://www.elastic.co/guide/en/elasticsearch/reference/master/indices-templates.html).
/// 
/// Only the `mappings` property is supported here, if you want to customise `aliases` index settings,
/// you can create multiple index templates in Elasticsearch with the same `template`.
/// The properties of all matching templates will be merged when an index is created.
pub struct IndexTemplate {
	/// The name of the index template.
	pub name: &'static str,
	/// The index name pattern to match.
	pub template: &'static str,
	/// An order used to resolve property value conflicts when multiple templates match an index.
	pub order: i32,
	mappings: BTreeMap<&'static str, Value>
}

impl IndexTemplate {
	/// Create a new `IndexTemplate` for the given pattern and an initial mapping.
	/// 
	/// The `IndexTemplate` requires at least one type mapping, so this method
	/// takes one to start off with.
	pub fn new<M>(name: &'static str, template: &'static str, t: M) -> Result<Self, Error> where
	M: ObjectMapping {
		let mut tmpl = IndexTemplate {
			name: name,
			template: template,
			order: 0,
			mappings: BTreeMap::new()
		};

		try!(tmpl.add_mapping(t));

		Ok(tmpl)
	}

	/// Add the given mapping type to the template if it doesn't exist already.
	pub fn add_mapping<M>(&mut self, t: M) -> Result<(), Error>  where
	M: ObjectMapping {
		if !self.mappings.contains_key(M::name()) {
			let m = try!(TypeMapper::to_value(t));
			self.mappings.insert(M::name(), m);
		}

		Ok(())
	}
}

impl Serialize for IndexTemplate {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where 
	S: Serializer {
		let mut state = try!(serializer.serialize_struct("mapping", 3));

		try!(serializer.serialize_struct_elt(&mut state, "template", self.template));
		try!(serializer.serialize_struct_elt(&mut state, "order", self.order));
		try!(serializer.serialize_struct_elt(&mut state, "mappings", &self.mappings));

		serializer.serialize_struct_end(state)
	}
}
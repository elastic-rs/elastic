use std::collections::BTreeMap;
use serde::{ Serialize, Serializer };
use serde_json::{ Value, Error };
use ::object::ObjectMapping;
use ::mappers::TypeMapper;

pub struct IndexTemplate {
	pub template: &'static str,
	pub order: Option<i32>,
	mappings: BTreeMap<&'static str, Value>
}

impl IndexTemplate {
	pub fn new(template: &'static str) -> Self {
		IndexTemplate {
			template: template,
			order: None,
			mappings: BTreeMap::new()
		}
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
		try!(serializer.serialize_struct_elt(&mut state, "order", &self.order));
		try!(serializer.serialize_struct_elt(&mut state, "mappings", &self.mappings));

		serializer.serialize_struct_end(state)
	}
}
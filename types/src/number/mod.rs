use std::marker::PhantomData;
use serde;
use serde::{ Serialize, Serializer };
use super::ElasticObjectTypeVisitor;
use ::mapping::{ ElasticTypeMapping };

/// The base requirements for mapping a `number` type.
/// 
/// Numbers can 
pub trait ElasticNumberMapping<T>
where Self : ElasticTypeMapping<()> + Sized + Serialize + Default + Clone {
	fn coerce() -> Option<bool> {
		None
	}

	fn boost() -> Option<bool> {
		None
	}

	fn doc_values() -> Option<bool> {
		None
	}

	fn ignore_malformed() -> Option<bool> {
		None
	}

	fn include_in_all() -> Option<bool> {
		None
	}

	fn index() -> Option<bool> {
		None
	}

	fn null_value() -> Option<T> {
		None
	}

	fn precision_step() -> Option<bool> {
		None
	}

	fn store() -> Option<bool> {
		None
	}
}

//TODO: Visitors for number types. Each one is unique
//TODO: ElasticNumber<T, M: ElasticNumberMapping<T>>

/// Visitor for a `number` field mapping.
#[derive(Debug, PartialEq, Default)]
pub struct ElasticNumberMappingVisitor<T, M: ElasticNumberMapping<T>> {
	phantom_t: PhantomData<T>,
	phantom_m: PhantomData<M>
}

impl <T, M: ElasticNumberMapping<T>> serde::ser::MapVisitor for ElasticNumberMappingVisitor<T, M> {
	fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
	where S: Serializer {
		try!(serializer.serialize_struct_elt("type", M::data_type()));

		Ok(None)
	}
}
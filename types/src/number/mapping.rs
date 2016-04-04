//! Mapping for the Elasticsearch `number` types.

use std::marker::PhantomData;
use serde;
use serde::{ Serialize, Deserialize, Serializer };
use ::mapping::{ ElasticType, ElasticTypeMapping, IndexAnalysis };

/// The base requirements for mapping a `number` type.
/// 
/// Custom mappings can be defined by implementing `ElasticNumberMapping` for some Rust primitive number type.
/// Currently, the link between `null_value`, `data_type` and the type value that's actually used by `ElasticNumber` is implied.
/// This isn't ideal, but just keep in mind that specifying your `number` takes an `Integer` won't force end-users
/// to use an `i32` on `ElasticNumber`.
/// 
/// # Examples
/// 
/// Define a custom `ElasticNumberMapping` for `i32`:
/// 
/// ## With Macros
/// 
/// ```
/// # extern crate serde;
/// # #[macro_use]
/// # extern crate elastic_types;
/// # fn main() {
/// use elastic_types::mapping::prelude::*;
/// use elastic_types::number::prelude::*;
/// 
/// #[derive(Debug, Clone, Default)]
/// pub struct MyNumberMapping;
/// impl ElasticNumberMapping for MyNumberMapping {
/// 	//Overload the mapping functions here
/// 	fn null_value() -> Option<Number> {
/// 		Some(Number::Integer(42))
/// 	}
/// }
/// 
/// impl_number_mapping!(MyNumberMapping, "integer");
/// # }
/// ```
/// 
/// ## Manually
/// 
/// ```
/// # extern crate serde;
/// # extern crate elastic_types;
/// # fn main() {
/// use elastic_types::mapping::prelude::*;
/// use elastic_types::number::prelude::*;
/// 
/// #[derive(Debug, Clone, Default)]
/// pub struct MyNumberMapping;
/// impl ElasticNumberMapping for MyNumberMapping {
/// 	//Overload the mapping functions here
/// 	fn null_value() -> Option<Number> {
/// 		Some(Number::Integer(42))
/// 	}
/// }
/// 
/// impl ElasticTypeMapping<()> for MyNumberMapping {
/// 	type Visitor = ElasticNumberMappingVisitor<MyNumberMapping>;
/// 	fn data_type() -> &'static str {
/// 		"integer"
/// 	}
/// }
/// 
/// impl serde::Serialize for MyNumberMapping {
/// 	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
/// 	where S: serde::Serializer {
/// 		serializer.serialize_struct("mapping", Self::get_visitor())
/// 	}
/// }
/// # }
/// ```
pub trait ElasticNumberMapping
where Self : ElasticTypeMapping<()> + Sized + Serialize + Default + Clone {
	/// Try to convert strings to numbers and truncate fractions for integers. Accepts `true` (default) and `false`.
	fn coerce() -> Option<bool> {
		None
	}

	/// Field-level index time boosting. Accepts a floating point number, defaults to `1.0`.
	fn boost() -> Option<bool> {
		None
	}

	/// Should the field be stored on disk in a column-stride fashion, 
	/// so that it can later be used for sorting, aggregations, or scripting? 
	/// Accepts `true` (default) or `false`.
	fn doc_values() -> Option<bool> {
		None
	}

	/// If `true`, malformed numbers are ignored. If `false` (default), 
	/// malformed numbers throw an exception and reject the whole document.
	fn ignore_malformed() -> Option<bool> {
		None
	}

	/// Whether or not the field value should be included in the `_all` field? 
	/// Accepts `true` or `false`. Defaults to false if index is set to no, 
	/// or if a parent object field sets `include_in_all` to false. 
	/// Otherwise defaults to `true`.
	fn include_in_all() -> Option<bool> {
		None
	}

	/// Should the field be searchable? Accepts `not_analyzed` (default) and `no`.
	fn index() -> Option<IndexAnalysis> {
		None
	}

	/// Accepts a numeric value of the same type as the field which is substituted for any explicit null values. 
	/// Defaults to `null`, which means the field is treated as missing.
	fn null_value() -> Option<Number> {
		None
	}

	/// Controls the number of extra terms that are indexed to make range queries faster. 
	/// The default depends on the numeric type.
	fn precision_step() -> Option<bool> {
		None
	}

	/// Whether the field value should be stored and retrievable separately from the `_source` field. 
	/// Accepts true or false (default).
	fn store() -> Option<bool> {
		None
	}
}

/// A representation of an Elasticsearch `number`.
/// 
/// Note: this type is designed for better typing around `null_value`, 
/// and doesn't deserialise.
#[derive(Debug, Clone, Copy)]
pub enum Number {
	/// A signed 32-bit integer.
	Integer(i32),
	/// A signed 64-bit integer.
	Long(i64),
	/// A signed 16-bit integer.
	Short(i16),
	/// A signed 8-bit integer.
	Byte(i8),
	/// A single-precision 32-bit IEEE 754 floating point.
	Float(f32),
	/// A double-precision 64-bit IEEE 754 floating point.
	Double(f64)
}

impl Default for Number {
	fn default() -> Self {
		Number::Integer(0)
	}
}

impl Serialize for Number {
fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
where S: serde::Serializer {
		match *self {
			Number::Integer(num) => serializer.serialize_i32(num),
			Number::Long(num) => serializer.serialize_i64(num),
			Number::Short(num) => serializer.serialize_i16(num),
			Number::Byte(num) => serializer.serialize_i8(num),
			Number::Float(num) => serializer.serialize_f32(num),
			Number::Double(num) => serializer.serialize_f64(num)
		}
	}
}
impl Deserialize for Number {
	fn deserialize<D>(_: &mut D) -> Result<Number, D::Error>
	where D: serde::Deserializer {
		Ok(Number::Integer(0))
	}
}

/// A Rust representation of an Elasticsearch `number`.
pub trait ElasticNumberType<T: ElasticTypeMapping<()> + ElasticNumberMapping> 
where Self: Sized + ElasticType<T, ()> + Default + Clone { }

/// Visitor for a `number` field mapping.
#[derive(Debug, PartialEq, Default)]
pub struct ElasticNumberMappingVisitor<M: ElasticNumberMapping> {
	phantom: PhantomData<M>
}

impl <M: ElasticNumberMapping> serde::ser::MapVisitor for ElasticNumberMappingVisitor<M> {
	fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
	where S: Serializer {
		try!(serializer.serialize_struct_elt("type", M::data_type()));

		Ok(None)
	}
}

macro_rules! impl_mapping {
	($t:ty, $es_ty:expr) => (
		impl ElasticNumberMapping for $t { }
		impl_number_mapping!($t, $es_ty);
	)
}

/// Default mapping for an `integer` type.
#[derive(Debug, Default, Clone, Copy)]
pub struct DefaultIntegerMapping;
impl_mapping!(DefaultIntegerMapping, "integer");
impl ElasticType<DefaultIntegerMapping, ()> for i32 { }
impl Into<Number> for i32 {
	fn into(self) -> Number {
		Number::Integer(self)
	}
}

/// Default mapping for a `long` type.
#[derive(Debug, Default, Clone, Copy)]
pub struct DefaultLongMapping;
impl_mapping!(DefaultLongMapping, "long");
impl ElasticType<DefaultLongMapping, ()> for i64 { }
impl Into<Number> for i64 {
	fn into(self) -> Number {
		Number::Long(self)
	}
}

impl ElasticType<DefaultLongMapping, ()> for isize { }
impl Into<Number> for isize {
	fn into(self) -> Number {
		Number::Long(self as i64)
	}
}

/// Default mapping for a `short` type.
#[derive(Debug, Default, Clone, Copy)]
pub struct DefaultShortMapping;
impl_mapping!(DefaultShortMapping, "short");
impl ElasticType<DefaultShortMapping, ()> for i16 { }
impl Into<Number> for i16 {
	fn into(self) -> Number {
		Number::Short(self)
	}
}

/// Default mapping for a `byte` type.
#[derive(Debug, Default, Clone, Copy)]
pub struct DefaultByteMapping;
impl_mapping!(DefaultByteMapping, "byte");
impl ElasticType<DefaultByteMapping, ()> for i8 { }
impl Into<Number> for i8 {
	fn into(self) -> Number {
		Number::Byte(self)
	}
}

/// Default mapping for a `float` type.
#[derive(Debug, Default, Clone, Copy)]
pub struct DefaultFloatMapping;
impl_mapping!(DefaultFloatMapping, "float");
impl ElasticType<DefaultFloatMapping, ()> for f32 { }
impl Into<Number> for f32 {
	fn into(self) -> Number {
		Number::Float(self)
	}
}

/// Default mapping for a `doub.e` type.
#[derive(Debug, Default, Clone, Copy)]
pub struct DefaultDoubleMapping;
impl_mapping!(DefaultDoubleMapping, "double");
impl ElasticType<DefaultDoubleMapping, ()> for f64 { }
impl Into<Number> for f64 {
	fn into(self) -> Number {
		Number::Double(self)
	}
}
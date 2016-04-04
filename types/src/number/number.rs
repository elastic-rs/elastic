use std::marker::PhantomData;
use serde::{ Serialize, Deserialize, Serializer, Deserializer };
use super::mapping::{ ElasticNumberType, ElasticNumberMapping, Number };
use ::mapping::{ ElasticType, ElasticTypeMapping };

/// An Elasticsearch `number` with a mapping.
/// 
/// Where you don't need a custom mapping, you can use standard library primitive numbers instead.
/// 
/// # Examples
/// 
/// Defining a number with a mapping:
/// 
/// ```
/// # extern crate serde;
/// # extern crate elastic_types;
/// # fn main() {
/// # use elastic_types::mapping::prelude::*;
/// use elastic_types::number::prelude::*;
/// # #[derive(Debug, Clone, Default)]
/// # pub struct MyNumberMapping;
/// # impl ElasticNumberMapping for MyNumberMapping {
/// # 	fn null_value() -> Option<Number> {
/// # 		Some(Number::Integer(42))
/// # 	}
/// # }
/// # impl ElasticTypeMapping<()> for MyNumberMapping {
/// # 	type Visitor = ElasticNumberMappingVisitor<MyNumberMapping>;
/// # 	fn data_type() -> &'static str {
/// # 		"integer"
/// # 	}
/// # }
/// # impl serde::Serialize for MyNumberMapping {
/// # 	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
/// # 	where S: serde::Serializer {
/// # 		serializer.serialize_struct("mapping", Self::get_visitor())
/// # 	}
/// # }
/// let num: ElasticNumber<i32, MyNumberMapping> = ElasticNumber::new(42);
/// # }
/// ```
#[derive(Debug, Default, Clone)]
pub struct ElasticNumber<T: Into<Number> + Default + Clone + Serialize + Deserialize, M: ElasticTypeMapping<()> + ElasticNumberMapping> {
	value: T,
	phantom: PhantomData<M>
}
impl <T: Into<Number> + Default + Clone + Serialize + Deserialize, M: ElasticTypeMapping<()> + ElasticNumberMapping> ElasticNumber<T, M> {
	/// Creates a new `ElasticNumber` with the given mapping.
	/// 
	/// # Examples
	/// 
	/// Create a new `ElasticNumber` from an `i32`:
	pub fn new<I: Into<T>>(num: I) -> ElasticNumber<T, M> {
		ElasticNumber {
			value: num.into(),
			phantom: PhantomData
		}
	}

	/// Get the value of the number.
	pub fn get(&self) -> &T {
		&self.value
	}

	/// Set the value of the number.
	pub fn set<I: Into<T>>(&mut self, num: I) {
		self.value = num.into()
	}

	/// Change the mapping of this number.
	/// 
	/// Note, the number type must be the same.
	/// 
	/// # Examples
	/// 
	/// Change the mapping for a given `ElasticNumber`:
	/// 
	/// ```
	/// # extern crate serde;
	/// # extern crate elastic_types;
	/// # fn main() {
	/// # use elastic_types::mapping::prelude::*;
	/// # use elastic_types::number::prelude::*;
	/// # #[derive(Debug, Clone, Default)]
	/// # pub struct MyNumberMapping;
	/// # impl ElasticNumberMapping for MyNumberMapping {
	/// # }
	/// # impl ElasticTypeMapping<()> for MyNumberMapping {
	/// # 	type Visitor = ElasticNumberMappingVisitor<MyNumberMapping>;
	/// # 	fn data_type() -> &'static str {
	/// # 		"integer"
	/// # 	}
	/// # }
	/// # impl serde::Serialize for MyNumberMapping {
	/// # 	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
	/// # 	where S: serde::Serializer {
	/// # 		unimplemented!()
	/// # 	}
	/// # }
	/// # #[derive(Debug, Clone, Default)]
	/// # pub struct MyOtherNumberMapping;
	/// # impl ElasticNumberMapping for MyOtherNumberMapping {
	/// # }
	/// # impl ElasticTypeMapping<()> for MyOtherNumberMapping {
	/// # 	type Visitor = ElasticNumberMappingVisitor<MyOtherNumberMapping>;
	/// # 	fn data_type() -> &'static str {
	/// # 		"integer"
	/// # 	}
	/// # }
	/// # impl serde::Serialize for MyOtherNumberMapping {
	/// # 	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
	/// # 	where S: serde::Serializer {
	/// # 		unimplemented!()
	/// # 	}
	/// # }
	/// let number1 = ElasticNumber::<i32, MyNumberMapping>::new(42);
	/// let number2: ElasticNumber<i32, MyOtherNumberMapping> = number1.into();
	/// # }
	/// ```
	pub fn into<MInto: ElasticTypeMapping<()> + ElasticNumberMapping>(self) -> ElasticNumber<T, MInto> {
		ElasticNumber::<T, MInto>::new(self.value)
	}
}

impl <T: Into<Number> + Default + Clone + Serialize + Deserialize, M: ElasticTypeMapping<()> + ElasticNumberMapping> ElasticType<M, ()> for ElasticNumber<T, M> { }
impl <T: Into<Number> + Default + Clone + Serialize + Deserialize, M: ElasticTypeMapping<()> + ElasticNumberMapping> ElasticNumberType<M> for ElasticNumber<T, M> { }

impl <T: Into<Number> + Default + Clone + Serialize + Deserialize, M: ElasticTypeMapping<()> + ElasticNumberMapping> From<T> for ElasticNumber<T, M> {
	fn from(num: T) -> Self {
		ElasticNumber::<T, M>::new(num)
	}
}

//Serialize elastic number
impl <T: Into<Number> + Default + Clone + Serialize + Deserialize, M: ElasticTypeMapping<()> + ElasticNumberMapping> Serialize for ElasticNumber<T, M> {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where S: Serializer {
		self.value.serialize(serializer)
	}
}

//Deserialize elastic number
impl <T: Into<Number> + Default + Clone + Serialize + Deserialize, M: ElasticTypeMapping<()> + ElasticNumberMapping> Deserialize for ElasticNumber<T, M> {
	fn deserialize<D>(deserializer: &mut D) -> Result<ElasticNumber<T, M>, D::Error> where D: Deserializer {
		let t = try!(T::deserialize(deserializer));

		Ok(ElasticNumber::<T, M>::new(t))
	}
}
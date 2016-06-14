//! Mapping compatibility between `elastic_types` and `rs-es`.
//!
//! `rs-es` uses a `HashMap` structure to build mappings.
//! The current design limits the depth of the structure to a single layer, which works in most
//! cases, but means you'll lose information when mapping nested types.
//!
//! The hashmap is built using a `serde::Serializer`, which holds the source for the borrowed
//! `HashMap<&str, HashMap<&str, &str>>` result.
//! This means you need to keep the `Serializer` around for as long as you need to access the
//! data for `rs-es`.

use std::marker::PhantomData;
use std::fmt;
use std::error::Error as StdError;
use std::collections::HashMap;
use serde;
use serde::Serializer as SerSerializer;
use serde::ser::Error as SerError;
use ::mapping::ElasticTypeVisitor;
use ::object::{ ElasticUserTypeMapping, ElasticUserTypeMappingVisitor };

//TODO: This currently only supports mapping structs 1 level deep. This means stuff can get lost.

/// Helper for mapping `elastic_type`s with `rs-es`.
pub struct RsesMapper<M, V> where
M: ElasticUserTypeMapping<Visitor = ElasticUserTypeMappingVisitor<V>>,
V: ElasticTypeVisitor {
    phantom_m: PhantomData<M>,
    phantom_v: PhantomData<V>
}

impl <M, V> RsesMapper<M, V> where
M: ElasticUserTypeMapping<Visitor = ElasticUserTypeMappingVisitor<V>>,
V: ElasticTypeVisitor {
    /// Map a user-defined type to a `HashMap`.
	///
    /// # Examples
	///
	/// ```no_run
	/// # #![feature(plugin, custom_derive)]
	/// # #![plugin(json_str, elastic_types_macros)]
	/// # #[macro_use]
	/// # extern crate elastic_types;
	/// # extern crate serde;
	/// # use serde::{ Serialize, Deserialize };
	/// use elastic_types::mapping::prelude::*;
    ///
	/// # use elastic_types::date::prelude::*;
	/// # #[derive(Default, Clone, Serialize, Deserialize, ElasticType)]
	/// # pub struct MyType {
	/// # 	pub my_date: ElasticDate<DefaultDateFormat>,
	/// # 	pub my_string: String,
	/// # 	pub my_num: i32
	/// # }
	/// # impl serde::Serialize for MyType {
	/// # 	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where S: serde::Serializer {
	/// # 		unimplemented!()
	/// # 	}
	/// # }
	/// # impl serde::Deserialize for MyType {
	/// # 	 fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error> where D: serde::Deserializer {
	/// # 		unimplemented!()
	/// # 	}
	/// # }
	/// # fn main() {
	/// let mut writer = RsesSerializer::default();
	/// let mut ser = RsesMapper::to_value(MyTypeMapping, &mut writer);
    ///
    /// //Map `ser` with `rs_es::Client`
	/// # }
	/// ```
    pub fn to_value(_: M, serializer: &mut Serializer) -> Result<HashMap<&str, HashMap<&str, &str>>, Error> {
        try!(serializer.serialize_struct("", V::new()));

        Ok(serializer.value.result())
    }
}

/// A representation of an `rs-es` mapping.
pub struct Mapping {
	interned: Vec<String>,
	map: HashMap<usize, HashMap<usize, usize>>
}

impl Default for Mapping {
	fn default() -> Mapping {
		Mapping {
			interned: Vec::new(),
			map: HashMap::new()
		}
	}
}

impl Mapping {
    /// Intern a `string` value and return its unique index.
	pub fn intern<I>(&mut self, string: I) -> usize where
    I: Into<String> {
		self.interned.push(string.into());
		self.interned.len() - 1
	}

    /// Get the value of an interned `string`.
    pub fn get(&self, i: usize) -> &str {
        &self.interned[i]
    }

    /// Build an `rs-es` mapping structure from the internal state.
	pub fn result(&self) -> HashMap<&str, HashMap<&str, &str>> {
		let mut map = HashMap::new();

        for (k, v) in &self.map {
            let mut props = HashMap::new();

            for (pk, pv) in v {
                props.insert(self.get(*pk), self.get(*pv));
            }

            map.insert(self.get(*k), props);
        }

        map
	}
}

/// A `serde` serialiser for `rs-es` mappings.
///
/// This serialiser maintains an internal state that's really only valid for a single serialised value.
/// It's best to use a single `Serializer` per mapping you want to produce, but it must be kept around
/// for as long as you want to access its `value`.
#[derive(Default)]
pub struct Serializer {
    /// The result of serialising a mapping `struct`.
	pub value: Mapping,
    state: State
}

enum State {
    Root,
    Property(usize),
    PropertyValue(usize, usize)
}

impl Default for State {
    fn default() -> State {
        State::Root
    }
}

impl State {
    pub fn root(&mut self) {
        *self = State::Root;
    }

    pub fn property(&mut self, index: usize) {
        *self = State::Property(index);
    }

    pub fn value(&mut self, prop_index: usize, val_index: usize) {
        *self = State::PropertyValue(prop_index, val_index);
    }
}

impl Serializer {
    fn set(&mut self, interned: usize) -> Result<(), <Self as SerSerializer>::Error> {
        match self.state {
            State::PropertyValue(i, v) => {
                if let Some(prop) = self.value.map.get_mut(&i) {
                    prop.insert(v, interned);

                    self.state.property(i);

                    Ok(())
                }
                else {
                    Err(Error(format!("Property name lookup {} failed for value {}", i, interned)))
                }
            },
            _ => Ok(())
        }
    }
}

impl serde::Serializer for Serializer {
	type Error = Error;

    fn serialize_bool(&mut self, value: bool) -> Result<(), Self::Error> {
        let vi = self.value.intern(
            if value {
                "true"
            }
            else {
                "false"
            }
        );

        self.set(vi)
	}

    fn serialize_i8(&mut self, value: i8) -> Result<(), Self::Error> {
        let vi = self.value.intern(value.to_string());

        self.set(vi)
	}

    fn serialize_i16(&mut self, value: i16) -> Result<(), Self::Error> {
        let vi = self.value.intern(value.to_string());

        self.set(vi)
	}

    fn serialize_i32(&mut self, value: i32) -> Result<(), Self::Error> {
        let vi = self.value.intern(value.to_string());

        self.set(vi)
	}

    fn serialize_i64(&mut self, value: i64) -> Result<(), Self::Error> {
        let vi = self.value.intern(value.to_string());

        self.set(vi)
	}

    fn serialize_u8(&mut self, value: u8) -> Result<(), Self::Error> {
        let vi = self.value.intern(value.to_string());

        self.set(vi)
	}

    fn serialize_u16(&mut self, value: u16) -> Result<(), Self::Error> {
        let vi = self.value.intern(value.to_string());

        self.set(vi)
	}

    fn serialize_u32(&mut self, value: u32) -> Result<(), Self::Error> {
        let vi = self.value.intern(value.to_string());

        self.set(vi)
	}

    fn serialize_u64(&mut self, value: u64) -> Result<(), Self::Error> {
        let vi = self.value.intern(value.to_string());

        self.set(vi)
	}

    fn serialize_f32(&mut self, value: f32) -> Result<(), Self::Error> {
        let vi = self.value.intern(value.to_string());

        self.set(vi)
	}

	fn serialize_f64(&mut self, value: f64) -> Result<(), Self::Error> {
        let vi = self.value.intern(value.to_string());

        self.set(vi)
	}

    fn serialize_str(&mut self, value: &str) -> Result<(), Self::Error> {
        let vi = self.value.intern(value.to_string());

        match self.state {
            State::Root => {
        		self.value.map.insert(vi, HashMap::new());
                self.state.property(vi);

                Ok(())
            }
            State::Property(i) => {
                self.state.value(i, vi);

                Ok(())
            }
            State::PropertyValue(_, _) => {
                self.set(vi)
            }
        }
	}

    fn serialize_unit(&mut self) -> Result<(), Self::Error> {
		Ok(())
	}

    fn serialize_none(&mut self) -> Result<(), Self::Error> {
		Ok(())
	}

    fn serialize_some<V>(&mut self, value: V) -> Result<(), Self::Error> where
	V: serde::Serialize {
        value.serialize(self)
	}

    fn serialize_seq<V>(&mut self, _: V) -> Result<(), Self::Error> where
	V: serde::ser::SeqVisitor {
		Ok(())
	}

    fn serialize_seq_elt<T>(&mut self, _: T) -> Result<(), Self::Error> where
	T: serde::Serialize {
		Ok(())
	}

    fn serialize_map<V>(&mut self, visitor: V) -> Result<(), Self::Error> where
	V: serde::ser::MapVisitor {
        match self.state {
            State::Root => {
                let mut visitor = visitor;
                while let Some(()) = try!(visitor.visit(self).map_err(|e| Self::Error::custom(e.description()))) { }

                Ok(())
            },
            State::Property(_) => {
                let mut visitor = visitor;
                while let Some(()) = try!(visitor.visit(self).map_err(|e| Self::Error::custom(e.description()))) { }

                self.state.root();

                Ok(())
            },
            _ => Ok(())
        }
	}

    fn serialize_map_elt<K, V>(&mut self, key: K, value: V) -> Result<(), Self::Error> where
	K: serde::Serialize,
	V: serde::Serialize {
        try!(key.serialize(self).map_err(|e| Self::Error::custom(e.description())));
        value.serialize(self).map_err(|e| Self::Error::custom(e.description()))
	}
}

/// An error encountered while serialising a mapping.
#[derive(Debug)]
pub struct Error(String);

impl StdError for Error {
    fn description(&self) -> &str {
        &self.0
    }

    fn cause(&self) -> Option<&StdError> {
        None
    }

}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.0)
    }
}

impl SerError for Error {
    fn custom<T: Into<String>>(msg: T) -> Error {
        Error(msg.into())
    }
}

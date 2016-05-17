macro_rules! impl_mapping {
	($($t:ty),*) => (
		$(
			impl $crate::mapping::ElasticType<$crate::mapping::NullMapping, ()> for $t { }
		)*
	)
}

macro_rules! impl_string_mapping {
	($t:ty) => (
		impl $crate::mapping::ElasticFieldMapping<()> for $t {
			type Visitor = $crate::string::mapping::ElasticStringMappingVisitor<$t>;
			type MultiFieldMapping = Self;

			fn data_type() -> &'static str {
				$crate::string::mapping::STRING_DATATYPE
			}
		}

		impl serde::Serialize for $t {
			fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
			where S: serde::Serializer {
				serializer.serialize_struct("mapping", Self::get_visitor())
			}
		}
	)
}

macro_rules! impl_boolean_mapping {
	($t:ty) => (
		impl $crate::mapping::ElasticFieldMapping<()> for $t {
			type Visitor = $crate::boolean::mapping::ElasticBooleanMappingVisitor<$t>;
			type MultiFieldMapping = Self;

			fn data_type() -> &'static str {
				$crate::boolean::mapping::BOOLEAN_DATATYPE
			}
		}

		impl serde::Serialize for $t {
			fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
			where S: serde::Serializer {
				serializer.serialize_struct("mapping", Self::get_visitor())
			}
		}
	)
}

macro_rules! impl_number_mapping {
	($t:ty, $v:ident, $es_ty:expr) => (
		impl $crate::mapping::ElasticFieldMapping<()> for $t {
			type Visitor = $v;
			type MultiFieldMapping = Self;

			fn data_type() -> &'static str {
				$es_ty
			}
		}

		impl serde::Serialize for $t {
			fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
			where S: serde::Serializer {
				serializer.serialize_struct("mapping", Self::get_visitor())
			}
		}
	)
}

macro_rules! impl_integer_mapping {
	($t:ty) => (
		impl $crate::mapping::ElasticFieldMapping<()> for $t {
			type Visitor = $crate::number::mapping::ElasticIntegerMappingVisitor<$t>;
			type MultiFieldMapping = Self;

			fn data_type() -> &'static str {
				$crate::number::mapping::INTEGER_DATATYPE
			}
		}

		impl serde::Serialize for $t {
			fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
			where S: serde::Serializer {
				serializer.serialize_struct("mapping", Self::get_visitor())
			}
		}
	)
}

macro_rules! impl_long_mapping {
	($t:ty) => (
		impl $crate::mapping::ElasticFieldMapping<()> for $t {
			type Visitor = $crate::number::mapping::ElasticLongMappingVisitor<$t>;
			type MultiFieldMapping = Self;

			fn data_type() -> &'static str {
				$crate::number::mapping::LONG_DATATYPE
			}
		}

		impl serde::Serialize for $t {
			fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
			where S: serde::Serializer {
				serializer.serialize_struct("mapping", Self::get_visitor())
			}
		}
	)
}

macro_rules! impl_short_mapping {
	($t:ty) => (
		impl $crate::mapping::ElasticFieldMapping<()> for $t {
			type Visitor = $crate::number::mapping::ElasticShortMappingVisitor<$t>;
			type MultiFieldMapping = Self;

			fn data_type() -> &'static str {
				$crate::number::mapping::SHORT_DATATYPE
			}
		}

		impl serde::Serialize for $t {
			fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
			where S: serde::Serializer {
				serializer.serialize_struct("mapping", Self::get_visitor())
			}
		}
	)
}

macro_rules! impl_byte_mapping {
	($t:ty) => (
		impl $crate::mapping::ElasticFieldMapping<()> for $t {
			type Visitor = $crate::number::mapping::ElasticByteMappingVisitor<$t>;
			type MultiFieldMapping = Self;

			fn data_type() -> &'static str {
				$crate::number::mapping::BYTE_DATATYPE
			}
		}

		impl serde::Serialize for $t {
			fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
			where S: serde::Serializer {
				serializer.serialize_struct("mapping", Self::get_visitor())
			}
		}
	)
}

macro_rules! impl_float_mapping {
	($t:ty) => (
		impl $crate::mapping::ElasticFieldMapping<()> for $t {
			type Visitor = $crate::number::mapping::ElasticFloatMappingVisitor<$t>;
			type MultiFieldMapping = Self;

			fn data_type() -> &'static str {
				$crate::number::mapping::FLOAT_DATATYPE
			}
		}

		impl serde::Serialize for $t {
			fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
			where S: serde::Serializer {
				serializer.serialize_struct("mapping", Self::get_visitor())
			}
		}
	)
}

macro_rules! impl_double_mapping {
	($t:ty) => (
		impl $crate::mapping::ElasticFieldMapping<()> for $t {
			type Visitor = $crate::number::mapping::ElasticDoubleMappingVisitor<$t>;
			type MultiFieldMapping = Self;

			fn data_type() -> &'static str {
				$crate::number::mapping::DOUBLE_DATATYPE
			}
		}

		impl serde::Serialize for $t {
			fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
			where S: serde::Serializer {
				serializer.serialize_struct("mapping", Self::get_visitor())
			}
		}
	)
}

#[cfg(any(feature = "date-ty", feature = "date-ty-nightly"))]
macro_rules! impl_date_mapping {
	($t:ty, $f:ty) => (
		impl $crate::mapping::ElasticFieldMapping<$f> for $t {
			type Visitor = $crate::date::mapping::ElasticDateMappingVisitor<$f, $t>;
			type MultiFieldMapping = Self;

			fn data_type() -> &'static str {
				$crate::date::mapping::DATE_DATATYPE
			}
		}

		impl serde::Serialize for $t {
			fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
			where S: serde::Serializer {
				serializer.serialize_struct("mapping", Self::get_visitor())
			}
		}
	);
	($t:ty) => (
		impl <T: $crate::date::DateFormat> $crate::mapping::ElasticFieldMapping<T> for $t {
			type Visitor = $crate::date::mapping::ElasticDateMappingVisitor<T, $t>;
			type MultiFieldMapping = Self;

			fn data_type() -> &'static str {
				$crate::date::mapping::DATE_DATATYPE
			}
		}

		impl <T: $crate::date::DateFormat> serde::Serialize for $t {
			fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
			where S: serde::Serializer {
				serializer.serialize_struct("mapping", Self::get_visitor())
			}
		}
	)
}

//TODO: Remove need to iterate over this. Requires updating date_fmt to return vec![..] instead of [..]
//TODO: See if we can return a borrowed &[Item] instead of owned Vec<Item>, needs to work on stable too
#[cfg(any(feature = "date-ty", feature = "date-ty-nightly"))]
macro_rules! impl_date_fmt {
	($t:ty, $f:tt, $n:expr) => (
		impl $crate::date::DateFormat for $t {
			fn fmt<'a>() -> Vec<chrono::format::Item<'a>> {
				date_fmt!($f)
					.iter()
					.cloned()
					.collect()
			}

			fn name() -> &'static str { $n }
		}
	)
}

#[cfg(any(feature = "geo-ty", feature = "geo-ty-nightly"))]
macro_rules! impl_geo_point_mapping {
	($t:ty, $f:ty) => (
		impl $crate::mapping::ElasticFieldMapping<$f> for $t {
			type Visitor = $crate::geo::point::mapping::ElasticGeoPointMappingVisitor<$f, $t>;
			type MultiFieldMapping = Self;

			fn data_type() -> &'static str {
				$crate::geo::point::mapping::GEOPOINT_TYPE
			}
		}

		impl serde::Serialize for $t {
			fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
			where S: serde::Serializer {
				serializer.serialize_struct("mapping", Self::get_visitor())
			}
		}
	);
	($t:ty) => (
		impl <T: $crate::geo::point::GeoPointFormat> $crate::mapping::ElasticFieldMapping<T> for $t {
			type Visitor = $crate::geo::point::mapping::ElasticGeoPointMappingVisitor<T, $t>;
			type MultiFieldMapping = Self;

			fn data_type() -> &'static str {
				$crate::geo::point::mapping::GEOPOINT_TYPE
			}
		}

		impl <T: $crate::geo::point::GeoPointFormat> serde::Serialize for $t {
			fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
			where S: serde::Serializer {
				serializer.serialize_struct("mapping", Self::get_visitor())
			}
		}
	)
}

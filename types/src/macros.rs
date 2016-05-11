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
				"string"
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
				"boolean"
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
			type Visitor = ElasticIntegerMappingVisitor<$t>;
			type MultiFieldMapping = Self;

			fn data_type() -> &'static str {
				"integer"
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
			type Visitor = ElasticLongMappingVisitor<$t>;
			type MultiFieldMapping = Self;

			fn data_type() -> &'static str {
				"long"
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
			type Visitor = ElasticShortMappingVisitor<$t>;
			type MultiFieldMapping = Self;

			fn data_type() -> &'static str {
				"short"
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
			type Visitor = ElasticByteMappingVisitor<$t>;
			type MultiFieldMapping = Self;

			fn data_type() -> &'static str {
				"byte"
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
			type Visitor = ElasticFloatMappingVisitor<$t>;
			type MultiFieldMapping = Self;

			fn data_type() -> &'static str {
				"float"
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
			type Visitor = ElasticDoubleMappingVisitor<$t>;
			type MultiFieldMapping = Self;

			fn data_type() -> &'static str {
				"double"
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

macro_rules! impl_date_mapping {
	($t:ty, $f:ty) => (
		impl $crate::mapping::ElasticFieldMapping<$f> for $t {
			type Visitor = $crate::date::mapping::ElasticDateMappingVisitor<$f, $t>;
			type MultiFieldMapping = Self;

			fn data_type() -> &'static str {
				"date"
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
				"date"
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

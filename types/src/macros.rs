macro_rules! impl_mapping {
	($($t:ty),*) => (
		$(
			impl $crate::mapping::ElasticType<$crate::mapping::DefaultMapping, ()> for $t { }
		)*
	)
}

macro_rules! impl_text_mapping {
	($t:ty) => (
		impl $crate::mapping::ElasticFieldMapping<()> for $t {
			type Visitor = $crate::string::mapping::ElasticTextMappingVisitor<$t>;

			fn data_type() -> &'static str {
				$crate::string::mapping::TEXT_DATATYPE
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

macro_rules! impl_keyword_mapping {
	($t:ty) => (
		impl $crate::mapping::ElasticFieldMapping<()> for $t {
			type Visitor = $crate::string::mapping::ElasticKeywordMappingVisitor<$t>;

			fn data_type() -> &'static str {
				$crate::string::mapping::KEYWORD_DATATYPE
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
			type Visitor = $crate::number::mapping::IntegerMappingVisitor<$t>;

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
			type Visitor = $crate::number::mapping::LongMappingVisitor<$t>;

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
			type Visitor = $crate::number::mapping::ShortMappingVisitor<$t>;

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
			type Visitor = $crate::number::mapping::ByteMappingVisitor<$t>;

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
			type Visitor = $crate::number::mapping::FloatMappingVisitor<$t>;

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
			type Visitor = $crate::number::mapping::DoubleMappingVisitor<$t>;

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

macro_rules! impl_date_mapping {
	($t:ty, $f:ty) => (
		impl $crate::mapping::ElasticFieldMapping<$f> for $t {
			type Visitor = $crate::date::mapping::DateMappingVisitor<$f, $t>;

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
		impl <F: $crate::date::DateFormat> $crate::mapping::ElasticFieldMapping<F> for $t {
			type Visitor = $crate::date::mapping::DateMappingVisitor<F, $t>;

			fn data_type() -> &'static str {
				$crate::date::mapping::DATE_DATATYPE
			}
		}

		impl <F: $crate::date::DateFormat> serde::Serialize for $t {
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
			}

			fn name() -> &'static str { $n }
		}
	)
}

macro_rules! impl_ip_mapping {
	($t:ty) => (
		impl $crate::mapping::ElasticFieldMapping<()> for $t {
			type Visitor = $crate::ip::mapping::IpMappingVisitor<$t>;

			fn data_type() -> &'static str {
				$crate::ip::mapping::IP_DATATYPE
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

macro_rules! impl_geo_point_mapping {
	($t:ty, $f:ty) => (
		impl $crate::mapping::ElasticFieldMapping<$f> for $t {
			type Visitor = $crate::geo::point::mapping::GeoPointMappingVisitor<$f, $t>;

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
		impl <F: $crate::geo::point::GeoPointFormat> $crate::mapping::ElasticFieldMapping<F> for $t {
			type Visitor = $crate::geo::point::mapping::GeoPointMappingVisitor<F, $t>;

			fn data_type() -> &'static str {
				$crate::geo::point::mapping::GEOPOINT_DATATYPE
			}
		}

		impl <F: $crate::geo::point::GeoPointFormat> serde::Serialize for $t {
			fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
			where S: serde::Serializer {
				serializer.serialize_struct("mapping", Self::get_visitor())
			}
		}
	)
}

macro_rules! impl_geo_shape_mapping {
	($t:ty) => (
		impl $crate::mapping::ElasticFieldMapping<()> for $t {
			type Visitor = $crate::geo::shape::mapping::GeoShapeMappingVisitor<$t>;

			fn data_type() -> &'static str {
				$crate::geo::shape::mapping::GEOSHAPE_DATATYPE
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

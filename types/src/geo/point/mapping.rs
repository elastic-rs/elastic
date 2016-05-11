pub const GEOPOINT_TYPE: &'static str = "geo_point";

pub trait ElasticGeoPointMapping<T> where
T: DateFormat,
Self: ElasticFieldMapping<T> + Sized + Serialize {
    fn boost() -> Option<f32> {
		None
	}
}

#[derive(Debug, Default, Clone, Copy)]
pub struct DefaultGeoPointMapping<T> where
T: DateFormat {
	phantom: PhantomData<T>
}
impl <T> ElasticGeoPointMapping<T> for DefaultGeoPointMapping<T> where
T: DateFormat { }

impl_geopoint_mapping!(DefaultGeoPointMapping<T>);

/// Visitor for a `date` map.
#[derive(Debug, PartialEq)]
pub struct ElasticGeoPointMappingVisitor<F, T> where
F: DateFormat,
T: ElasticDateMapping<F> {
	phantom_f: PhantomData<F>,
	phantom_t: PhantomData<T>
}

impl <F, T> ElasticTypeVisitor for ElasticGeoPointMappingVisitor<F, T> where
F: DateFormat,
T: ElasticGeoPointMapping<F> {
	fn new() -> Self {
		ElasticGeoPointMappingVisitor {
			phantom_f: PhantomData,
			phantom_t: PhantomData
		}
	}
}
impl <F, T> serde::ser::MapVisitor for ElasticGeoPointMappingVisitor<F, T>  where
F: DateFormat,
T: ElasticGeoPointMapping<F> {
	fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
	where S: Serializer {
		try!(serializer.serialize_struct_elt("type", T::data_type()));

		if let Some(boost) = T::boost() {
			try!(serializer.serialize_struct_elt("boost", boost));
		};

		Ok(None)
	}
}

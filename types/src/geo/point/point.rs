pub struct ElasticGeoPoint<F, T = DefaultGeoPointMapping<F>> where
F: GeoPointFormat,
T: ElasticFieldMapping<F> + ElasticGeoPointMapping<F> {
    pub lon: f64,
    pub lat: f64,
	phantom_f: PhantomData<F>,
	phantom_t: PhantomData<T>
}

impl <F, T> ElasticGeoPoint where
F: GeoPointFormat,
T: ElasticFieldMapping<F> + ElasticGeoPointMapping<F> {
    pub fn new(lon: f64, lat: f64) -> ElasticGeoPoint<F, T> {
        ElasticFieldMapping {
            lon: lon,
            lat: lat
        }
    }

	pub fn into<FInto, TInto>(self) -> ElasticDate<FInto, TInto> where
	FInto: DateFormat,
	TInto: ElasticFieldMapping<FInto> + ElasticDateMapping<FInto> {
		ElasticDate::<FInto, TInto>::new(self.value)
	}
}

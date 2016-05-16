pub struct ElasticGeoPoint<F, T = DefaultGeoPointMapping<F>> where
F: GeoPointFormat,
T: ElasticFieldMapping<F> + ElasticGeoPointMapping<F> {
    pub value: Point,
	phantom_f: PhantomData<F>,
	phantom_t: PhantomData<T>
}

impl <F, T> ElasticGeoPoint where
F: GeoPointFormat,
T: ElasticFieldMapping<F> + ElasticGeoPointMapping<F> {
    pub fn new(point: Coordinate) -> ElasticGeoPoint<F, T> {
        ElasticGeoPoint {
            value: Point(point),
            phantom_f: PhantomData,
            phantom_t: PhantomData
        }
    }

	pub fn into<FInto, TInto>(self) -> ElasticDate<FInto, TInto> where
	FInto: DateFormat,
	TInto: ElasticFieldMapping<FInto> + ElasticDateMapping<FInto> {
		ElasticDate::<FInto, TInto>::new(self.value)
	}
}

//TODO: impl default for geo::Point

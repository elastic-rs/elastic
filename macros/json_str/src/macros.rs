#[macro_export]
macro_rules! js {
	($j:tt) => ({
		let json_raw = stringify!($j);
		let mut json = String::with_capacity(json_raw.len());
		$crate::parse::sanitise(json_raw.as_bytes(), &mut json);

		json
	})
}

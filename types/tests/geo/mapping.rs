#![cfg(feature = "nightly", feature(custom_derive, custom_attribute, plugin))]
#![cfg(feature = "nightly", plugin(serde_macros, json_str, elastic_types_macros, elastic_date_macros))]

#[cfg_attr(feature = "nightly", allow(plugin_as_library))]
#[macro_use]
extern crate json_str;
#[cfg_attr(feature = "nightly", allow(plugin_as_library))]
#[macro_use]
extern crate elastic_date_macros;

#[test]
fn serialise_mapping_default() {
	panic!("implement")
}

#[test]
fn serialise_mapping_custom() {
	panic!("implement")
}

#![feature(test, plugin, custom_derive)]
#![cfg_attr(feature = "nightly", plugin(json_str))]

#[cfg_attr(feature = "nightly", allow(plugin_as_library))]
#[macro_use]
extern crate json_str;

extern crate test;

use test::Bencher;

#[bench]
fn parse_plain_json_str_sml(b: &mut Bencher) {
	b.iter(|| {
		json_str!({
			query: {
				filtered: {
					query: {
						match_all: {}
					},
					filter: {
						geo_distance: {
							distance: "20km",
							location: {
								lat: 37.776,
								lon: -122.41
							}
						}
					}
				}
			}
		})
	});
}

#[bench]
fn parse_plain_json_str_med(b: &mut Bencher) {
	b.iter(|| {
		json_str!({
			query: {
				filtered: {
					query: {
						filtered: {
							query: {
								match_all: {}
							},
							filter: {
								geo_distance: {
									distance: "20km",
									location: {
										lat: 37.776,
										lon: -122.41
									}
								}
							}
						}
					},
					filter: {
						geo_distance: {
							distance: "20km",
							location: {
								lat: 37.776,
								lon: -122.41
							}
						}
					}
				}
			}
		})
	});
}

#[bench]
fn parse_plain_json_str_lrg(b: &mut Bencher) {
	b.iter(|| {
		json_str!({
			query: {
				filtered: {
					query: {
						filtered: {
							query: {
								filtered: {
									query: {
										match_all: {}
									},
									filter: {
										geo_distance: {
											distance: "20km",
											location: {
												lat: 37.776,
												lon: -122.41
											}
										}
									}
								}
							},
							filter: {
								geo_distance: {
									distance: "20km",
									location: {
										lat: 37.776,
										lon: -122.41
									}
								}
							}
						}
					},
					filter: {
						geo_distance: {
							distance: "20km",
							location: {
								lat: 37.776,
								lon: -122.41
							}
						}
					}
				}
			}
		})
	});
}

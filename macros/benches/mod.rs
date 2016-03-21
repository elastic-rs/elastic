#![feature(test, plugin)]
#![plugin(elastic_macros)]
#![plugin(serde_macros)]
#![plugin(json_macros)]

extern crate test;
extern crate serde;
extern crate serde_json;

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

#[bench]
fn parse_plain_json_value_sml(b: &mut Bencher) {
	b.iter(|| {
		serde_json::to_string(&json!({
			"query": {
				"filtered": {
					"query": {
						"match_all": {}
					},
					"filter": {
						"geo_distance": {
							"distance": "20km",
							"location": {
								"lat": 37.776,
								"lon": -122.41
							}
						}
					}
				}
			}
		}))
	});
}

#[bench]
fn parse_plain_json_value_med(b: &mut Bencher) {
	b.iter(|| {
		serde_json::to_string(&json!({
			"query": {
				"filtered": {
					"query": {
						"filtered": {
							"query": {
								"match_all": {}
							},
							"filter": {
								"geo_distance": {
									"distance": "20km",
									"location": {
										"lat": 37.776,
										"lon": -122.41
									}
								}
							}
						}
					},
					"filter": {
						"geo_distance": {
							"distance": "20km",
							"location": {
								"lat": 37.776,
								"lon": -122.41
							}
						}
					}
				}
			}
		}))
	});
}

#[bench]
fn parse_plain_json_value_lrg(b: &mut Bencher) {
	b.iter(|| {
		serde_json::to_string(&json!({
			"query": {
				"filtered": {
					"query": {
						"filtered": {
							"query": {
								"filtered": {
									"query": {
										"match_all": {}
									},
									"filter": {
										"geo_distance": {
											"distance": "20km",
											"location": {
												"lat": 37.776,
												"lon": -122.41
											}
										}
									}
								}
							},
							"filter": {
								"geo_distance": {
									"distance": "20km",
									"location": {
										"lat": 37.776,
										"lon": -122.41
									}
								}
							}
						}
					},
					"filter": {
						"geo_distance": {
							"distance": "20km",
							"location": {
								"lat": 37.776,
								"lon": -122.41
							}
						}
					}
				}
			}
		}))
	});
}
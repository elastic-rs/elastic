#![feature(test, plugin)]
#![plugin(elastic_types_codegen)]
#![plugin(serde_macros)]

extern crate test;
extern crate serde;
extern crate serde_json;

use test::Bencher;

#[bench]
fn parse_plain_json_sml(b: &mut Bencher) {
	b.iter(|| {
		json!({
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
fn parse_plain_json_med(b: &mut Bencher) {
	b.iter(|| {
		json!({
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
fn parse_plain_json_lrg(b: &mut Bencher) {
	b.iter(|| {
		json!({
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
fn parse_repl_json_sml(b: &mut Bencher) {
	let dist = "20km";
	let lat = 37.776;
	let lon = -122.41;

	let query = "query";
	let filtered = "filtered";
	let filter = "filter";

	b.iter(|| {
		json!(query, filtered, filter, dist, lat, lon, {
			$query: {
				$filtered: {
					$query: {
						match_all: {}
					},
					$filter: {
						geo_distance: {
							distance: $dist,
							location: {
								lat: $lat,
								lon: $lon
							}
						}
					}
				}
			}
		})
	});
}

#[bench]
fn parse_repl_json_med(b: &mut Bencher) {
	let dist = "20km";
	let lat = 37.776;
	let lon = -122.41;

	let query = "query";
	let filtered = "filtered";
	let filter = "filter";

	b.iter(|| {
		json!(query, filtered, filter, dist, lat, lon, {
			$query: {
				$filtered: {
					$query: {
						$filtered: {
							$filtered: {
								query: {
									match_all: {}
								},
								$filter: {
									geo_distance: {
										distance: $dist,
										location: {
											lat: $lat,
											lon: $lon
										}
									}
								}
							}
						}
					},
					$filter: {
						geo_distance: {
							distance: $dist,
							location: {
								lat: $lat,
								lon: $lon
							}
						}
					}
				}
			}
		})
	});
}

#[bench]
fn parse_repl_json_lrg(b: &mut Bencher) {
	let dist = "20km";
	let lat = 37.776;
	let lon = -122.41;

	let query = "query";
	let filtered = "filtered";
	let filter = "filter";

	b.iter(|| {
		json!(query, filtered, filter, dist, lat, lon, {
			$query: {
				$filtered: {
					$query: {
						$filtered: {
							$filtered: {
								$query: {
									$filtered: {
										$filtered: {
											$query: {
												match_all: {}
											},
											$filter: {
												geo_distance: {
													distance: $dist,
													location: {
														lat: $lat,
														lon: $lon
													}
												}
											}
										}
									}
								},
								$filter: {
									geo_distance: {
										distance: $dist,
										location: {
											lat: $lat,
											lon: $lon
										}
									}
								}
							}
						}
					},
					$filter: {
						geo_distance: {
							distance: $dist,
							location: {
								lat: $lat,
								lon: $lon
							}
						}
					}
				}
			}
		})
	});
}
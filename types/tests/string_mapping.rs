#[test]
fn serialise_mapping_default() {
	/*let mapping = DefaultDateMapping::<BasicDateTime>::new();
	let ser = serde_json::to_string(&mapping).unwrap();

	assert_eq!(r#"{"format":"basic_date_time"}"#, ser);*/
}

#[test]
fn serialise_mapping_custom() {
	/*let mapping = MyDateMapping;
	let ser = serde_json::to_string(&mapping).unwrap();

	assert_eq!(r#"{"boost":1.01,"doc_values":true,"include_in_all":false,"index":"no","store":true,"format":"epoch_millis","ignore_malformed":true,"null_value":"0","precision_step":6}"#, ser);*/
}

#[test]
fn serialise_mapping_field_data() {
	/*let index_opts: Vec<String> = vec![
		IndexAnalysis::Analyzed,
		IndexAnalysis::NotAnalyzed,
		IndexAnalysis::No
	]
	.iter()
	.map(|i| serde_json::to_string(i).unwrap())
	.collect();

	let expected_opts = vec![
		r#""analyzed""#,
		r#""not_analyzed""#,
		r#""no""#
	];

	let mut success = true;
	for i in 0..index_opts.len() {
		if expected_opts[i] != &index_opts[i] {
			success = false;
			break;
		}
	}

	assert!(success);*/
}

#[test]
fn serialise_mapping_field_data_loading() {
	/*let index_opts: Vec<String> = vec![
		IndexAnalysis::Analyzed,
		IndexAnalysis::NotAnalyzed,
		IndexAnalysis::No
	]
	.iter()
	.map(|i| serde_json::to_string(i).unwrap())
	.collect();

	let expected_opts = vec![
		r#""analyzed""#,
		r#""not_analyzed""#,
		r#""no""#
	];

	let mut success = true;
	for i in 0..index_opts.len() {
		if expected_opts[i] != &index_opts[i] {
			success = false;
			break;
		}
	}

	assert!(success);*/
}

#[test]
fn serialise_mapping_field_filter() {
	/*let index_opts: Vec<String> = vec![
		IndexAnalysis::Analyzed,
		IndexAnalysis::NotAnalyzed,
		IndexAnalysis::No
	]
	.iter()
	.map(|i| serde_json::to_string(i).unwrap())
	.collect();

	let expected_opts = vec![
		r#""analyzed""#,
		r#""not_analyzed""#,
		r#""no""#
	];

	let mut success = true;
	for i in 0..index_opts.len() {
		if expected_opts[i] != &index_opts[i] {
			success = false;
			break;
		}
	}

	assert!(success);*/
}

#[test]
fn serialise_mapping_index_options() {
	/*let index_opts: Vec<String> = vec![
		IndexAnalysis::Analyzed,
		IndexAnalysis::NotAnalyzed,
		IndexAnalysis::No
	]
	.iter()
	.map(|i| serde_json::to_string(i).unwrap())
	.collect();

	let expected_opts = vec![
		r#""analyzed""#,
		r#""not_analyzed""#,
		r#""no""#
	];

	let mut success = true;
	for i in 0..index_opts.len() {
		if expected_opts[i] != &index_opts[i] {
			success = false;
			break;
		}
	}

	assert!(success);*/
}

#[test]
fn serialise_mapping_norms() {
	/*let index_opts: Vec<String> = vec![
		IndexAnalysis::Analyzed,
		IndexAnalysis::NotAnalyzed,
		IndexAnalysis::No
	]
	.iter()
	.map(|i| serde_json::to_string(i).unwrap())
	.collect();

	let expected_opts = vec![
		r#""analyzed""#,
		r#""not_analyzed""#,
		r#""no""#
	];

	let mut success = true;
	for i in 0..index_opts.len() {
		if expected_opts[i] != &index_opts[i] {
			success = false;
			break;
		}
	}

	assert!(success);*/
}

#[test]
fn serialise_mapping_norms_loading() {
	/*let index_opts: Vec<String> = vec![
		IndexAnalysis::Analyzed,
		IndexAnalysis::NotAnalyzed,
		IndexAnalysis::No
	]
	.iter()
	.map(|i| serde_json::to_string(i).unwrap())
	.collect();

	let expected_opts = vec![
		r#""analyzed""#,
		r#""not_analyzed""#,
		r#""no""#
	];

	let mut success = true;
	for i in 0..index_opts.len() {
		if expected_opts[i] != &index_opts[i] {
			success = false;
			break;
		}
	}

	assert!(success);*/
}

#[test]
fn serialise_mapping_terms_vector() {
	/*let index_opts: Vec<String> = vec![
		IndexAnalysis::Analyzed,
		IndexAnalysis::NotAnalyzed,
		IndexAnalysis::No
	]
	.iter()
	.map(|i| serde_json::to_string(i).unwrap())
	.collect();

	let expected_opts = vec![
		r#""analyzed""#,
		r#""not_analyzed""#,
		r#""no""#
	];

	let mut success = true;
	for i in 0..index_opts.len() {
		if expected_opts[i] != &index_opts[i] {
			success = false;
			break;
		}
	}

	assert!(success);*/
}
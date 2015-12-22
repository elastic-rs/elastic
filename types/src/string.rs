#[derive(Default)]
pub struct StringMeta {
	pub analyzer: Option<String>,
	pub boost: Option<f32>,
	pub doc_values: Option<bool>,
	pub fielddata: Option<FieldData>
}

pub enum FieldData {
	PagedBytes,
	Disabled
}

impl FieldData {
	fn parse(fd: &str) -> FieldData {
		match fd {
			"disabled" => FieldData::Disabled,
			_ => FieldData::PagedBytes
		}
	}
}

impl Default for FieldData {
	fn default() -> FieldData {
		FieldData::PagedBytes
	}
}

impl ToString for FieldData {
	fn to_string(&self) -> String {
		match *self {
			FieldData::Disabled => "disabled".to_string(),
			FieldData::PagedBytes => "paged_bytes".to_string()
		}
	}
}
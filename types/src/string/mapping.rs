use std::collections::BTreeMap;
use std::marker::PhantomData;
use serde;
use serde::{ Serializer, Serialize };
use ::mapping::{ ElasticMapping, ElasticType, IndexAnalysis };

/// The base requirements for mapping a `string` type.
/// 
/// # Examples
/// 
/// Custom mappings can be defined by implementing `ElasticStringMapping`:
/// 
/// ```
/// use std::collections::BTreeMap;
/// use elastic_types::string::{ ElasticStringMapping, ElasticStringFieldMapping, NullValue };
/// 
/// struct MyStringMapping;
/// impl ElasticStringMapping for MyStringMapping {
/// 	fn get_null_value() -> Option<NullValue> {
/// 		Some(NullValue::Default("my default value"))
/// 	}
/// 
/// 	fn get_fields() -> Option<BTreeMap<&'static str, ElasticStringFieldMapping>> {
/// 		let mut fields = BTreeMap::new();
/// 		fields.insert("raw", ElasticStringFieldMapping {
/// 			analyzer: Some("my_analyzer"),
/// 			..Default::default()
/// 		});
/// 		fields.insert("bm25_field", ElasticStringFieldMapping {
/// 			similarity: Some("BM25"),
/// 			..Default::default()
/// 		});
/// 		
/// 		Some(fields)
/// 	}
/// }
/// ```
pub trait ElasticStringMapping 
where Self : Sized + Serialize {
	/// Field-level index time boosting. Accepts a floating point number, defaults to `1.0`.
	fn get_boost() -> Option<f32> {
		None
	}

	/// Should the field be stored on disk in a column-stride fashion, 
	/// so that it can later be used for sorting, aggregations, or scripting? 
	/// Accepts `true` (default) or `false`.
	fn get_doc_values() -> Option<bool> {
		None
	}

	/// Whether or not the field value should be included in the `_all` field? 
	/// Accepts true or false. 
	/// Defaults to `false` if index is set to `no`, or if a parent object field sets `include_in_all` to false. 
	/// Otherwise defaults to `true`.
	fn get_include_in_all() -> Option<bool> {
		None
	}

	/// Should the field be searchable? Accepts `not_analyzed` (default) and `no`.
	fn get_index() -> Option<IndexAnalysis> {
		None
	}

	/// Whether the field value should be stored and retrievable separately from the `_source` field. 
	/// Accepts `true` or `false` (default).
	fn get_store() -> Option<bool> {
		None
	}

	/// The analyzer which should be used for analyzed string fields, 
	/// both at index-time and at search-time (unless overridden by the `search_analyzer`). 
	/// Defaults to the default index analyzer, or the `standard` analyzer.
	fn get_analyzer() -> Option<&'static str> {
		None
	}

	/// Can the field use in-memory fielddata for sorting, aggregations, or scripting? 
	/// Accepts disabled or `paged_bytes` (default). 
	/// Not analyzed fields will use doc values in preference to fielddata.
	fn get_fielddata() -> Option<FieldData> {
		None
	}

	/// Multi-fields allow the same string value to be indexed in multiple ways for different purposes, 
	/// such as one field for search and a multi-field for sorting and aggregations, 
	/// or the same string value analyzed by different analyzers.
	fn get_fields() -> Option<BTreeMap<&'static str, ElasticStringFieldMapping>> {
		None
	}

	/// Do not index or analyze any string longer than this value. Defaults to 0 (disabled).
	fn get_ignore_above() -> Option<usize> {
		None
	}

	/// What information should be stored in the index, for search and highlighting purposes. 
	/// Defaults to positions for analyzed fields, and to docs for not_analyzed fields.
	fn get_index_options() -> Option<IndexOptions> {
		None
	}

	/// Whether field-length should be taken into account when scoring queries.
	fn get_norms() -> Option<Norms> {
		None
	}

	/// Accepts a string value which is substituted for any explicit null values. 
	/// Defaults to null, which means the field is treated as missing.
	fn get_null_value() -> Option<&'static str> {
		None
	}

	/// Whether field-length should be taken into account when scoring queries. 
	fn get_position_increment_gap() -> Option<usize> {
		None
	}

	/// The analyzer that should be used at search time on analyzed fields. 
	/// Defaults to the analyzer setting.
	fn get_search_analyzer() -> Option<&'static str> {
		None
	}

	/// The analyzer that should be used at search time when a phrase is encountered. 
	/// Defaults to the search_analyzer setting.
	fn get_search_quote_analyzer() -> Option<&'static str> {
		None
	}

	/// Which scoring algorithm or similarity should be used. Defaults to `default`, which uses TF/IDF.
	fn get_similarity() -> Option<&'static str> {
		None
	}

	/// Whether term vectors should be stored for an analyzed field. Defaults to `no`.
	fn get_term_vector() -> Option<TermVector> {
		None
	}
}

impl <M: ElasticStringMapping> ElasticMapping for M {
	type Visitor = ElasticStringMappingVisitor<M>;

	fn get_type() -> &'static str {
		"string"
	}
}

/// A multi-field string mapping.
#[derive(Debug, Default, Clone, Copy, Serialize)]
pub struct ElasticStringFieldMapping {
/// The analyzer which should be used for analyzed string fields, 
	/// both at index-time and at search-time (unless overridden by the `search_analyzer`). 
	/// Defaults to the default index analyzer, or the `standard` analyzer.
	pub analyzer: Option<&'static str>,
	/// Can the field use in-memory fielddata for sorting, aggregations, or scripting? 
	/// Accepts disabled or `paged_bytes` (default). 
	/// Not analyzed fields will use doc values in preference to fielddata.
	pub fielddata: Option<FieldData>,
	/// Do not index or analyze any string longer than this value. Defaults to 0 (disabled).
	pub ignore_above: Option<usize>,
	/// What information should be stored in the index, for search and highlighting purposes. 
	/// Defaults to positions for analyzed fields, and to docs for not_analyzed fields.
	pub index_options: Option<IndexOptions>,
	/// Whether field-length should be taken into account when scoring queries.
	pub norms: Option<Norms>,
	/// Accepts a string value which is substituted for any explicit null values. 
	/// Defaults to null, which means the field is treated as missing.
	pub null_value: Option<&'static str>,
	/// Whether field-length should be taken into account when scoring queries. 
	pub position_increment_gap: Option<usize>,
	/// The analyzer that should be used at search time on analyzed fields. 
	/// Defaults to the analyzer setting.
	pub search_analyzer: Option<&'static str>,
	/// The analyzer that should be used at search time when a phrase is encountered. 
	/// Defaults to the search_analyzer setting.
	pub search_quote_analyzer: Option<&'static str>,
	/// Which scoring algorithm or similarity should be used. Defaults to `default`, which uses TF/IDF.
	pub similarity: Option<&'static str>,
	/// Whether term vectors should be stored for an analyzed field. Defaults to `no`.
	pub term_vector: Option<TermVector>
}

/// Can the field use in memory fielddata for sorting, aggregations, or scripting?
#[derive(Debug, Clone, Copy)]
pub enum FieldData {
	/// Allow in-memory fielddata.
	PagedBytes(Option<FieldDataLoading>, Option<FieldDataFilter>),
	/// Disallow in-memory fielddata.
	Disabled
}

impl serde::Serialize for FieldData {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
	where S: Serializer
	{
		match *self {
			FieldData::PagedBytes(None, None) => Ok(()),
			fielddata => serializer.serialize_struct("fielddata", FieldDataVisitor::new(fielddata))
		}
	}
}

struct FieldDataVisitor {
	value: FieldData
}
impl FieldDataVisitor {
	pub fn new(value: FieldData) -> FieldDataVisitor {
		FieldDataVisitor {
			value: value
		}
	}
}

impl serde::ser::MapVisitor for FieldDataVisitor {
	fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
	where S: Serializer {
		match self.value {
			FieldData::Disabled => try!(serializer.serialize_struct_elt("format", "disabled")),
			FieldData::PagedBytes(loading, filter) => {
				match loading {
					Some(loading) => try!(serializer.serialize_struct_elt("loading", loading)),
					None => ()
				};

				match filter {
					Some(filter) => try!(serializer.serialize_struct_elt("filter", filter)),
					None => ()
				};
			}
		}

		Ok(None)
	}
}

/// This per-field setting controls when fielddata is loaded into memory.
#[derive(Debug, Clone, Copy)]
pub enum FieldDataLoading {
	/// Fielddata is only loaded into memory when it is needed. (default).
	Lazy,
	/// Fielddata is loaded into memory before a new search segment becomes visible to search. 
	/// This can reduce the latency that a user may experience if their search request has to 
	/// trigger lazy loading from a big segment.
	Eager,
	/// Loading fielddata into memory is only part of the work that is required. 
	/// After loading the fielddata for each segment, Elasticsearch builds the 
	/// Global ordinals data structure to make a list of all unique terms across all the segments in a shard. 
	/// By default, global ordinals are built lazily. If the field has a very high cardinality, 
	/// global ordinals may take some time to build, in which case you can use eager loading instead.
	EagerGlobalOrdinals
}

impl serde::Serialize for FieldDataLoading {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
	where S: Serializer
	{
		serializer.serialize_str(match *self {
			FieldDataLoading::Lazy => "lazy",
			FieldDataLoading::Eager => "eager",
			FieldDataLoading::EagerGlobalOrdinals => "eager_global_ordinals"
		})
	}
}

/// Fielddata filtering can be used to reduce the number of terms loaded into memory, and thus reduce memory usage.
#[derive(Debug, Clone, Copy)]
pub enum FieldDataFilter {
	/// The frequency filter allows you to only load terms whose term frequency falls between a min and max value, 
	/// which can be expressed an absolute number (when the number is bigger than 1.0) or as a percentage (eg 0.01 is 1% and 1.0 is 100%). 
	/// Frequency is calculated per segment. Percentages are based on the number of docs which have a value for the field, 
	/// as opposed to all docs in the segment.
	Frequency(FrequencyFilter),
	/// Terms can also be filtered by regular expression - only values which match the regular expression are loaded. 
	/// Note: the regular expression is applied to each term in the field, not to the whole field value. 
	Regex(RegexFilter)
}

/// The frequency filter allows you to only load terms whose term frequency falls between a min and max value, 
/// which can be expressed an absolute number (when the number is bigger than 1.0) or as a percentage (eg 0.01 is 1% and 1.0 is 100%). 
/// Frequency is calculated per segment. Percentages are based on the number of docs which have a value for the field, 
/// as opposed to all docs in the segment.
#[derive(Debug, Clone, Copy, Serialize)]
pub struct FrequencyFilter { 
	/// The min frequency.
	pub min: f32, 
	/// The max frequency.
	pub max: f32, 
	/// The minimum segment size before loading.
	pub min_segment_size: usize 
}

/// Terms can also be filtered by regular expression - only values which match the regular expression are loaded. 
/// Note: the regular expression is applied to each term in the field, not to the whole field value. 
#[derive(Debug, Clone, Copy, Serialize)]
pub struct RegexFilter { 
	/// The regex pattern.
	pub pattern: &'static str 
}

impl serde::Serialize for FieldDataFilter {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
	where S: Serializer
	{
		serializer.serialize_struct("filter", FieldDataFilterVisitor::new(*self))
	}
}

struct FieldDataFilterVisitor {
	value: FieldDataFilter
}
impl FieldDataFilterVisitor {
	pub fn new(value: FieldDataFilter) -> FieldDataFilterVisitor {
		FieldDataFilterVisitor {
			value: value
		}
	}
}

impl serde::ser::MapVisitor for FieldDataFilterVisitor {
	fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
	where S: Serializer {
		match self.value {
			FieldDataFilter::Frequency(freq) => try!(serializer.serialize_struct_elt("frequency", freq)),
			FieldDataFilter::Regex(pat) => try!(serializer.serialize_struct_elt("regex", pat))
		}

		Ok(None)
	}
}

/// The index_options parameter controls what information is added to the inverted index, for search and highlighting purposes.
#[derive(Debug, Clone, Copy)]
pub enum IndexOptions {
	/// Only the doc number is indexed. Can answer the question Does this term exist in this field?
	Docs,
	/// Doc number and term frequencies are indexed. 
	/// Term frequencies are used to score repeated terms higher than single terms.
	Freqs,
	/// Doc number, term frequencies, and term positions (or order) are indexed. 
	/// Positions can be used for proximity or phrase queries.
	Positions,
	/// Doc number, term frequencies, positions, 
	/// and start and end character offsets (which map the term back to the original string) are indexed. 
	/// Offsets are used by the postings highlighter.
	Offsets
}

impl serde::Serialize for IndexOptions {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
	where S: Serializer
	{
		serializer.serialize_str(match *self {
			IndexOptions::Docs => "docs",
			IndexOptions::Freqs => "freqs",
			IndexOptions::Positions => "positions",
			IndexOptions::Offsets => "offsets"
		})
	}
}

/// Whether field-length should be taken into account when scoring queries. 
#[derive(Debug, Clone, Copy)]
pub enum Norms {
	/// Enabled norms with eagerness.
	Enabled { 
		/// Whether the loading is eager or lazy.
		loading: NormsLoading 
	},
	/// Disabled norms.
	Disabled
}

struct NormsVisitor {
	value: Norms
}
impl NormsVisitor {
	pub fn new(value: Norms) -> NormsVisitor {
		NormsVisitor {
			value: value
		}
	}
}

impl serde::ser::MapVisitor for NormsVisitor {
	fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
	where S: Serializer {
		match self.value {
			Norms::Enabled { loading: l } => try!(serializer.serialize_struct_elt("loading", l)),
			Norms::Disabled => try!(serializer.serialize_struct_elt("enabled", false))
		}

		Ok(None)
	}
}

impl serde::Serialize for Norms {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
	where S: Serializer
	{
		serializer.serialize_struct("norms", NormsVisitor::new(*self))
	}
}

/// Whether the norms should be loaded into memory eagerly (`eager`), 
/// whenever a new segment comes online, or they can loaded lazily (`lazy`, default).
#[derive(Debug, Clone, Copy)]
pub enum NormsLoading {
	/// Load norms whenever a new segment comes online.
	Eager,
	/// Load norms lazily.
	Lazy
}

impl serde::Serialize for NormsLoading {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
	where S: Serializer
	{
		serializer.serialize_str(match *self {
			NormsLoading::Eager => "eager",
			NormsLoading::Lazy => "lazy"
		})
	}
}

/// Term vectors contain information about the terms produced by the analysis process.
#[derive(Debug, Clone, Copy)]
pub enum TermVector {
	/// No term vectors are stored. (default)
	No,
	/// Just the terms in the field are stored.
	Yes,
	/// Terms and positions are stored.
	WithPositions,
	/// Terms and character offsets are stored.
	WithOffsets,
	/// Terms, positions, and character offsets are stored.
	WithPositionsOffsets
}

impl serde::Serialize for TermVector {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
	where S: Serializer
	{
		serializer.serialize_str(match *self {
			TermVector::No => "no",
			TermVector::Yes => "yes",
			TermVector::WithPositions => "with_positions",
			TermVector::WithOffsets => "with_offsets",
			TermVector::WithPositionsOffsets => "with_positions_offsets"
		})
	}
}

/// Default mapping for `String`.
#[derive(Debug, Clone)]
pub struct DefaultStringMapping;
impl ElasticStringMapping for DefaultStringMapping { }

impl Serialize for DefaultStringMapping {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
		where S: serde::Serializer
	{
		serializer.serialize_struct("mapping", ElasticStringMappingVisitor::<DefaultStringMapping>::default())
	}
}

/// A Rust representation of an Elasticsearch `string`.
pub trait ElasticStringType<T: ElasticMapping + ElasticStringMapping> where Self: Sized + ElasticType<T, ()> { }

//TODO: Make this take in str for field name
/// Base visitor for serialising string mappings.
#[derive(Debug, PartialEq)]
pub struct ElasticStringMappingVisitor<T: ElasticMapping> {
	phantom: PhantomData<T>
}

impl <T: ElasticMapping> Default for ElasticStringMappingVisitor<T> {
	fn default() -> ElasticStringMappingVisitor<T> {
		ElasticStringMappingVisitor::<T> {
			phantom: PhantomData
		}
	}
}

impl <T: ElasticMapping + ElasticStringMapping> serde::ser::MapVisitor for ElasticStringMappingVisitor<T> {
	fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
	where S: serde::Serializer {
		match T::get_boost() {
			Some(boost) => try!(serializer.serialize_struct_elt("boost", boost)),
			None => ()
		};
		match T::get_doc_values() {
			Some(doc_values) => try!(serializer.serialize_struct_elt("doc_values", doc_values)),
			None => ()
		};
		match T::get_include_in_all() {
			Some(include_in_all) => try!(serializer.serialize_struct_elt("include_in_all", include_in_all)),
			None => ()
		};
		match T::get_index() {
			Some(index) => try!(serializer.serialize_struct_elt("index", index)),
			None => ()
		};
		match T::get_store() {
			Some(store) => try!(serializer.serialize_struct_elt("store", store)),
			None => ()
		};

		match T::get_analyzer() {
			Some(analyzer) => try!(serializer.serialize_struct_elt("analyzer", analyzer)),
			None => ()
		};

		match T::get_fields() {
			Some(fields) => try!(serializer.serialize_struct_elt("fields", fields)),
			None => ()
		};

		match T::get_fielddata() {
			Some(FieldData::PagedBytes(None, None)) => (),
			Some(fielddata) => try!(serializer.serialize_struct_elt("fielddata", fielddata)),
			None => ()
		};

		match T::get_ignore_above() {
			Some(ignore_above) => try!(serializer.serialize_struct_elt("ignore_above", ignore_above)),
			None => ()
		};

		match T::get_index_options() {
			Some(index_options) => try!(serializer.serialize_struct_elt("index_options", index_options)),
			None => ()
		};

		match T::get_norms() {
			Some(norms) => try!(serializer.serialize_struct_elt("norms", norms)),
			None => ()
		};

		match T::get_null_value() {
			Some(null_value) => try!(serializer.serialize_struct_elt("null_value", null_value)),
			None => ()
		};

		match T::get_position_increment_gap() {
			Some(position_increment_gap) => try!(serializer.serialize_struct_elt("position_increment_gap", position_increment_gap)),
			None => ()
		};

		match T::get_search_analyzer() {
			Some(search_analyzer) => try!(serializer.serialize_struct_elt("search_analyzer", search_analyzer)),
			None => ()
		};

		match T::get_search_quote_analyzer() {
			Some(search_quote_analyzer) => try!(serializer.serialize_struct_elt("search_quote_analyzer", search_quote_analyzer)),
			None => ()
		};

		match T::get_similarity() {
			Some(similarity) => try!(serializer.serialize_struct_elt("similarity", similarity)),
			None => ()
		};

		match T::get_term_vector() {
			Some(term_vector) => try!(serializer.serialize_struct_elt("term_vector", term_vector)),
			None => ()
		};

		Ok(None)
	}
}
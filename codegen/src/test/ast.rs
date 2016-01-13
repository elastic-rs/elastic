//! Test Spec Abstract Syntax Tree
//! 
//! Contains Rust structures for the YAML Tests.
//! Structs in this module are designed for inspecting after instantiation by `serde`, rather than constructing directly.

/// Do expression.
/// 
/// Represents an action to take that involves the Elasticsearch API.
pub struct Do<'a> {
	/// The action to perform.
	pub action: &'a str,
	/// The arguments to use.
	pub args: Vec<&'a str>
}
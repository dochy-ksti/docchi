pub(crate) mod json_to_rust;
pub(crate) mod rust_to_json;
pub(crate) mod version_adjuster;
pub(crate) mod structs;

///This module contains low-level accessors.
/// They are intended to be used by generated codes, not humans.
pub mod intf;

mod imp;
mod error;

pub use imp::generate_interface::generate_interface;
pub use imp::generate_accessor_from_json_dir::generate_accessor_from_json_dir;
pub use imp::structs::root_source::RootSource;
pub use error::{IntfError, IntfResult};
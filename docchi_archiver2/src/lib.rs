#![warn(unreachable_pub)]
#![warn(unused_crate_dependencies)]

mod imp;
#[cfg(test)]
#[allow(dead_code, unused_imports)]
mod testing;
mod error;



pub type ArcResult<T> = std::result::Result<T, crate::error::NouArcError>;
pub use error::NouArcError;

pub use imp::read_archive_data_from_directory::read_archive_data_from_directory;
pub use imp::read_archive::read_archive;
pub use imp::write_archive::write_archive;

pub use imp::structs::archive_data::{ArchiveData, ArchiveDataItem};

pub use imp::structs::archive_options::ArchiveOptions;
pub use imp::structs::archive_options::ArchiveOptionsBuilder;


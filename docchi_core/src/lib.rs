#![feature(once_cell)]

mod old;
mod error;
mod imp;
pub mod structs;
#[allow(dead_code)]
mod testing;

pub static JSON_ARC_OPT : SyncLazy<ArchiveOptions> = SyncLazy::new(|| {
    ArchiveOptionsBuilder::new()
        .add_extension("json5")
        .archive_subfolders(false)
        .build().unwrap()
});

pub use imp::rust_to_json::root_to_json::root_to_json_new_default;
pub use imp::json_to_rust::roots::json_dir_to_root::json_dir_to_root;
pub use imp::json_to_rust::roots::json_dir_to_root::json_dir_to_root_with_hash;
pub use imp::json_to_rust::roots::archive_file_to_root::archive_file_to_root_with_hash;
pub use imp::json_to_rust::roots::archive_file_to_root::archive_file_to_root;
pub use imp::json_to_rust::roots::archive_file_to_root::read_archive_to_root_with_hash;
pub use imp::json_to_rust::roots::archive_file_to_root::read_archive_to_root;
pub use imp::json_to_rust::roots::archive_src_dir::archive_src_dir;
pub use imp::json_to_rust::roots::archive_data_to_root::archive_to_root;
pub use imp::version_adjuster::version_adjuster::adjust_versions;
pub use imp::intf;

pub use imp::structs::util::hash_m::HashMt;
pub use imp::structs::util::hash_m::HashM;
pub use imp::structs::util::set_sabun::SetSabunError;
pub use imp::structs::util::identity_equal_trait::IdentityEqual;

pub use imp::structs::json_file::JsonFile;
pub use imp::structs::json_file::JsonFileImpl;
use std::lazy::SyncLazy;
use docchi_archiver2::{ArchiveOptions, ArchiveOptionsBuilder};

pub use error::{CoreError, CoreResult};


#![deny(unreachable_pub)]
#![deny(unused_crate_dependencies)]

mod imp;
mod diff_error;

pub use imp::get_diff::get_diff;
pub use imp::get_diff::get_kvals;
pub use imp::apply_diff::apply_diff;
pub use imp::apply_diff::apply_root_diff_r;
pub use imp::apply_diff::get_root_diff_r;
pub use imp::structs_read::RootDiffR;
pub use diff_error::{DiffResult, DiffError};
use docchi_core::intf::*;
use docchi_core::structs::RootObject;
use crate::imp::structs::root_source::RootSource;
use crate::imp::generate_root_source::generate_root_source;


// データに静的にアクセスできるラッパーを生成する。
// RootIntf::newしそこを起点にしてアクセスする。
// RootIntfから取れたポインタは、RootIntfが削除されれば全て不正になる

/// Generates Rust source code to access the root object
pub fn generate_interface(root : &RootObject) -> RootSource{
    let mem_descs = member_desc::get_member_desc(root);
    generate_root_source(&mem_descs)

}

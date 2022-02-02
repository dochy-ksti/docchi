use crate::error::CoreResult;
use crate::imp::structs::root_obj::RootObject;
use crate::imp::structs::root_value::RootValue;
use std::sync::Arc;
use crate::imp::structs::meta_table::MetaTable;
use crate::imp::json_to_rust::roots::archive_data_to_root::TItem;
use crate::imp::structs::rust_list::ConstTable;

///root.jsonからとったRootに、各ファイルからとった個別のメンバを混ぜる。ファイルはアルファベット順に、root.jsonの末尾に加わっていく
/// vecはあらかじめアルファベット順に並んでいる必要がある
pub(crate) fn construct_root(root : RootObject, vec : Vec<TItem>) -> CoreResult<RootObject>{
    let (default_v, sabun_v, old, _meta) = root.deconstruct();
    let mut default_v = default_v;
    let default = Arc::make_mut(&mut default_v).def_mut();
    let mut sabun_v = sabun_v;
    let sabun = Arc::make_mut(&mut sabun_v);
    println!("default v {}", default.len());
    for item in vec {
        match item {
            TItem::Item((name, value, sab)) => {
                println!("item {}", name);
                let id = default.len();
                default.insert(name.to_string(), (id, value));
                if let Some(sab) = sab {
                    sabun.insert(name, sab);
                }
            },
            TItem::Table((name, def, list, old)) =>{
                println!("table {}", name);
                let id = default.len();
                let root = RootValue::Table(ConstTable::construct(def, list, old));
                default.insert(name, (id, root));
            }
        }
    }
    let meta = MetaTable::from_root(default_v.def());
    println!("default v {}", default_v.len());
    let root = RootObject::construct(default_v, sabun_v, old, Arc::new(meta));

    // if validation{
    //     validate_root(&root, false)?
    // }

    return Ok(root);
}
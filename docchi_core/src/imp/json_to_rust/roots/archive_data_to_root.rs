use docchi_archiver2::ArchiveData;
use crate::error::{CoreResult};
use crate::structs::{RootObject, RootValue, RootSabValue, ListDefObj};
use crate::imp::json_to_rust::construct_root::construct_root;
use crate::imp::structs::docchi_archive::{ArchivingItem, DocchiArchive};
use crate::imp::json_to_rust::validation::validate_root::validate_root;
use crate::HashM;
use crate::imp::structs::rust_list::ConstItem;
use crate::imp::structs::util::hash_m::HashS;

pub fn archive_to_root(archive : DocchiArchive, validation : bool) -> CoreResult<RootObject>{
    let (root, _hash) = archive_data_to_root_with_hash(archive.data)?;
    if validation{
        validate_root(&root, false)?;
    }
    Ok(root)
}

pub(crate) fn archive_data_to_root_with_hash(data : ArchiveData<CoreResult<ArchivingItem>>)
    -> CoreResult<(RootObject, u128)> {
    let hash = data.hash();
    let mut tree = data.deconstruct();
    let item = if let Some(item) = tree.remove("root.json5"){
        item
    } else{
        Err("root.json5 couldn't be found")?
    };
    let (item, _) = item.deconstruct();
    let root = match item? {
        ArchivingItem::Root(root) => root,
        _ => unreachable!(),
    };


    let mut vec = Vec::with_capacity(tree.len());
    for (path, val) in tree {
        let (item, _) = val.deconstruct();
        match item {
            Ok(ArchivingItem::Item((name, val, sab))) =>{
                match val{
                    RootValue::Table(table) => {
                        let (def, list, old) = table.deconstruct();
                        vec.push(TItem::Table((name, def, list, old)));
                    }
                    _ =>{ vec.push(TItem::Item((name, val, sab))); }
                }
            },
            Ok(ArchivingItem::TableItem((parent, id, item))) =>{
                //path順に並べると、hoge.json5 hoge/huga.json5...となり、.と/はASCIIコード表で隣同士なので、
                //validな名前としてその間に入り込める可能性はない。なのでTableとTableItemは連続する、
                //というかなりナイーブな実装になっている。しかしそうしないと微妙に手間のかかる処理になって気持ち悪い(実害はないと思うが
                if let Some(TItem::Table((name, _def,list,_old))) = vec.last_mut(){
                    if parent.eq(name) {
                        list.insert(id, item);
                    } else{
                        Err(format!("{} corresponding table couldn't be found", path))?
                    }
                } else{
                    Err(format!("{} corresponding table couldn't be found", path))?
                }
            },
            Err(e) =>{ return Err(e); }
            _ => { Err("Multiple Root?")? } //Rootが複数なければここにはこれない・・・
        }
    }

    let root = construct_root(root, vec)?;
    return Ok((root, hash));
}

pub(crate) enum TItem{
    Item((String, RootValue, Option<RootSabValue>)),
    Table((String, Box<ListDefObj>, Box<HashM<String, ConstItem>>, Box<HashS<String>>))
}
use docchi_archiver2::ArchiveData;
use crate::error::{CoreResult};
use crate::structs::{ RootObject};
use crate::imp::json_to_rust::construct_root::construct_root;
use crate::imp::structs::dochy_archive::{ArchivingItem, DochyArchive};
use crate::imp::json_to_rust::validation::validate_root::validate_root;

pub fn archive_to_root(archive : DochyArchive, validation : bool) -> CoreResult<RootObject>{
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
    let root: CoreResult<RootObject> = (|| {
        let key =
            if let Some((key, _)) = tree.iter().find(|(_key, val)| {
                if let Ok(ArchivingItem::Root(_)) = val.converted_data() {
                    true
                } else {
                    false
                }
            }) {
                key.to_string()
            } else {
                Err("couldn't find root.json5")?
            };

        let item = tree.remove(&key).unwrap();
        let (item, _) = item.deconstruct();

        if let Ok(ArchivingItem::Root(root)) = item {
            return Ok(root)
        } else {
            unreachable!()
        }
    })();
    let root = root?;

    let mut vec = Vec::with_capacity(tree.len());
    for (_path, val) in tree {
        let (item, _) = val.deconstruct();
        match item {
            Ok(ArchivingItem::Item((name, val, sab))) =>{
                vec.push((name, val, sab));
            }
            Err(e) =>{ return Err(e); }
            _ => { Err("Multiple Root?")? } //Rootが複数なければここにはこれない・・・
        }
    }

    let root = construct_root(root, vec)?;
    return Ok((root, hash));
}
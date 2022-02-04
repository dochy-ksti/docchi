use crate::{ArcResult, ArchiveOptions, read_archive_data_from_directory, ArchiveOptionsBuilder, write_archive, read_archive, ArchiveData};

use std::sync::Arc;
use std::ops::Deref;
use std::sync::mpsc::channel;
use once_cell::sync::Lazy;

pub(crate) static JSON_ARC_OPT : Lazy<ArchiveOptions> = Lazy::new(|| {
    ArchiveOptionsBuilder::new()
        .add_extension("json5")
        .add_extension("txt")
        .archive_subfolders(true)
        .build().unwrap()
});

fn hoge<'a, T:Send + 'static>(f : impl Fn(&'a [u8]) -> T + Send + Sync + 'static) -> Arc<dyn Fn(&'a [u8]) -> T + Send + Sync + 'static>{
    Arc::new(f)
}

#[test]
fn archive_test() -> ArcResult<()>{
    let archive_data : ArchiveData<()> = read_archive_data_from_directory(
        "./src/json/general",
        &*JSON_ARC_OPT,
        |_name, _dat| (),

    )?;
    for (path, _) in archive_data.btree(){
        dbg!(path);
    }
    let mut buf = Vec::new();
    write_archive(&archive_data, &mut buf)?;
    let read : ArchiveData<()> = read_archive(|_name, _slice| (), &mut buf.as_slice())?;
    for (path, dat) in read.btree(){
        dbg!(path);
        let got = archive_data.btree().get(path).ok_or("error")?;
        assert_eq!(got.raw_data(), dat.raw_data());
    }

    Ok(())
}


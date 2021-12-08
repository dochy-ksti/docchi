use anyhow::Result;
use crate::a1_hello_world::hello_world_accessor::RootIntf;
use docchi::fs::filesys::{save_docchi_file, SaveDirInfo, list_docchi_files, load_docchi_file};
use docchi::fs::common::{CurrentSrc};

#[test]
fn hello_world_save_test() -> Result<()> // DpResult can handle every error type of Docchi.
{
    let save_dir = "src/a1_hello_world/save_dir";

    // make sure the save_dir exists. pretty unnecessary for Hello World.
    std::fs::create_dir(save_dir).ok();

    let src_dir = "src/a1_hello_world/src_dir";

    // You need SaveDirInfo to save Docchi-files.
    // save_dir and src_dir's paths are needed to create SaveDirInfo.
    let info = SaveDirInfo::create(save_dir, CurrentSrc::from_src_dir(src_dir))?;
    //SaveDirInfo::create returns FsResult, which is the result type of the module docchi_fs.
    //It's automatically converted to anyhow::Result with ? operator.
    //Every error type of Docchi implements std::error::Error,
    //so you can convert them into anyhow::Error with ? operator


    // RootObject is created from Docchi-source. It can be cloned via SaveDirInfo
    let root = info.clone_src_root();
    let mut root = RootIntf::new(root);
    root.set_message("Hello the next world".to_string());

    let _saved_path = save_docchi_file(
        &info,
        "next_world.docchi",
        root.root_obj_ref(),
        true)?;

    hello_world_load_test()?;

    Ok(())
}

fn hello_world_load_test() -> Result<()> {
    let save_dir = "src/a1_hello_world/save_dir";
    let src_dir = "src/a1_hello_world/src_dir";
    let info = SaveDirInfo::create(save_dir, CurrentSrc::from_src_dir(src_dir))?;


    let files = list_docchi_files(save_dir)?;

    let file = files.iter().find(|f| f.name() == "next_world.docchi").unwrap();

    let root = load_docchi_file(
        file.calc_path(save_dir),
        &info,
        true
    )?;
    let root = RootIntf::new(root);
    assert_eq!(root.message(), "Hello the next world");

    Ok(())
}
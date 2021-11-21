use crate::imp::filesys::save_docchi_file::save_docchi_file;
use docchi_core::intf::RootObjectPtr;
use docchi_core::intf::root::{set_bool, get_bool, set_int, get_int};
use docchi_core::structs::Qv;
use tempfile::tempdir;
use crate::error::FsResult;
use crate::imp::filesys::load_docchi_file::load_docchi_file;
use crate::test_fs::copy_dir_all::copy_dir_all;
use crate::imp::common::list::list_files::list_files;
use crate::imp::filesys::save_dir_info::SaveDirInfo;
use crate::imp::filesys::save_cache_map::force_update_and_get_info_us;
use crate::common::CurrentSrc;


///ソースを書き換えてadjustさせるテスト
//#[test]
fn save_test() -> FsResult<()> {
    let dir = tempdir()?;
    let proj_dir_path = dir.path();
    let src_dir_path = proj_dir_path.join("simple_src");

    copy_dir_all("src/json_dir/simple", &src_dir_path)?;

    let first_save_path;
    {
        let current_src = CurrentSrc::SrcDir(src_dir_path.clone());
        let info = SaveDirInfo::create(proj_dir_path, current_src)?;

        let mut root = info.clone_src_root();
        let p = RootObjectPtr::new(&mut root);
        set_bool(p, "b", Qv::Val(true));

        let path = save_docchi_file(&info, "test1", &root, false)?;

        let mut loaded = load_docchi_file(&path, &info, false)?;
        first_save_path = path;

        let p = RootObjectPtr::new(&mut loaded);
        let b = get_bool(p, "b")?;
        assert_eq!(b.value().unwrap(), &true);
    }
    let src_dir_path = proj_dir_path.join("simple_src_mod");

    copy_dir_all("src/json_dir/simple", &src_dir_path)?;
    std::fs::copy("src/json_dir/simple_mod1/root.json5", &src_dir_path.join("root.json5"))?;

    {
        let current_src = CurrentSrc::SrcDir(src_dir_path.clone());
        let info = unsafe{ force_update_and_get_info_us(proj_dir_path, current_src)? };

        let mut root = info.clone_src_root();
        let p = RootObjectPtr::new(&mut root);
        set_int(p, "int", Qv::Val(-1));

        let path = save_docchi_file(&info, "test2", &root, false)?;
        let mut loaded = load_docchi_file(&path, &info, false)?;

        let p = RootObjectPtr::new(&mut loaded);
        let i = get_int(p, "int")?;
        assert_eq!(i.value().unwrap(), &-1);
    }

    {
        let current_src = CurrentSrc::SrcDir(src_dir_path.clone());
        let info = SaveDirInfo::create(proj_dir_path, current_src)?;

        let mut loaded = load_docchi_file(&first_save_path, &info, false)?;

        let p = RootObjectPtr::new(&mut loaded);
        let b = get_bool(p, "b")?;
        assert_eq!(b.value().unwrap(), &true);

        let i = get_int(p, "int")?;
        assert_eq!(i.value().unwrap(), &10);

        let a = get_int(p, "added")?;
        assert_eq!(a.value().unwrap(), &100);
    }

    for file in list_files(proj_dir_path)?{
        println!("{} len {}", file.name(), file.len());
    }


    Ok(())
}
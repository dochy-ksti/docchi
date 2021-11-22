[prev](b1_save_docchi_files_save_docchi_files.md.md)
[index](index.md)
[next](b1_save_docchi_files_load_docchi_file.rs.md)

### 6-1. Save Docchi File

```rust
use docchi::error::DpResult;
use docchi::fs::filesys::{SaveDirInfo, save_docchi_file};
use docchi::fs::common::CurrentSrc;
use crate::b1_save_docchi_files::load_docchi_file::load_docchi_file_test;
use crate::b1_save_docchi_files::save_docchi_files_accessor::RootIntf;

#[test]
fn save_docchi_file_test() -> DpResult<()>
{
    let save_dir = "src/b1_save_docchi_files/save_dir";

    // make sure the save_dir exists.
    std::fs::create_dir(save_dir).ok();

    let src_dir = "src/b1_save_docchi_files/src_dir";

    // You need SaveDirInfo to save Docchi Data files.
    // save_dir and src_dir's paths are needed to create SaveDirInfo.
    let info = SaveDirInfo::create(save_dir, CurrentSrc::from_src_dir(src_dir))?;

    // SaveDirInfo has a RootObject created from "src_dir"

    // We can clone RootObject instantly, because RootObject consists of Arcs (Arc means Atomic-Reference-Count Pointer)
    let root = info.clone_src_root();

    let mut root = RootIntf::new(root);
    root.set_message("Hello the next world".to_string());

    // To save Docchi Data file, use "save_docchi_file" function.
    // Docchi Data file is often abbreviated to "docchi file".
    let _saved_path = save_docchi_file(
        &info, // SaveDirInfo to specify save_dir, and Docchi Src
        "next_world.docchi", // filename
        root.root_obj_ref(), // the reference of the data. We can get it from "RootIntf::root_obj_ref"
        /* overwrite flag */true)?;

    // SaveDirInfo contains RootObject created from Docchi Src
    // RootIntf contains modified RootObject, and "root_obj_ref()" can get the reference
    // "save_docchi_file" saves the difference of the two.
    // (The difference is "Hello World" -> "Hello the next world")

    // Saving completed. Now let's check out what we just created.

    // save_dir - 80a3e5062f0fbeede35cab8cab5d0826 ┬- created_time.dat
    //                                             ├- next_world.docchi
    //                                             └- src.archive

    // "80a3e5062f0fbeede35cab8cab5d0826" is the hash code created from Docchi Src. It's 128 bit hex value and the algorithm is MetroHash.
    //
    // "src.archive" is the file which archives the Docchi Src. Archiving algorithm is original (Docchi Archiver2) and the archiver also calculates the hash.
    // "created_time.dat" is the file to store when this directory is created. (Copying directories often confuses the OS-managed created-time property, so we decided to manage it ourself).
    // "next_world.docchi" is the save file. We designated the filename.

    // Docchi File (and Docchi History) file is placed with "src.archive", which is the Docchi Src the file is originated from.
    // They are placed in a hash-named directory calculated from the Src.
    // When the Src is modified, new hash is calculated and new "src.archive" is created in the new hash-directory and new save files are placed in it.
    // When an old data file is loaded, the data is composed from the correspond old "src.archive" placed in the same directory,
    // and converted to the new version automatically referencing the new version of Docchi Src.

    // We'll talk about the conversion later.

    // There's very little chance that some 128 bit hashes accidentally collides.
    // Should it happen, append the space character at tha end of the Docchi Src and change the hash.

    // Now let's see how to load.

    load_docchi_file_test()?;

    Ok(())
}


```


[prev](b1_save_docchi_files_save_docchi_files.md.md)
[index](index.md)
[next](b1_save_docchi_files_load_docchi_file.rs.md)

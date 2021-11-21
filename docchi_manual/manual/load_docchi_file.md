[prev](save_docchi_file_test.md)
[index](index.md)
[next](whats_docchi_history.md)

### 6-2. Load And Remove Docchi File

```rust
use docchi::error::DpResult;
use docchi::fs::filesys::{SaveDirInfo, list_docchi_files, load_docchi_file};
use docchi::fs::common::{CurrentSrc, hash_dir_path};
use crate::b1_save_docchi_files::save_docchi_files_accessor::RootIntf;

pub(crate) fn load_docchi_file_test() -> DpResult<()> {
    let save_dir = "src/b1_save_docchi_files/save_dir";
    let src_dir = "src/b1_save_docchi_files/src_dir";
    let info = SaveDirInfo::create(save_dir, CurrentSrc::from_src_dir(src_dir))?;

    // Gets the file list.
    let files = list_docchi_files(save_dir)?;

    // Finds the file to load.
    let file = files.iter().find(|f| f.name() == "next_world.docchi").unwrap();

    // "file" needs "save_dir" to calculate the file path. (It's not ergonomic, but efficient...
    let file_path = file.calc_path(save_dir);

    // To load a Docchi Data file, use "load_docchi_file"
    let root = load_docchi_file(
        &file_path,
        &info,
        /* validation flag */true
    )?;
    let root = RootIntf::new(root);

    // Make sure the message is modified.
    assert_eq!(root.message(), "Hello the next world");

    // You can remove Docchi files directly.
    std::fs::remove_file(&file_path).unwrap();

    // You can remove entire hash_dir if you want to
    let hash_dir = hash_dir_path(save_dir, info.hash());
    std::fs::remove_dir_all(hash_dir).unwrap();

    // Removing system managed files(src.archive, created_time.dat) is unsafe,
    // but removing an entire hash directory is safe
    Ok(())
}
```


[prev](save_docchi_file_test.md)
[index](index.md)
[next](whats_docchi_history.md)

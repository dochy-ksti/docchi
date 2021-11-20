[prev](save_dochy_file_test.md)
[index](index.md)
[next](whats_dochy_history.md)

### 6-2. Load And Remove Dochy File

```rust
use docchi::error::DpResult;
use docchi::fs::filesys::{SaveDirInfo, list_dochy_files, load_dochy_file};
use docchi::fs::common::{CurrentSrc, hash_dir_path};
use crate::b1_save_dochy_files::save_dochy_files_accessor::RootIntf;

pub(crate) fn load_dochy_file_test() -> DpResult<()> {
    let save_dir = "src/b1_save_dochy_files/save_dir";
    let src_dir = "src/b1_save_dochy_files/src_dir";
    let info = SaveDirInfo::create(save_dir, CurrentSrc::from_src_dir(src_dir))?;

    // Gets the file list.
    let files = list_dochy_files(save_dir)?;

    // Finds the file to load.
    let file = files.iter().find(|f| f.name() == "next_world.dochy").unwrap();

    // "file" needs "save_dir" to calculate the file path. (It's not ergonomic, but efficient...
    let file_path = file.calc_path(save_dir);

    // To load a Dochy Data file, use "load_dochy_file"
    let root = load_dochy_file(
        &file_path,
        &info,
        /* validation flag */true
    )?;
    let root = RootIntf::new(root);

    // Make sure the message is modified.
    assert_eq!(root.message(), "Hello the next world");

    // You can remove Dochy files directly.
    std::fs::remove_file(&file_path).unwrap();

    // You can remove entire hash_dir if you want to
    let hash_dir = hash_dir_path(save_dir, info.hash());
    std::fs::remove_dir_all(hash_dir).unwrap();

    // Removing system managed files(src.archive, created_time.dat) is unsafe,
    // but removing an entire hash directory is safe
    Ok(())
}
```


[prev](save_dochy_file_test.md)
[index](index.md)
[next](whats_dochy_history.md)

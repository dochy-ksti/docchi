## Hello World

Docchi is a language, so we need "Hello World".
```json5
//root.json5
{
  message : "Hello World"
}
```
Docchi's source file is written in JSON5 which is a better JSON.

The file name must be "root.json5",

A dedicated directory to place the file is needed. 
(The directory shines when the number of source files is increased, but for now, that's definitely unnecessary)
```
proj_dir ┬ src_dir-root.json5
         └ save_dir
```
Let's convert the source file into a Docchi's object.
```Rust
use docchi::error::DpResult; //DpResult is the Docchi's error type.
use docchi::core::structs::RootObject;
use docchi::core::json_dir_to_root;

#[test]
fn hello_world() -> DpResult<()> {
    let _root_obj : RootObject = json_dir_to_root("src/a1_hello_world/some_dir", true)?;
    Ok(())
}
```
The function "json_dir_to_root" converts the source file to "RootObject". 
Designating directory's path is needed.

"RootObject" has low-level interfaces, 
but generally you need to generate the source code to access the converted object.
```Rust
use docchi::error::DpResult;
use docchi::core::structs::RootObject;
use docchi::core::json_dir_to_root;
use docchi::intf::generate_interface;

#[test]
fn hello_world_generate() -> DpResult<()> {
    let mut root_obj : RootObject = json_dir_to_root("src/a1_hello_world/some_dir", true)?;

    let ans = generate_interface(&mut root_obj);
    std::fs::write(
        "src/a1_hello_world/hello_world_accessor.rs",
        &ans.to_string(),
    ).unwrap();
    Ok(())
}
```
"generate_interface" function generates the source code to access the object.

You need to copy the source and compile to use it,
so std::fs::write is used.

Let's use the generated source.
```Rust
use docchi::error::DpResult;
use docchi::core::json_dir_to_root;
use crate::a1_hello_world::hello_world_accessor::RootIntf;

#[test]
fn hello_world_test() -> DpResult<()> {
    let root = json_dir_to_root("src/a1_hello_world/some_dir", true)?;

    let mut root = RootIntf::new(root);
    assert_eq!(root.message(), "Hello World");
    root.set_message("Hello the next world".to_string());
    assert_eq!(root.message(), "Hello the next world");
    Ok(())
}
```
"RootIntf" is the generated accessor.
"message" gets the message, and "set_message" sets the message.

We modified the data, then saved the data to a file.
```Rust
fn hello_world_save_test() -> DpResult<()> {
    let root = json_dir_to_root("src/a1_hello_world/some_dir", true)?;

    let mut root = RootIntf::new(root);
    root.set_message("Hello the next world".to_string());

    let saved_path = save_file(
        "src/a1_hello_world/save_dir",
        unsafe{ root.root_obj_ref() },
        &CurrentSrc::SrcDir(PathBuf::from("src/a1_hello_world/some_dir")),
        "next_world.docchi",
        true)?;

    println!("{:?}", &saved_path);

    Ok(())
}
```
The saved_path is 
"src/a1_hello_world/save_dir/7ddf0e5d6e73619ba205e3c47575905e/next_world.docchi".

"7ddf0e5d6e73619ba205e3c47575905e" is the hash of the source file.

Why we need such a directory is hard to explain, so we ignore it for now.

Let's find the file and load it.
```Rust
use docchi::error::DpResult;
use docchi::core::json_dir_to_root;
use crate::a1_hello_world::hello_world_accessor::RootIntf;
use docchi::fs::filesys::{save_file, load_saved_file, list_saved_files};
use docchi::fs::common::CurrentSrc;
use std::path::PathBuf;

#[test]
fn hello_world_load_test() -> DpResult<()> {
    let save_dir = "src/a1_hello_world/save_dir";
    let files = list_saved_files(save_dir)?;

    let file = files.iter().find(|f| f.name() == "next_world.docchi")?;

    let root = load_saved_file(
        file.calc_path(save_dir),
        &CurrentSrc::SrcDir(PathBuf::from("src/a1_hello_world/some_dir")),
        true
    )?;
    let root = RootIntf::new(root);
    assert_eq!(root.message(), "Hello the next world");

    Ok(())
}
```
"list_saved_file" lists the files, and "load_saved_file" loads the file.
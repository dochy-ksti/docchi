An archiver which archives a folder with the Snappy compression, 
and calculate 128bit hash from the metadata of the files.

Reading actual files doesn't need to calculate the hash. The hash algorithm is MetroHash.

It is one of the components of the Dochy File System.

```Rust
fn main() -> ArcResult<()>{
    let mut buf: Vec<u8> = vec![];
    let r = create_archive_from_directory(
        "foo/some_dir_path", &mut buf,
        |hash| {
            //No files are modified when the same hash already exists,
            //so you can safely cancel the archiving process
            Path::new("bar/some_dir2").join(format!("{}", hash)).exists()
        }, &ArchiveOptions::new())?;
    
    let path = match r{
        CreateArchiveFromDirectory::WrittenSuccessfully(_size, hash) =>{
            let path = Path::new("bar/some_dir2").join(format!("{}", hash));
            //save the hash and the archived data
            let mut file = File::create(path)?;
            file.write_all(&buf);
            path
        },
        CreateArchiveFromDirectory::Canceled(hash) =>{
            Path::new("bar/some_dir2").join(format!("{}", hash));
        }
    };

    //archive file exists either way
    let mut archive_file = File::open(path)?;
    let archive_data = read_archive_data(&mut archive_file)?;
    
    //resume the archive to the target directory
    resume_archive("baz/target_dir", &archive_data, true)?;
    Ok(())
}
```

## changelog

0.3.2 & 0.3.3
- fixed readme.

0.3.1
- fixed the mistakes from 0.3.0 and changed the signatures of some functions again.

0.3.0
- added documentation.
- changed the signatures of the many public functions and hid some implementation details.

0.2.0
- completely overhauled and now this lib can be used as a general purpose archiver

0.1.3
- updated readme


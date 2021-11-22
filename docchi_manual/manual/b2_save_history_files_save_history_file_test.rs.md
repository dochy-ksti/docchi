[prev](b2_save_history_files_whats_docchi_history.md.md)
[index](index.md)
[next](b2_save_history_files_load_history_file_test.rs.md)

### 7-1. Save History File

```rust
use docchi::error::DpResult;
use docchi::fs::common::{CurrentSrc, hash_dir_path};
use docchi::fs::history::{HistoryInfo, save_history_file, HistoryOptionsBuilder, CumulativeOptionsBuilder};
use crate::b2_save_history_files::save_history_files_accessor::RootIntf;
use std::path::Path;
use crate::b2_save_history_files::load_history_file_test::load_history_file_test;

//#[test]
fn save_history_file_test() -> DpResult<()> {
    let history_dir = "src/b2_save_history_files/history_dir";

    // initialize history_dir
    std::fs::remove_dir_all(history_dir).ok();
    std::fs::create_dir(history_dir).unwrap();

    let src_dir = "src/b2_save_history_files/src_dir";

    // HistoryInfo is needed for save/load Docchi History files
    let info = HistoryInfo::create(history_dir,
                                   CurrentSrc::from_src_dir(src_dir),
                                   HistoryOptionsBuilder::new()
                                       .max_phase(2)
                                       .cumulative(Some(CumulativeOptionsBuilder::new()
                                           .limit_count(Some(3)) //Sets limit_count 3 (default: 10)
                                           .limit_nth(Some(1)))) // Sets limit_nth 1 (default : 1, so this line is not needed)
                                       .build()?)?;

    // HistoryOptions is linked to history_dir at the first creation of HistoryInfo with the history_dir
    // You can create HistoryInfo twice, but changing the options is forbidden
    // CurrentSrc is also linked and unchangeable.

    let root = info.clone_src_root();
    let mut root = RootIntf::new(root);
    let mut count = 0;

    modify(&mut root, &mut count);
    let _file = save_history_file(&info, None, root.root_obj_ref())?;

    let hash_dir = hash_dir_path(history_dir, info.hash());
    print_dir(&hash_dir)?;

    // Let's see what we created.
    //
    // created_time.dat 12 bytes
    // src.archive 130 bytes
    // _0_0.his 28 bytes

    // "crated_time.dat" and "src.archive" were already explained.
    // "_0_0.his" is the history file we just created.
    // ".his" is the Docchi History file's extension.
    //
    // The first number "_0" is what we call "control number". It's appended to make the filename unique.
    // the second "_0" is "phase-0 number". It's 0 because it's the first phase-0 file.
    // Phase-0 files only depend on Docchi Src. It's almost the same as Docchi Data files.
    // Docchi History files have some metadata, so they are not identical, though.

    modify(&mut root, &mut count);
    let _file = save_history_file(&info, None, root.root_obj_ref())?;
    print_dir(&hash_dir)?;

    // "_0_0_0.his 33 bytes" is just created.
    // It means "control 0 phase-0 0 phase-1 0"
    // The last number is phase-1, so this file is a phase-1 file.
    // Phase-1 files are derived from phase-0 files.
    // The parent file can be specified from the filename.
    // In this case, it's "_0_0" (control 0, phase-0 0).
    // Phase numbers other than the last number is always identical with the parent's phase numbers,
    // and the control number is also basically the same with the parent's.
    //
    // The parent file is "_0_0.his", so Docchi calculated diff from "_0_0.his", and save it as "_0_0_0.his"
    //
    // Each time, the modification is always "appending 10 bytes string".
    // So the size of diff is basically the same.
    // But the more the phase is increased, the more data is needed in the metadata,
    // so the file size is slightly increased each time.

    modify(&mut root, &mut count);
    let _file = save_history_file(&info, None, root.root_obj_ref())?;
    print_dir(&hash_dir)?;

    // "_0_0_0_0.his 37 bytes" is just created.
    // It means "control 0 phase-0 0 phase-1 0 phase-2 0"
    // This is the first phase-2 file, and the parent file is "_0_0_0.his".
    //
    // Docchi Calculated diff from "_0_0_0.his".
    // It means, theoretically, first, Docchi created a RootObject from Docchi Src,
    // and Docchi opened "_0_0.his", and applied the diff to the object,
    // and opened "_0_0_0.his", and applied the diff to the object,
    // and calculated diff from the object to the current object, and save it as "_0_0_0_0.his".
    //
    // Actually, Docchi has cache data of them. The latest phase files are always cached(and Docchi Src, too),
    // so Docchi Src, phase-0, phase-1 files are already cached.
    // So Docchi did compare the phase-1 cache data to the current object, and save it as "_0_0_0_0.his".
    //
    // We set max_phase 2 in the HistoryOptions, so phase-2 is the max phase.
    //
    // "_0_0_0_0.his" is 37 bytes, so there's 37 bytes in the max_phase. The total file size of the the max_phase is currently 37.
    // (Actually, most of the 37 bytes is metadata, and the total file size must be calculated without metadata,
    // so the statement above is very inaccurate)

    // We set limit_nth "1". so the max_phase is limited by the most largest file in its ancestors.
    // The most largest file in its ancestors is "_0_0_0.his 33 bytes".
    // The total file size of the max phase is already larger than the most largest file,
    // so Docchi considers the max phase is too large.

    modify(&mut root, &mut count);
    let _file = save_history_file(&info, None, root.root_obj_ref())?;
    print_dir(&hash_dir)?;

    // "_0_1.his 77 bytes" is just created. It means "control 0 phase-0 1"
    // When max_phase is overflowed, Docchi calculates which phase the next file should be shifted to( [algorithm](...) ),
    // and the file is shifted to the phase-0.
    // The diff is calculated from the Docchi Src, and saved as "0_1.his".
    //
    // The data is modified four times. Each time, 10 bytes string is appended, so theoretically, the total diff size is 40 bytes.
    // The actual file size is 77 bytes. It's kind of bloated?
    // "0_0.his" was 28 bytes, and the data is modified three times since then. the diff is theoretically 30 bytes,
    // so you may think it should be 58 bytes.
    //
    // Actually, Docchi doesn't compare values without Int, Float and Bool.
    // Docchi doesn't compare strings and create diff of strings.
    // Docchi only confirm whether it's modified, and if it's modified, the entire value is saved.
    // The initial string is 5 bytes, and appended 10 bytes, so the diff is actually 15 bytes.
    //
    // It modified three times so the total diff is 45 bytes.
    // "0_0.his" was 28 bytes and "_0_1.his" was 77 bytes.
    // It should be theoretically 73 bytes. The difference is 4 bytes.
    //
    // Docchi encodes numbers to variable bytes.
    // When a number is increased, the data size can be grown. Maybe the size grown was 4 bytes.

    modify(&mut root, &mut count);
    let _file = save_history_file(&info, None, root.root_obj_ref())?;
    print_dir(&hash_dir)?;

    // "_0_1_0.his 33 bytes" is just created. It means "control 0 phase-0 1 phase-1 0".
    // The parent is "_0_1.his", and it is the first phase-1 file which has the parent "_0_1.his",
    // so it's called "0_1_0.his".
    //
    // The size is 33 bytes. "0_0_0.his" was 33 bytes, so they have the same size.

    modify(&mut root, &mut count);
    let _file = save_history_file(&info, None, root.root_obj_ref())?;
    print_dir(&hash_dir)?;

    // "_0_1_0_0.his 37 bytes" is just created. It means "control 0 phase-0 1 phase-1 0 phase-2 0".
    // The parent is "_0_1_0.his", and it is the first phase-2 file which has the parent "_0_1_0.his",
    // so it's called "0_1_0_0.his".
    //
    // The size is 37 bytes. "0_0_0_0.his" was 37 bytes, so they have the same size.

    modify(&mut root, &mut count);
    let _file = save_history_file(&info, None, root.root_obj_ref())?;
    print_dir(&hash_dir)?;

    // "_0_1_0_1.his 38 bytes" is just created. It means "control 0 phase-0 1 phase-1 0 phase-2 1".
    // The second phase-2 file is created, and the phase-2 is the "max_phase".
    // When "cumulative" option is Some, max_phase will be "cumulative".
    // When it's cumulative, the parent of a file is the previous file in max_phase.
    // We enabled the option, so the parent of "_0_1_0_1.his" is "_0_1_0_0.his".
    //
    // If "cumulative" option is off, the parent of "_0_1_0_1.his" is "_0_1_0.his".
    // The modification occurred twice since then, so the file size would be much bigger.
    // (I don't think "cumulative off" has useful use cases, so I might get rid of the option.)
    //
    // The "max_phase" is limited by the most largest file size in its ancestors.
    // It's "_0_1.his 77 byes".
    // The sum of "_0_1_0_0.his 37 bytes" and "_0_1_0_1.his 38 bytes" is 75 bytes.
    // It's smaller than the largest ancestor, so "max_phase" is not overflown yet.
    // (As I said before, this statement is very inaccurate...)

    modify(&mut root, &mut count);
    let _file = save_history_file(&info, None, root.root_obj_ref())?;
    print_dir(&hash_dir)?;

    // "_0_1_0_2.his 40 bytes" is just created.
    // Comparing it with "_0_1_0_0.his 37 bytes" and "_0_1_0_1.his 38 bytes",
    // they are cumulative, so the file size is almost the same,
    // but the file size is slightly increasing with the metadata grown.
    //
    // The max_phase is also limited by the option "limit_count".
    // We set the limit '3", so a max_phase must have less than three files.
    // "_0_1_0_2.his" is the third file of the max_phase, so Docchi thinks the max_phase is overflown.

    modify(&mut root, &mut count);
    let _file = save_history_file(&info, None, root.root_obj_ref())?;
    print_dir(&hash_dir)?;

    // "_0_1_1.his 101 bytes" is just created.
    // The parent is "_0_1.his 77 bytes". It's rare that a younger phase file has bigger file size.
    // It's modified five times, so the total diff size is 75 bytes.

    modify(&mut root, &mut count);
    let _file = save_history_file(&info,
                                  Some(format!("You_Can_Append_Tag")),
                                  root.root_obj_ref())?;
    print_dir(&hash_dir)?;

    // "#You_Can_Append_Tag#_0_1_1_0.his 40 bytes" is just created.
    // You can append a tag to a history file to characterize it.

    // Let's move on to loading, and we can explain saving more with it.
    load_history_file_test()?;

    Ok(())
}

pub(crate) fn modify(root : &mut RootIntf, count : &mut usize){
    let c = *count % 10;
    *count += 1;
    let m = match c{
        0 => root.d0_mut(),
        1 => root.d1_mut(),
        2 => root.d2_mut(),
        3 => root.d3_mut(),
        4 => root.d4_mut(),
        5 => root.d5_mut(),
        6 => root.d6_mut(),
        7 => root.d7_mut(),
        8 => root.d8_mut(),
        9 => root.d9_mut(),
        _ => unreachable!(),
    };
    m.push_str("0123456789");
}

pub(crate) fn print_dir<P : AsRef<Path>>(dir : P) -> DpResult<()>{
    for entry in std::fs::read_dir(dir)?{
        let entry = entry?;
        let name = entry.file_name().to_str().unwrap().to_string();
        let len = entry.metadata().unwrap().len();
        println!("{} {} bytes", name, len);
    }
    Ok(())
}
```


[prev](b2_save_history_files_whats_docchi_history.md.md)
[index](index.md)
[next](b2_save_history_files_load_history_file_test.rs.md)

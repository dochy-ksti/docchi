// use crate::{ArcResult, ArchiveOptions, resume_archive, read_archive_data, ArchiveOptionsBuilder};
// use crate::imp::create_archive_from_directory::CreateArchiveFromDirectory;
// use anyhow::anyhow;
//
// //#[test]
// fn general_to_simple_test() -> ArcResult<()>{
//     let mut buf : Vec<u8> = Vec::new();
//     if let CreateArchiveFromDirectory::WrittenSuccessfully(_a,_b) =
//     crate::imp::create_archive_from_directory::create_archive_from_directory(
//         "./src/json/general", &mut buf, |_| false, &ArchiveOptions::from(ArchiveOptionsBuilder{
//             archive_subfolders : false,
//             extensions_archived : vec!["json5"],
//         })?)?{
//
//         let archive = read_archive_data(&mut buf.as_slice())?;
//
//         resume_archive("./src/output/general_to_simple", &archive, true)?;
//         Ok(())
//     } else{
//         unreachable!()
//     }
// }
//

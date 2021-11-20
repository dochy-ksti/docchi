// use crate::{ArcResult, ArchiveOptions, resume_archive, read_archive_data};
// use crate::imp::create_archive_from_directory::CreateArchiveFromDirectory;
// use anyhow::anyhow;
//
// //#[test]
// fn resume_test() -> ArcResult<()>{
//     let mut buf : Vec<u8> = Vec::new();
//     if let CreateArchiveFromDirectory::WrittenSuccessfully(_a,_b) =
//     crate::imp::create_archive_from_directory::create_archive_from_directory(
//         "./src/json/general", &mut buf, |_| false, &ArchiveOptions::new())?{
//
//         let archive = read_archive_data(&mut buf.as_slice())?;
//
//         resume_archive("./src/output/general/", &archive, true)?;
//         Ok(())
//     } else{
//         unreachable!()
//     }
// }
//

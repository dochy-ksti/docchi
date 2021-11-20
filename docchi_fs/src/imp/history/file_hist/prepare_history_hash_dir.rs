// use std::path::{Path, PathBuf};
// use crate::common::CurrentSrc;
// use crate::error::FsResult;
// use dochy_archiver::{create_archive_from_directory, CreateArchiveFromDirectory, get_hash_and_metadata_from_archive};
// use crate::imp::common::path::hash_dir_path::hash_dir_path;
// use crate::imp::common::archive::find_or_make_and_read_archive::{make_archive_if_not_exist, BufOrPath};
// use std::fs::File;
// use crate::imp::common::archive::archive_opt::JSON_ARC_OPT;
// use std::ops::Deref;
//
// pub(crate) fn prepare_history_hash_dir(history_dir: &Path, src : &CurrentSrc) -> FsResult<(PathBuf, u128)>{
//
//     let (history_hash_dir, hash) = match src {
//         CurrentSrc::SrcDir(src_dir) => {
//             let mut buf : Vec<u8> = vec![];
//
//             let r = create_archive_from_directory(
//                 src_dir, &mut buf,
//                 |src_hash| {
//                     hash_dir_path(history_dir, src_hash).exists()
//                 }, JSON_ARC_OPT.deref())?;
//             match r {
//                 CreateArchiveFromDirectory::Canceled(hash) => {
//                     make_archive_if_not_exist(history_dir, hash, BufOrPath::SrcDir(src_dir))?;
//                     (hash_dir_path(history_dir, hash), hash)
//                 },
//                 CreateArchiveFromDirectory::WrittenSuccessfully(_, hash) => {
//                     make_archive_if_not_exist(history_dir, hash, BufOrPath::Buf(&buf))?;
//                     (hash_dir_path(history_dir, hash), hash)
//                 }
//             }
//         },
//         CurrentSrc::ArchiveFile(archive_path) =>{
//             let (hash,_) = {
//                 let mut file = File::open(archive_path)?;
//                 get_hash_and_metadata_from_archive(&mut file)?
//             };
//             make_archive_if_not_exist(history_dir, hash, BufOrPath::ArchivePath(archive_path))?;
//             (hash_dir_path(history_dir, hash), hash)
//         }
//     };
//     Ok((history_hash_dir, hash))
// }
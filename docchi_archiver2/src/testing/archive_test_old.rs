// use crate::{ArcResult, ArchiveOptions};
// use crate::imp::create_archive_from_directory::CreateArchiveFromDirectory;
// use anyhow::anyhow;
//
// //#[test]
// fn archive_test() -> ArcResult<()>{
//     let mut buf : Vec<u8> = Vec::new();
//     if let CreateArchiveFromDirectory::WrittenSuccessfully(_a,_b) =
//     crate::imp::create_archive_from_directory::create_archive_from_directory(
//         "./src/json/simple", &mut buf, |_| false, &ArchiveOptions::new())?{
//
//         println!("buflen {}", buf.len());
//         let archive_data = crate::read_archive_data(&mut buf.as_slice())?;
//         println!("{:#?}", archive_data);
//         Ok(())
//     } else{
//         unreachable!()
//     }
// }
//
// //最後にだけflushするようにしたところさらに少し縮まった
//
// //first_len 62
// //len 10 veclen() 62
// //len 7 veclen() 62
// //len 31 veclen() 62
// //len 5449 veclen() 62
// //veclen() 3028
//
// //毎回encoderをnewすると多少ファイルサイズが膨らむので、(そもそもencoderの初期化処理も入ってしまうし)
// //一気にやったほうがよろしいようだ
// //小さいデータは圧縮すると大きくなるが、やはりある程度大きければ縮むようだ
//
// //first_len 62
// //len 10 veclen() 90
// //len 7 veclen() 105
// //len 31 veclen() 144
// //len 5449 veclen() 3052
//
// //first vec len 62
// //len 10 vec.len 90
// //len 7 vec.len 115
// //len 31 vec.len 164
// //len 5449 vec.len 3082
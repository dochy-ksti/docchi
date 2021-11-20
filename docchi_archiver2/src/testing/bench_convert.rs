//
// #[cfg(test)]
// mod tests {
//     //use rand::prelude::ThreadRng;
//     //use rand::Rng;
//
//     use test::Bencher;
//     use std::time::Instant;
//
//     //100MB
//     const SIZE : usize = 10_000_000;
//
//     fn init_vec() -> Vec<u8> {
//         let mut vec : Vec<u8> = Vec::with_capacity(SIZE);
//
//         for i in 0..SIZE{
//             vec.push(i as u8);
//         }
//
//         vec
//     }
//
//     fn init_vec_u64() -> Vec<u64> {
//         let mut vec : Vec<u64> = Vec::with_capacity(SIZE);
//
//         for i in 0..SIZE{
//             vec.push(i as u64);
//         }
//
//         vec
//     }
//
//     #[test]
//     fn test_time(){
//         let now = Instant::now();
//
//         //bencherのns/iterがかなりかかっていて信じられないので普通に時間を測ってみる
//         let source = init_vec();
//         let mut sum = 0;
//
//         for _ in 0..10 {
//             let mut vec: Vec<u8> = Vec::with_capacity(source.len());
//             for i in &source {
//                 vec.push(*i + 1);
//             }
//
//             let mut vec2: Vec<u8> = Vec::with_capacity(vec.len());
//             for i in &vec {
//                 vec2.push(*i + 1);
//             }
//
//             sum += vec2[vec2[1] as usize] as u8;
//         }
//
//         println!("{} milliseconds {}", now.elapsed().as_millis(), sum);
//     }
//
//
//     #[test]
//     fn test_time_u64(){
//         let now = Instant::now();
//
//         let source = init_vec_u64();
//         let mut sum = 0;
//
//         for _ in 0..10 {
//             let mut vec: Vec<u64> = Vec::with_capacity(source.len());
//             for i in &source {
//                 vec.push(*i + 1);
//             }
//
//             let mut vec2: Vec<u64> = Vec::with_capacity(vec.len());
//             for i in &vec {
//                 vec2.push(*i + 1);
//             }
//
//             sum += vec2[vec2[1] as usize];
//         }
//
//         println!("{} milliseconds {}", now.elapsed().as_millis(), sum);
//     }
//
//     #[test]
//     fn test_time_clone(){
//         let now = Instant::now();
//
//         let source = init_vec_u64();
//         let mut sum = 0;
//
//         for _ in 0..10 {
//             let vec: Vec<u64> = source.clone();
//
//
//             let vec2: Vec<u64> = vec.clone();
//
//             sum += vec2[vec2[1] as usize];
//         }
//
//         println!("clone {} milliseconds {}", now.elapsed().as_millis(), sum);
//     }
//
//     #[bench]
//     fn bench_convert_twice(b: &mut Bencher) {
//         //onceに対して概ね40% with_bufferと比べて50%遅い
//         let source = init_vec();
//
//         b.iter(|| {
//             let mut vec : Vec<u8> = Vec::with_capacity(source.len());
//             for i in &source{
//                 vec.push(*i + 1);
//             }
//
//             let mut vec2 : Vec<u8> = Vec::with_capacity(vec.len());
//             for i in &vec{
//                 vec2.push(*i + 1);
//             }
//         });
//     }
//
//     #[bench]
//     fn bench_copy_twice(b: &mut Bencher) {
//         let source = init_vec();
//
//         b.iter(|| {
//             let vec : Vec<u8> = source.to_vec();
// //            for i in &source{
//   //              vec.push(*i + 1);
//     //        }
//
//             vec.to_vec();
//             //for i in &vec{
//               //  vec2.push(*i + 1);
//             //}
//         });
//     }
//
//     fn read(buffer : &mut Vec<u8>, source : &Vec<u8>, source_position : usize) -> usize{
//         let mut counter = source_position;
//         loop {
//             let i = counter;
//             counter += 1;
//             let buffer_pos = i - source_position;
//
//             if source.len() <= i {
//                 return buffer_pos;
//             }
//
//             if buffer.len() <= buffer_pos {
//                 return buffer.len();
//             }
//
//             buffer[buffer_pos] = source[i] + 1;
//         }
//     }
//
//     #[bench]
//     fn bench_convert_with_buffer(b: &mut Bencher) {
//         let source = init_vec();
//         let buffer_size = 1024;
//
//         b.iter(|| {
//             let mut result_vec : Vec<u8> = Vec::with_capacity(source.len());
//             let mut buffer : Vec<u8> = Vec::with_capacity(buffer_size);
//             for _ in 0..buffer_size{
//                 buffer.push(0);
//             }
//
//             let mut counter = 0;
//             loop{
//                 let read_bytes = read(&mut buffer, &source, counter);
//                 if read_bytes == 0{ break; }
//                 counter += read_bytes;
//                 for i in 0..read_bytes{
//                     result_vec.push(buffer[i] + 1);
//                 }
//             }
//         });
//     }
//
//     #[bench]
//     fn bench_convert_once(b: &mut Bencher) {
//         let source = init_vec();
//
//         b.iter(|| {
//             let mut result_vec : Vec<u8> = Vec::with_capacity(source.len());
//
//             let mut inter_vec : Vec<u8> = vec![0];
//
//             for i in &source{
//                 inter_vec.push(*i + 1);
//                 result_vec.push(inter_vec.pop().unwrap() + 1);
//             }
//         });
//     }
//
// }
// #[cfg(test)]
// mod tests {
//     use crate::HashM, LinkedList};
//
//     use rand::prelude::ThreadRng;
//     use rand::Rng;
//
//     use test::Bencher;
//     use linked_hash_map::LinkedHashMap;
//
//     const NAME_LEN : usize = 5;
//     const NUM_NAMES : usize = 10;
//     const REPEAT : usize = 100;
//
//     ///56バイトだとBoxに入れる価値はない
//     /// 168バイトだとBoxに入れる価値大いにある
//     struct BigItem {
//         map1: HashMap<String, usize>,
//         map2: HashMap<String, usize>,
//         map3: HashMap<String, usize>,
//     }
//
//     struct BigItem2 {
//         map1: HashMap<Box<str>, usize>,
//         map2: HashMap<Box<str>, usize>,
//         map3: HashMap<Box<str>, usize>,
//     }
//
//     struct BigItem3 {
//         map1: Box<HashMap<String, usize>>,
//         map2: Box<HashMap<String, usize>>,
//         map3: Box<HashMap<String, usize>>,
//     }
//
//     fn init1() -> (ThreadRng, Vec<String>) {
//         let mut rng = rand::thread_rng();
//
//         let mut names : Vec<String> = vec![];
//
//         for _ in 0..NUM_NAMES{
//             names.push(get_random_name(NAME_LEN, rng));
//         }
//
//         (rng, names)
//     }
//
//     fn init2() -> (ThreadRng, Vec<Box<str>>) {
//         let mut rng = rand::thread_rng();
//
//         let mut names : Vec<Box<str>> = Vec::with_capacity(NUM_NAMES);
//
//         for _ in 0..NUM_NAMES{
//             names.push(get_random_name(NAME_LEN, rng).into_boxed_str());
//         }
//
//         (rng, names)
//     }
//
//     fn get_random_name(len : usize, mut rng : ThreadRng) -> String{
//         let mut s = String::with_capacity(len);
//         for i in 0..len {
//             s.push(rng.gen_range('a' as u8, 'z' as u8) as char );
//         }
//         return s;
//     }
//
//
//
//     #[bench]
//     fn bench_init_linked_hash_map(b: &mut Bencher) {
//         let (rng, names) = init1();
//
//
//         b.iter(|| {
//             let mut linked = LinkedList::new();
//             for _ in 0..REPEAT {
//                 linked.push_back(init_linked_hash_map(&names))
//             }
//             linked
//         });
//     }
//
//
//
//     #[bench]
//     fn bench_init_hash_map(b: &mut Bencher) {
//         let (rng, names) = init1();
//
//         b.iter(|| {
//             let mut linked = LinkedList::new();
//             for _ in 0..REPEAT {
//                 linked.push_back(init_hash_map(&names))
//             }
//             linked
//         })
//     }
//
//     #[bench]
//     fn bench_init_hash_map2(b: &mut Bencher) {
//         let (rng, names) = init1();
//
//         b.iter(||{
//             let mut linked = LinkedList::new();
//             for _ in 0..REPEAT {
//                 linked.push_back(init_hash_map(&names))
//             }
//             linked
//         })
//     }
//
//
//     #[bench]
//     fn bench_init_hash_map_with_box(b: &mut Bencher) {
//         let (rng, names) = init1();
//
//         b.iter(||{
//             let mut linked = LinkedList::new();
//             for _ in 0..REPEAT {
//                 linked.push_back(init_hash_map_with_box(&names))
//             }
//             linked
//         });
//     }
//
//     #[bench]
//     fn bench_init_hash_map_string_to_boxed_slice(b: &mut Bencher) {
//         let (rng, names) = init1();
//
//         b.iter(||{
//             let mut linked = LinkedList::new();
//             for _ in 0..REPEAT {
//                 linked.push_back(init_hash_map_string_to_boxed_slice(&names))
//             }
//             linked
//         });
//     }
//
//     #[bench]
//     fn bench_init_hash_map_boxed_slice(b: &mut Bencher) {
//         let (rng, names) = init2();
//
//         b.iter(||{
//             let mut linked = LinkedList::new();
//             for _ in 0..REPEAT {
//                 linked.push_back(init_hash_map_boxed_slice(&names))
//             }
//             linked
//         });
//     }
//
//     #[bench]
//     fn bench_init_hash_map_2boxes(b: &mut Bencher) {
//         let (rng, names) = init2();
//
//
//         b.iter(||{
//             let mut linked = LinkedList::new();
//             for _ in 0..REPEAT {
//                 linked.push_back(init_hash_map_2boxes(&names))
//             }
//             linked
//         });
//     }
//
//     #[bench]
//     fn bench_init_hash_map_boxed_item(b: &mut Bencher) {
//         let (rng, names) = init1();
//
//         b.iter(||{
//             let mut linked = LinkedList::new();
//             for _ in 0..REPEAT {
//                 linked.push_back(init_hash_map_boxed_item(&names))
//             }
//             linked
//         });
//     }
//
//
//     fn init_linked_hash_map(names : &Vec<String>) -> LinkedHashMap<String, BigItem>{
//         let mut map : LinkedHashMap<String, BigItem> = LinkedHashMap::with_capacity(names.len());
//
//         for name in names{
//             map.insert(name.to_string(), BigItem{ map1 : make_hash(names), map2 : make_hash(names), map3 : make_hash(names) });
//         }
//
//         return map;
//     }
//
//
//     fn init_hash_map(names : &Vec<String>) -> HashMap<String, BigItem>{
//         let mut map : HashMap<String, BigItem> = HashMap::with_capacity(names.len());
//
//         for name in names{
//             map.insert(name.to_string(), BigItem{ map1 : make_hash(names), map2 : make_hash(names), map3 : make_hash(names) });
//         }
//
//         return map;
//     }
//
//     fn init_hash_map_with_box(names : &Vec<String>) -> HashMap<String, Box<BigItem>>{
//         let mut map : HashMap<String, Box<BigItem>> = HashMap::with_capacity(names.len());
//
//         for name in names{
//             map.insert(name.to_string(), Box::new(BigItem{ map1 : make_hash(names), map2 : make_hash(names), map3 : make_hash(names) }));
//         }
//
//         return map;
//     }
//
//     fn init_hash_map_boxed_slice(names : &Vec<Box<str>>) -> HashMap<Box<str>, BigItem2>{
//         let mut map : HashMap<Box<str>, BigItem2> = HashMap::with_capacity(names.len());
//
//         for name in names{
//             map.insert(name.clone(), BigItem2{ map1 : make_hash2(names), map2 : make_hash2(names), map3 : make_hash2(names) });
//         }
//
//         return map;
//     }
//
//     fn init_hash_map_2boxes(names : &Vec<Box<str>>) -> HashMap<Box<str>, Box<BigItem2>>{
//         let mut map : HashMap<Box<str>, Box<BigItem2>> = HashMap::with_capacity(names.len());
//
//         for name in names{
//             map.insert(name.clone(), Box::new(BigItem2{ map1 : make_hash2(names), map2 : make_hash2(names), map3 : make_hash2(names) }));
//         }
//
//         return map;
//     }
//
//     fn init_hash_map_string_to_boxed_slice(names : &Vec<String>) -> HashMap<Box<str>, BigItem2>{
//         let mut map : HashMap<Box<str>, BigItem2> = HashMap::with_capacity(names.len());
//
//         for name in names{
//             map.insert(name.to_string().into_boxed_str(), BigItem2{ map1 : make_hash2_2(names), map2 : make_hash2_2(names), map3 : make_hash2_2(names) });
//         }
//
//         return map;
//     }
//
//     fn init_hash_map_boxed_item(names : &Vec<String>) -> HashMap<String, BigItem3>{
//         let mut map : HashMap<String, BigItem3> = HashMap::with_capacity(names.len());
//
//         for name in names{
//             map.insert(name.to_string(), BigItem3{ map1 : make_hash3(names), map2 : make_hash3(names), map3 : make_hash3(names) });
//         }
//
//         return map;
//     }
//
//     fn make_hash(names : &Vec<String>) -> HashMap<String, usize>{
//         let mut map : HashMap<String, usize> = HashMap::with_capacity(names.len());
//         for name in names{
//             map.insert(name.clone(), 0);
//         }
//         map
//     }
//
//     fn make_hash2(names : &Vec<Box<str>>) -> HashMap<Box<str>, usize>{
//         let mut map : HashMap<Box<str>, usize> = HashMap::with_capacity(names.len());
//         for name in names{
//             map.insert(name.clone(), 0);
//         }
//         map
//     }
//
//     fn make_hash2_2(names : &Vec<String>) -> HashMap<Box<str>, usize>{
//         let mut map : HashMap<Box<str>, usize> = HashMap::with_capacity(names.len());
//         for name in names{
//             map.insert(name.to_string().into_boxed_str(), 0);
//         }
//         map
//     }
//
//     fn make_hash3(names : &Vec<String>) -> Box<HashMap<String, usize>>{
//         let mut map : HashMap<String, usize> = HashMap::with_capacity(names.len());
//         for name in names{
//             map.insert(name.clone(), 0);
//         }
//         Box::new(map)
//     }
// }
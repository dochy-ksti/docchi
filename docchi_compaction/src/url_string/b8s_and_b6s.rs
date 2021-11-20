use bit_vec::BitVec;

///convert bytes to the 6-bit array.
pub fn to_b6s(b8s : &[u8]) -> Vec<u8>{
    let mut result = BitVec::new();
    let bytes = BitVec::from_bytes(b8s);
    let bytes_len = bytes.len();

    let mut index = 0;
    let mut count6 = 6;
    loop{
        if bytes_len <= index{ break; }
        if count6 == 6{
            count6 = 0;
            //falseを2つpushして、6bit pushして、 falseを2つpushして・・・と繰り返す
            result.push(false);
            result.push(false);
        }
        result.push(bytes[index]);
        index += 1;
        count6 += 1;

    }
    result.to_bytes()
}

///convert the 6-bit array to bytes.
pub fn to_b8s(b6s : &[u8]) -> Vec<u8>{
    let mut result = BitVec::new();
    let bit_len = b6s.len() * 6;
    let byte_size = bit_len / 8;

    fn get_bit(b6 : u8, bit : u8) -> bool{
        let filter : u8 = 1 << (bit - 1);
        return b6 & filter != 0;
    }

    for byte in b6s{
        result.push(get_bit(*byte, 6));
        result.push(get_bit(*byte, 5));
        result.push(get_bit(*byte, 4));
        result.push(get_bit(*byte, 3));
        result.push(get_bit(*byte, 2));
        result.push(get_bit(*byte, 1));
    }

    let mut vec = result.to_bytes();
    if byte_size != vec.len(){
        vec.pop();
    }
    result.to_bytes()
}
use std::ffi::{OsStr, OsString};

pub fn folder_name_to_hash(s : &OsStr) -> Option<u128> {
    let s = s.to_string_lossy();
    let bytes = s.as_bytes();
    if bytes.len() != 32{
        return None;
    }
    let upper = hex_to_u64(&bytes[0..16])?;
    let lower = hex_to_u64(&bytes[16..32])?;
    return Some(((upper as u128) << 64) + lower as u128);
}

fn hex_to_u64(s : &[u8]) -> Option<u64>{
    let mut index = 0;
    let mut r : u64 = 0;
    loop{
        r <<= 4;
        let b = s[index];
        let num = b - '0' as u8;
        if num <= 9 {
            r += num as u64;
        } else{
            let num = b - 'a' as u8;
            if num <= 5{
                r += 10 + num as u64;
            } else{
                return None;
            }
        }
        if index == 15{
            return Some(r);
        }
        index += 1;

    }
}

pub fn hash_to_folder_name(hash: u128) -> OsString{
    let mut s = String::with_capacity(32);
    let upper = (hash >> 64) as u64;
    u64_to_hex_file_name(upper, &mut s);
    let lower = hash as u64;
    u64_to_hex_file_name(lower, &mut s);
    OsString::from(s)
}

fn u64_to_hex_file_name(num : u64, r : &mut String){
    let mut num = num;
    for _ in 0..16{
        let c = get_char_and_shift(&mut num);
        r.push(c as char)
    }
}

fn get_char_and_shift(num : &mut u64) -> u8{
    let hex = (*num & 0xf000_0000_0000_0000) >> 60;
    *num <<= 4;
    if hex < 10{
        return '0' as u8 + hex as u8;
    }
    return 'a' as u8 + (hex - 10) as u8;
}
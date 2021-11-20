
/// A-Za-z0-9, hyphen or underscore is converted to 0-63
pub fn char_to_b6(c : char) -> Option<u8>{
    fn sub(l : char, r : char) -> u8{
        return l as u8 - r as u8;
    }
    return match c {
        'A'..='Z' => Some(sub(c,'A')),
        'a'..='z' => Some(sub(c, 'a') + 26),
        '0'..='9' => Some(sub( c, '0') + 52),
        '-' => Some(62),
        '_' => Some(63),
        _ => None
    }
}

/// 0-63 is converted to A-Za-z0-9, hyphen or underscore
pub fn b6_to_char(b : u8) -> Option<char>{
    fn add(l : char, diff : usize) -> char{
        return (l as usize + diff) as u8 as char;
        //((l as usize) + diff) as char
    }
    let b = b as usize;
    return match b{
        0..=25 => Some(add('A', b)),
        26..=51 => Some(add('a', b - 26)),
        52..=61 => Some(add('0', b - 52)),
        62 => Some('-'),
        63 => Some('_'),
        _ => None,
    }
}

/// The 6-bit array is converted to the string composed of A-Za-z0-9, hyphen or underscore.
/// This is supposed to be used for encoding data to URL
pub fn b6s_to_string(b6s : &[u8]) -> Option<String>{
    let mut s = String::new();

    for b6 in b6s{
        let c = b6_to_char(*b6)?;
        s.push(c)
    }
    return Some(s);
}

/// The string composed of A-Za-z0-9, hyphen or underscore is converted to the 6-bit array.
/// This is supposed to be used for decoding data in URL
pub fn string_to_b6s(s : &str) -> Option<Vec<u8>>{
    let mut result: Vec<u8> = vec![];
    for c in s.chars(){
        let b = char_to_b6(c)?;
        result.push(b);
    }
    return Some(result);
}
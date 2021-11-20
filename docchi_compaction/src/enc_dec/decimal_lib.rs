pub(crate) fn to_string(num : i128, dot : u8) -> String{
    let mut s = num.to_string();
    if dot == 0{ return s; }
    let sign;
    if num < 0{
        sign = false;
        s.remove(0);
    } else{
        sign = true;
    }
    let zeros = dot as isize - s.len() as isize;
    let mut result = String::new();
    if 0 <= zeros{
        result.push_str("0.");
        if 1 <= zeros {
            result.push_str(&String::from_utf8(vec!['0' as u8; zeros as usize]).unwrap());
        }
        result.push_str(&s);
    } else{
        result.push_str(&s);
        let index = s.len() - dot as usize;
        result.insert(index,'.');
    }

    if sign == false{
        result.insert(0, '-');
    }

    return result;
}

pub(crate) fn to_f64(num : i128, dot : u8) -> f64{
    (num as f64) / 10f64.powi(dot as i32)
}
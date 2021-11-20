use crate::kval_enum::KVal;
use std::f64;

/// If it's integer, compact the num to an integer.
/// If it's
/// 動的型言語でnumがf64に決まっている場合や、静的型言語でf64に戻すことがわかっている場合に縮める事ができる。
pub fn comp_double(num : f64) -> KVal {
    //-0.0は整数としては扱えない。
    //0.0 == -0.0はtrueなので面倒な比較をする。
    if num.to_bits() == (-0.0f64).to_bits(){ return KVal::Double(num); }

    let int = num.trunc();
    //整数ではない
    if int != num{ return KVal::Double(num) }
    if int as i64 as f64 == int {
        //Intが8byteだとDoubleにくらべて全然小さくならないしむしろ1bit大きくなる・・・？？
        return comp_int(int as i64);
    } else if num as f32 as f64 == num {
        return KVal::Float(num as f32);
    } else {
        return KVal::Double(num);
    }
}

///f32が整数で表せるなら整数で、bitやbyteに出来るならそこまで削って表す。
pub fn comp_float(num : f32) -> KVal {
    //-0.0は整数としては扱えない。
    //0.0 == -0.0はtrueなので面倒な比較をする。
    if num.to_bits() == (-0.0f32).to_bits(){ return KVal::Float(num) }

    let int = num.trunc();
    //整数ではない
    if int != num{ return KVal::Float(num) }
    //それ以上だと節約にならないので、i32までの数値ならという条件をつける。
    if int as i32 as f32 == int {
        return comp_int(int as i64);
    } else{
        return KVal::Float(num);
    }
}

///bitやbyteに出来るならそこまで削って表す。 KVal::as_i64で戻す
pub fn comp_int(int : i64) -> KVal {
    if int == 0{ return KVal::Bit(false); }
    if int == 1{ return KVal::Bit(true); }
    if int as i8 as i64 == int{ return KVal::Byte(int as i8); }
    return KVal::Int(int);
}

///文字列を適切な形式で保存する。 empty stringならbitにする KVal::as_string(&self)を使って戻す
pub fn comp_str(s : String) -> KVal {
    let len = s.len();
    if len == 0{ return KVal::Bit(false) } //サイズが一番小さいので・・・意味論的にはメチャクチャだけどまあええやろ
    if len < 16{ return KVal::Str16(s); }
    if len < 256{ return KVal::Str256(s); }
    return KVal::BigStr(s);
}

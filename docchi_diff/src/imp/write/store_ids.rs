
use std::collections::BTreeMap;
use docchi_core::structs::{RustParam, Qv};

#[derive(Debug)]
pub(crate) enum StoredIDs{
    Zero,
    U64(u64),
    Bits(Vec<u64>),
    Numbers(Vec<usize>),
}

enum StoredIDsType{
    U64, Bits, Numbers, Zero
}

pub(crate) fn store_ids<'a, T>(params : &'a BTreeMap<usize, &'a RustParam>, lists : &'a BTreeMap<usize, T>) -> StoredIDs{
    let mut max : usize = 0;
    let mut len_sum : usize = 0;
    update_max_and_len(params, &mut max, &mut len_sum);
    update_max_and_len(lists, &mut max, &mut len_sum);


    match calc_type(max, len_sum){
        StoredIDsType::Zero => StoredIDs::Zero,
        StoredIDsType::U64 => StoredIDs::U64(get_u64(params, lists)),
        StoredIDsType::Bits => StoredIDs::Bits(get_bits(params, lists, max)),
        StoredIDsType::Numbers => StoredIDs::Numbers(get_numbers(params, lists, len_sum)),
    }
}

pub(crate) fn store_ids_ref(refs : &BTreeMap<usize, &Qv<String>>) -> StoredIDs{
    let mut max : usize = 0;
    let mut len_sum : usize = 0;
    update_max_and_len(refs, &mut max, &mut len_sum);

    match calc_type(max, len_sum){
        StoredIDsType::Zero => StoredIDs::Zero,
        StoredIDsType::U64 =>{
            let mut u = 0;
            update_u64(refs, &mut u);
            StoredIDs::U64(u)
        }
        StoredIDsType::Bits =>{
            StoredIDs::Bits(get_bits_ref(refs, max))
        },
        StoredIDsType::Numbers => StoredIDs::Numbers(get_numbers_ref(refs, len_sum)),
    }
}


fn get_u64<'a,T>(params : &'a BTreeMap<usize, &'a RustParam>, lists : &'a BTreeMap<usize,T>) -> u64{
    let mut r : u64 = 0;
    update_u64(params, &mut r);
    update_u64(lists, &mut r);
    r
}

fn update_u64<T>(map : &BTreeMap<usize, T>, r : &mut u64){
    for &i in map.keys(){
        *r |= 1 << i;
    }
}

fn get_bits<'a, T>(params : &'a BTreeMap<usize, &'a RustParam>, lists : &'a BTreeMap<usize, T>, max : usize) -> Vec<u64> {
    let len = max / 64 + 1;
    let mut r : Vec<u64> = Vec::with_capacity(len);
    for _ in 0..len{
        r.push(0);
    }

    update_bits(params, &mut r);
    update_bits(lists, &mut r);
    return r;
}

fn get_bits_ref(refs : &BTreeMap<usize, &Qv<String>>, max : usize) -> Vec<u64> {
    let len = max / 64 + 1;
    let mut r : Vec<u64> = Vec::with_capacity(len);
    for _ in 0..len{
        r.push(0);
    }

    update_bits(refs, &mut r);
    return r;
}

fn update_bits<T>(map : &BTreeMap<usize, T>, r : &mut Vec<u64>){
    for &i in map.keys(){
        let ind = i / 64;
        let bit = i % 64;

        unsafe {
            *r.get_unchecked_mut(ind) |= 1 << bit;
        }
    }
}

fn get_numbers<'a, T>(params : &'a BTreeMap<usize, &'a RustParam>, lists : &'a BTreeMap<usize, T>, len : usize) -> Vec<usize> {
    let mut r : Vec<usize> = Vec::with_capacity(len);
    update_numbers(params, &mut r);
    update_numbers(lists, &mut r);
    r
}

fn get_numbers_ref(refs : &BTreeMap<usize, &Qv<String>>, len : usize) -> Vec<usize> {
    let mut r : Vec<usize> = Vec::with_capacity(len);
    update_numbers(refs, &mut r);
    r
}

fn update_numbers<T>(map : &BTreeMap<usize, T>, r : &mut Vec<usize>){
    for &i in map.keys(){
        r.push(i);
    }
}

fn calc_type(max : usize, len : usize)->StoredIDsType {
    if len == 0 { return StoredIDsType::Zero; }

    //numberは表すのにひとつあたり11bitぐらいかかるのに対し、u64なら1bitである。
    // u64の場合あるものもないものも割り当てる必要があるが、Numbersはあるものにだけ割り当てる
    //あるものが1/11なら節約になる場合がある
    //
    //別に必要のない機能に思うが、膨大なメンバ数やリストのアイテム数になったときに、1つ2つだけ変化がある、
    // という場合にはこの細かい指定が意味を持ってくるはずだ。一般的にはあまり意味を持たない気もするが、
    // ありうる範囲ならばワーストケースに備えたい
    //
    //ないものだけを指定する、という方式で節約できる可能性もあるのだが、そんなのあるかなあ？
    //あったとしても変化データが膨大になるのでこのフラグ部分を節約することにたいした意味を感じない
    //面倒臭さがまさる
    if max < 64 {
        if len > max / 11 {
            StoredIDsType::U64
        } else {
            StoredIDsType::Numbers
        }
    } else if max < 32768{
        if len > max / 24 {
            StoredIDsType::Bits
        } else {
            StoredIDsType::Numbers
        }
    } else if max < 8388608{
        if len > max / 32 {
            StoredIDsType::Bits
        } else {
            StoredIDsType::Numbers
        }
    } else if max < 2147483648{
        if len > max / 40 {
            StoredIDsType::Bits
        } else {
            StoredIDsType::Numbers
        }
    } else{
        //このサイズまで処理できると考えるべきではないだろう。未来のコンピュータならわからないが・・・
        if len > max / 48 {
            StoredIDsType::Bits
        } else {
            StoredIDsType::Numbers
        }
    }
}

fn update_max_and_len<T>(map : &BTreeMap<usize,T>, max : &mut usize, len : &mut usize){
    let (max_t, len_t) = max_and_len(map);
    *len += len_t;
    *max = (*max).max(max_t);
}

fn max_and_len<T>(map : &BTreeMap<usize,T>) -> (usize, usize){
    if map.len() == 0{ (0,0) }
    else{
        let (id,_) = map.iter().next_back().unwrap();
        (*id, map.len())
    }
}
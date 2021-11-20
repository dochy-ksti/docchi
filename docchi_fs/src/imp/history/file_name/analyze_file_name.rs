use crate::imp::history::file_name::file_name_props::FileNameProps;
use std::str::FromStr;

/// #hogehoge#(0)_0_0_0_0_0_0.his といった感じである。tagがない場合
/// (0)_0_0_0_0_0_0.his となる。
/// 派生していない場合 _0_0_0_0_0_0.his となり、()は使われない
///
/// 最初の数字は管理番号であり、newまたは派生すると1増える。newしたら管理番号_0にリセットされる。
///
///
/// 最初は管理番号 0 の 0 なので 0_0
/// next すると 0_0_0
/// 簡単のため、仮に2-phaseとすると、次にnextしたとき 0_0_1になる。
/// そして 0_0_2・・・と増えていくが条件を満たすとphase shiftする。phase Aを1増やす
///
/// 0_1
///
/// そしてnextでまた増えていく
///
/// 0_1_0
/// 0_1_1 ...
///
/// 0_1_0 や 0_1_1 を開くためには、元となる 0_1 が必要である。そういった依存関係がファイル名として記されていて、
/// ファイル名をいじるとこのシステムはぶっ壊れる
///
/// で、newすると
///
/// 1_0 となる。
///
/// ここで一旦過去のファイル、0_1_0から派生することにする。
///
/// (0)2_1_0_0
///
/// 派生元の管理番号は0、新管理番号は最新の番号+1になるので、先程newしたときに1_0が出来たので、この次である2_0になる
///
/// 派生した場合、派生元は、最後の一つを除いた番号になる。(0)2_1_0_0であれば、管理番号を戻して、最後の一つを削除し、
/// 0_1_0ということになる。
///
/// 最終Phaseから派生する場合、Phase Shiftが発生する可能性があり、ややこしいので、親Phaseから派生することにする
/// その方がキャッシュが残っていやすいのでよかろうと思う（これはDochyキャッシュの設定次第だが・・・)
///
/// 最新管理番号をgetする方法であるが、最新のファイルは必ず最新の管理番号になる、つまり子フェーズがないファイルから派生する場合でも、
/// それが最新でないならば管理番号を新しくつけ直す。
///
/// 実装には正規表現を使うべきだろうが、手で書いたほうが動作速度が速いと思っているし、この処理はできるだけ速くしたい
pub(crate) fn analyze_file_name(s : &str, hint_max_phase : Option<usize>) -> Option<FileNameProps>{
    if s.ends_with(".his") == false{
        return None;
    }

    let tag = get_tag(s);

    let s = if let Some(tag) = tag{
        &s[(tag.len() + 2)..]
    } else{
        s
    };

    let prev_ctl = get_prev_ctl(s);
    let mut s = if let Some(prev_ctl) = prev_ctl{
        &s[(prev_ctl.len() + 2)..]
    } else{
        s
    };

    let n = if let Some((num, read)) = get_num(s.as_bytes()){
        s = &s[read..];
        num
    } else{
        return None;
    };

    let prev_ctl = prev_ctl.and_then(|s| u32::from_str(s).ok()).unwrap_or(n);

    let capacity = hint_max_phase.unwrap_or(0) + 1;
    let mut order : Vec<u32> = Vec::with_capacity(capacity);
    loop{
        if let Some((num, read)) = get_num(s.as_bytes()){
            order.push(num);
            s = &s[read..];
        } else{
            if s == ".his" && order.len() != 0{
                if let Ok(props) = FileNameProps::new(n, prev_ctl,
                                                      order, tag.map(|t| t.to_string())){
                    return Some(props);
                }
            }
            return None;
        }
    }
}

fn get_tag(s : &str) -> Option<&str>{
    let bytes = s.as_bytes();
    if bytes.len() <= 1{ return None; }
    let first = bytes[0];
    if first != '#' as u8{
        return None;
    }
    for (i,c) in bytes.iter().skip(1).enumerate(){
        if *c == '#' as u8{
            return Some(&s[1..i])
        }
    }
    return None;
}

fn get_prev_ctl(s : &str) -> Option<&str>{
    let bytes = s.as_bytes();
    if bytes.len() <= 1{ return None; }
    let first = bytes[0];
    if first != '(' as u8{
        return None;
    }
    for (i,c) in bytes.iter().skip(1).enumerate(){
        if *c == ')' as u8{
            return Some(&s[1..i+1])
        }
    }
    return None;
}

///numと読んだ文字数を返す
fn get_num(s : &[u8]) -> Option<(u32, usize)>{
    if s.len() == 0 { return None; }
    if s[0] != '_' as u8{ return None; }
    let mut i = 1;
    let mut result : u32 = 0;

    loop{
        if s.len() <= i{ return None; }

        let b = s[i];
        if '0' as u8 <= b && b <= '9' as u8{
            result *= 10;
            result += (b - '0' as u8) as u32;
        } else{
            return Some((result, i));
        }
        i += 1;
        if i == 10{ return None; } //オーバーフローは避ける
    }


}
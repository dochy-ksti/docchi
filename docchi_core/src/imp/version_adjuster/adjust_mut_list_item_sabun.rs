//use crate::{HashM, HashMt};
use crate::error::CoreResult;
use crate::imp::json_to_rust::names::Names;
use crate::imp::version_adjuster::adjust_mut_list::adjust_mut_list;
use crate::imp::structs::list_value::{ListDefValue};
use crate::imp::structs::list_def_obj::ListDefObj;
use crate::imp::structs::util::hash_m::{HashM, HashMt};
use crate::imp::structs::list_sab_value::ListSabValue;

pub(crate) fn adjust_mut_list_item_sabun(def : &ListDefObj, old_sabun : &mut HashM<String, ListSabValue>, names : &Names) -> CoreResult<HashM<String, ListSabValue>>{


    //デフォルトから変化しない場合はsabunには加わらないが、sabun.len()だと、
    //undefinedで一個増えただけでテーブル再構成＆cap2倍にされてしまう可能性がある
    //実際はpower of 2のcapacityにしかならないので、その可能性は低そうにも見えるが、1個,2個,4個とかのことも多いと思うのでどうなんでしょうな
    //まあ1,2,4個とかなら再構成されても誤差の範囲ではないかと思うが・・・
    //sabun数256個 undefined1個とかでのperformance低下を重く見るべきか？
    //default.len()でサイズを大きくとって、やたらと無駄が発生することを重く見るべきか？　（そんなに無駄が出るだろうか。書き換えないメンバはそう多くないと思うが・・・)
    //変なユースケースでの最悪を想定するべきで、やたらとdefにメンバを大量に用意して、sabunはちょっとしかないという戦略もありうるので、
    //その場合に無駄で巨大なhashtableを初期化する可能性を重く見るべきではなかろうか
    let mut result : HashM<String, ListSabValue> = HashMt::with_capacity(old_sabun.len());

    for (def_key,_, def_v) in def.default(){
        let sabun_v = if let Some(v) = old_sabun.remove(def_key){ v } else {
            match def_v{
                ListDefValue::Param(p, vt) =>{
                    if vt.undefiable(){
                        ListSabValue::Param(p.to_undefined())
                    } else{
                        continue;
                    }
                },
                ListDefValue::MilDef(mut_def) =>{
                    if mut_def.undefiable(){
                        ListSabValue::Mil(None)
                    } else{
                        continue;
                    }
                },
                _ =>{
                    Err(format!("{} {} mut list's default item can only have Param or MilDef", names, def_key))?
                }
            }
        };
        match sabun_v{
            ListSabValue::Param(p) =>{
                //型チェックめんどいからなしでいいかな・・・？
                result.insert(def_key.to_string(), ListSabValue::Param(p));
            },
            ListSabValue::Mil(op) =>{
                match op{
                    Some(im) =>{
                        let new = adjust_mut_list(def, im, &names.append(def_key))?;
                        result.insert(def_key.to_string(), ListSabValue::Mil(Some(new)));
                    },
                    None =>{
                        result.insert(def_key.to_string(), ListSabValue::Mil(None));
                    }
                }
            },
            _ =>{ Err(format!("{} {} mut list items can only have Param or Mil", names, def_key))? }
        }
    }
    Ok(result)
}
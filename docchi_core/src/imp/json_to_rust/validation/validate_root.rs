use crate::error::CoreResult;
use crate::imp::json_to_rust::validation::validate_data::validate_table;
use crate::imp::json_to_rust::names::Names;
use crate::imp::json_to_rust::validation::validate_list::validate_const_list;
use crate::imp::json_to_rust::validation::validate_mut_list::validate_mut_list;
use crate::imp::json_to_rust::validation::validate_list_def::validate_list_def;
use crate::imp::structs::root_obj::RootObject;
use crate::imp::structs::root_value::RootValue;
use crate::imp::json_to_rust::validation::validate_old_def_mem::validate_old_root_def_mem;
use crate::imp::structs::root_sab_value::RootSabValue;

/// json読み出し時のチェックがあり、adjust時のチェックもある。
/// それらでは補足しきれないチェックをするのがこれの役割。
///
/// ここではどこまでチェックするのか、補足しきれないものというのはどこまでか、というのが問題なのだけど、基本的には全部チェックする。他のチェックに依存しない。
/// というところを目指すことになるだろう。ダブルチェックや。
///
/// 型により可能な値はかなり限られるので、その中ですべてが妥当な値であるかどうかを調べることも出来ないことはないはず。
/// 基本的に
/// ・sabunがdefにあり、型があっていて、(optional)oldじゃないか
/// ・ref先が存在し、(optional)oldじゃないか
/// ・InnerListDefがListのDefにあり、InnerListがListItemにあるか。Root直下にInnerがないか。
/// ・Oldに指定されたメンバまたはIDが存在するか
/// を確かめている
///
/// can_use_oldがtrueだとoldでも気にしなくなる。Jsonで初期値を読み込んだ後はcan_use_old=false,
/// 旧バージョンから以降した場合はcan_use_old=trueでやるとよかろうと思う
pub(crate) fn validate_root(root : &RootObject, can_use_old: bool) -> CoreResult<()>{
    validate_old_root_def_mem(root.old(), root.default().def(), &Names::new("."))?;

    for (name, (_id, val)) in root.default().def(){
        let names = &Names::new(name);
        //RootはOldでも値を入れざるを得ないので入れて良い
        //なのでここではOldは無視
        match val {
            RootValue::Param(p, _) => {
                if let Some(sab) = root.sabun().get(name) {
                    if let RootSabValue::Param(sab) = sab {
                        if p.acceptable(sab) == false {
                            Err(format!("Root's member {}'s sabun is invalid", name))?
                        }
                    } else {
                        Err(format!("Root's member {} is not a param", name))?
                    }
                }
            },
            RootValue::Table(data) =>{
                validate_list_def(data.default(), root, can_use_old, false, names)?;
                validate_table(data.default(), data.list(), root, data.old(), can_use_old, names)?
            },
            RootValue::CList(list) =>{
                validate_list_def(list.default(), root, can_use_old, false, names)?;
                validate_const_list(list.default(),list.list(), root, can_use_old, names)?
            },
            RootValue::MList(m) =>{
                validate_list_def(m.default(), root, can_use_old, true, names)?;
                if let Some(sab) = root.sabun().get(name) {
                    if let RootSabValue::Mut(sab) = sab {
                        if let Some(list) = sab {
                            validate_mut_list(m.default(), list.list(), root, can_use_old, names)?
                        } else {
                            if !m.undefiable() {
                                Err(format!("Root's member {} can't be undefined", name))?
                            }
                        }
                    } else{
                        Err(format!("Root's member {} is not a mut list", name))?
                    }
                }

            },
        }
    }

    return Ok(());
}
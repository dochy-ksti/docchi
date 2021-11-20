///1文字目を大文字にする
/// snake_caseを発見したら大文字に変換する
pub(crate) fn to_type_name(s : &str) -> String{
    let mut r = String::with_capacity(s.len());
    let mut to_upper_case = false;

    for (i,c) in s.chars().enumerate(){
        if i == 0{
            if c == '@'{
                r.push_str("At");
            } else {
                r.push(c.to_ascii_uppercase());
            }
        } else{
            if c == '_'{
                to_upper_case = true;
            } else {
                if to_upper_case{
                    r.push(c.to_ascii_uppercase());
                    to_upper_case = false;
                } else {
                    r.push(c)
                }
            }
        }
    }
    r
}

///小文字にしてsnake_caseにする
pub(crate) fn to_snake_name(s : &str) -> String{
    let mut r = String::new();
    for (i,c) in s.chars().enumerate(){
        if i == 0{
            if c == '@'{
                r.push_str("at");
            } else {
                r.push(c.to_ascii_lowercase());
            }
        } else{
            if c.is_ascii_uppercase(){
                r.push('_');
                r.push(c.to_ascii_lowercase());
            } else {
                r.push(c)
            }
        }
    }
    r
}

pub(crate) fn to_citem_type_name(stem : &str) -> String{
    format!("{}CItem", to_type_name(stem))
}
pub(crate) fn to_mitem_type_name(stem : &str) -> String{
    format!("{}MItem", to_type_name(stem))
}

pub(crate) fn to_table_type_name(stem : &str) -> String{
    format!("{}Table", to_type_name(stem))
}

pub(crate) fn to_ids_type_name(stem : &str) -> String{
    format!("{}TableID", to_type_name(stem))
}

pub(crate) fn to_enum_type_name(stem : &str) -> String{ format!("{}Enum", to_type_name(stem)) }

pub(crate) fn to_kind_type_name(stem : &str) -> String{ format!("{}Kind", to_type_name(stem)) }
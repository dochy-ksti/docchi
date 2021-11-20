use crate::imp::rust_to_json::list::tmp_json_list::TmpJsonList;
use crate::imp::rust_to_json::list::list_type_to_string::list_type_to_string;
use crate::imp::rust_to_json::list::default_to_json::default_to_json;
use crate::imp::rust_to_json::string_set_to_json::{string_set_to_json};
use crate::imp::rust_to_json::list::tmp_obj_to_json::tmp_obj_to_json;
use crate::imp::structs::my_json::Value;
use crate::imp::structs::list_type::ListType;

pub(crate) fn rust_list_to_json(l : &TmpJsonList, list_type : ListType) -> Value{
   let mut result : Vec<Value> = vec![];

   result.push(val_s(list_type_to_string(&list_type)));

   // if let Some(compatible) = &l.compatible{
   //    result.push(string_set_to_json("Compatible", compatible));
   // }
   if let Some(old) = &l.old{
      result.push(string_set_to_json("Old", old));
   }
   if let Some(next_id) = l.next_id {
      result.push(Value::Array(vec![
         val_str("NextID"),
         Value::Number(next_id as f64)
      ]));
   }
   //余計な情報は上に持ってきて、defaultとvecの距離を近づけよう
   if let Some(default) = &l.default{
      result.push(Value::Array(vec![default_to_json(default)]))
   }

   for item in &l.vec{
      result.push(tmp_obj_to_json(item))
   }

   return Value::Array(result);
}

pub(crate) fn val_str(s : &str) -> Value{
   Value::String(s.to_string())
}
pub(crate) fn val_s(s : String) -> Value{
   Value::String(s)
}
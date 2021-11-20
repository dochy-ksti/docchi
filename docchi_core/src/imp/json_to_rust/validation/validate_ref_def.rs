use crate::imp::json_to_rust::names::Names;
use crate::error::CoreResult;
use crate::imp::json_to_rust::validation::validate_old_def_mem::validate_old_ref_def;
use crate::imp::structs::ref_def_obj::RefDefObj;
use crate::imp::structs::qv::Qv;


pub(crate) fn validate_ref_def(def : &RefDefObj, names : &Names) -> CoreResult<()> {
    if def.is_enum() {
        for (_, _, v) in def.refs() {
            match v.value() {
                Qv::Null => {},
                _ => Err(format!("{} all default members of Enum must be null", names))?,
            }
        }
    }
    if def.is_enum() {
        validate_old_ref_def(def.old(), def.refs(), &names.append("Enum"))?;
    } else {
        validate_old_ref_def(def.old(), def.refs(), &names.append("Ref"))?;
    }

    Ok(())
}
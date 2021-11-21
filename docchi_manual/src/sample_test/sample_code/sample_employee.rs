use docchi::core::intf::{RootObjectPtr, MListPtr, MItemPtr};
use docchi::core::intf::{mitem};
use docchi::core::structs::Qv;
use docchi::core::root_to_json_new_default;
use docchi_diff::apply_diff;
use docchi::error::DpResult;


//#[test]
fn sample_employee() -> DpResult<()>{
    let ini_path= "sample_code_json/sample_employee_docchi_initial";
    let mut r = docchi::core::json_dir_to_root(ini_path, false)?;

    let rp = RootObjectPtr::new(&mut r);

    let mut ml : MListPtr<MItemPtr> = docchi::core::intf::root::get_mlist_mut(
        rp, "@Employees")?.unwrap();
    ml.insert();
    let item = ml.insert();
    mitem::set_str(item, "userId", Qv::Val("devid".to_string()));
    mitem::set_str(item, "firstName", Qv::Val("Devid".to_string()));
    mitem::set_str(item, "lastName", Qv::Val("Rome".to_string()));
    mitem::set_str(item, "employeeCode", Qv::Val("E2".to_string()));
    mitem::set_str(item, "phoneNumber", Qv::Val("1111111".to_string()));
    mitem::set_str(item, "emailAddress",
                   Qv::Val("devid.rome@learningcontainer.com".to_string()));

    let item = ml.insert();
    mitem::set_str(item, "userId", Qv::Val("tin".to_string()));
    mitem::set_str(item, "jobTitle", Qv::Val("Program Directory".to_string()));
    mitem::set_str(item, "firstName", Qv::Val("tin".to_string()));
    mitem::set_str(item, "lastName", Qv::Val("jonson".to_string()));
    mitem::set_str(item, "employeeCode", Qv::Val("E3".to_string()));
    mitem::set_str(item, "phoneNumber", Qv::Val("2222222".to_string()));
    mitem::set_str(item, "emailAddress",
                   Qv::Val("tin.jonson@learningcontainer.com".to_string()));
    //let hoge = rust_to_json_new_default(&r)?;
    //println!("{}", hoge.to_string_pretty());

    let mut from = docchi::core::json_dir_to_root(
        ini_path, false)?;
    let vec = docchi_diff::get_diff(&from, &r)?;
    println!("{}", vec.len());
    apply_diff(&mut from, &mut vec.as_slice())?;

    let from = root_to_json_new_default(&r)?;
    println!("{}", from.to_string_pretty());
    Ok(())

}
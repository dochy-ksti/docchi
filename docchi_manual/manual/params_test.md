[prev](docchi_params_generate.md)
[index](index.md)
[next](root.md)

### 1-1-2. Use Generated Source Code

```rust
use docchi::error::DpResult;
use docchi::core::json_dir_to_root;
use docchi::core::structs::{RootObject, NullOr, UndefOr, Qv};
use crate::a3_docchi_langs_basics::docchi_params_accessor::RootIntf;

#[test]
fn params_test() -> DpResult<()> {
    // You validated Docchi Src when you generated the source code, so validation is not needed now.
    let root_obj : RootObject = json_dir_to_root("src/a3_docchi_langs_basics/docchi_params", false)?;

    // RootIntf is the struct created in the generated source code.
    // It's always named "RootIntf".
    let mut root = RootIntf::new(root_obj);

    // Let's get values
    let ival : i64 = root.int_value();
    let fval : f64  = root.float_value();
    let bool : bool = root.bool_value();

    // "[variable_name]()" gets the variable.

    // These values are not modified, so RootIntf returns default values.
    //  "[variable_name]_def_val()" can get the default value
    assert_eq!(ival, root.int_value_def_val());
    assert_eq!(fval, root.float_value_def_val());
    assert_eq!(bool, root.bool_value_def_val());

    // Values can be set by "set_[variable_name]" method

    root.set_int_value(23);

    assert_eq!(23, root.int_value());

    let sval : &String = root.string_value();
    let _iarray : &Vec<i64> = root.int_array();
    let _farray : &Vec<f64> = root.float_array();
    let _bin : &Vec<u8> = root.binary();

    // Int, Float, and Bool are copyable, but String and Vec are not.
    // These types return references

    // "def_val" also returns reference
    let str_def_val : &String = root.string_value_def_val();

    assert_eq!(sval, str_def_val);

    // You can set String and Vec
    root.set_string_value("New String".to_string());
    root.set_int_array(vec![10,20,30]);

    assert_eq!(root.string_value(), "New String");
    assert_eq!(root.int_array(), &[10,20,30]);

    // You can get mutable references
    let sval : &mut String = root.string_value_mut();
    sval.push_str(" push");

    assert_eq!(root.string_value(), "New String push");

    // You can use "match" to use nullable values,
    match root.nullable_int2(){
        NullOr::Val(v) => assert_eq!(v, 5),
        NullOr::Null => {assert!(false)},
    }

    // "undefiable" value
    match root.undefiable_int(){
        UndefOr::Val(_) => { assert!(false)},
        UndefOr::Undefined =>{},
    }

    // Both "undefiable" and "nullable". Qv stands for "Questionable value"
    match root.undef_nullable_int(){
        Qv::Val(v) =>{ assert_eq!(v, 2) },
        Qv::Null =>{assert!(false)},
        Qv::Undefined =>{assert!(false)},
    }

    let bin : &mut Vec<u8> = root.binary_mut();

    // Docchi doesn't have f32 types currently.
    // Binary type can be a workaround

    unsafe{
        bin.clear();
        bin.reserve_exact(400);
        bin.set_len(400);
        // Unsafely created uninitialized 400-byte Vec<u8>

        let slice : &mut [f32] = std::slice::from_raw_parts_mut(bin.as_mut_ptr() as *mut f32, 100);

        // regarded the 400-byte Vec<u8> as a mutable [f32] slice with 100 length.

        //initializes slice
        for i in 0..100{
            slice[i] = 0.1 * i as f32;
        }

        // Gets the same slice as readonly
        let s = std::slice::from_raw_parts(root.binary().as_ptr() as *const f32, 100);

        for i in 0..100{
            assert_eq!(s[i], 0.1 * i as f32)
        }

        // You can support unsupported types with this kind of techniques.

        // ** Note **

        // Normal allocators should allocate Vec to 8-byte alignment, but I believe it's not guaranteed,
        // so this code is technically invalid...?

        // Transferring data to the system with different endianness can be a problem for this kind of techniques.

        // Docchi has no intention to support transferring data between machines with different endianness
    }

    Ok(())
}
```


[prev](docchi_params_generate.md)
[index](index.md)
[next](root.md)

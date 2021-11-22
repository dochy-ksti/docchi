[prev](a4_clist_cil_root.json5.md)
[index](index.md)
[next](a5_mlist_mlist_root.json5.md)

### 2-2. Use Clist And Cil

```rust
use docchi::error::DpResult;
use docchi::core::structs::RootObject;
use docchi::core::json_dir_to_root;
use crate::a4_clist::cil_accessor::RootIntf;

//#[test]
fn cil_test() -> DpResult<()> {
    let root_obj : RootObject = json_dir_to_root("src/a4_clist/cil", false)?;

    let root = RootIntf::new(root_obj);

    // Iterates the CList
    for item in root.clist().iter(){

        // Gets the inner list from the item.
        let il = item.inner_list();
        println!("len {}", il.len());
        // When inner_list is omitted in the source, len == 0

        // Iterates the inner list.
        for item in il.iter(){
            println!("item name is {}", item.name());
        }
    }
    // Output:
    // len 2
    // item name is p
    // item name is q
    // len 0
    Ok(())
}
```


[prev](a4_clist_cil_root.json5.md)
[index](index.md)
[next](a5_mlist_mlist_root.json5.md)

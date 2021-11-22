[prev](a5_mlist_mil_root.json5.md)
[index](index.md)
[next](a6_table_table_root.json5.md)

### 3-2. Use Mil

```rust
use docchi::error::DpResult;
use docchi::core::structs::RootObject;
use docchi::core::json_dir_to_root;
use crate::a5_mlist::mil_accessor::RootIntf;

//#[test]
fn mil_test() -> DpResult<()> {
    let root_obj : RootObject = json_dir_to_root("src/a5_mlist/mil", false)?;

    let mut root = RootIntf::new(root_obj);

    // When it's mutably borrowed, we consider it "modified". If it's actually modified is not relevant.
    // When it's "modified", if it's cloned (typically, cloning occurs when saving), the item/value is copied,
    // so the two items are on the different memory address.
    // We compare these address, and determine if it's modified.
    // See [Arc::make_mut](https://doc.rust-lang.org/std/sync/struct.Arc.html#method.make_mut)
    let mut mlist = root.mlist_mut();

    // When we insert, an item is created and auto-increment ID is assigned,
    // and the item is inserted to the last position. (we also have insert_first)
    // and the mutable reference of it is returned.
    let mut item = mlist.insert();
    item.set_val(100);

    // we can change orders of items in a Linked-List way.

    // Gets the last item's ID
    let last_id = mlist.last_id().unwrap();
    // Gets the first item's ID
    let first_id = mlist.first_id().unwrap();

    // Puts the newly created last item into the next position of the first-item.
    mlist.move_to_next(
        /* prev_items_id */ first_id,
        /* id */ last_id);

    let mut first = mlist.first_mut().unwrap();
    let mut il = first.inner_list_mut();
    let mut item = il.insert_first();
    item.set_name("first of the first item".to_string());

    let mut last = mlist.last_mut().unwrap();
    let mut il = last.inner_list_mut();
    let mut item = il.insert();
    let name = item.name_mut();
    // You can also use mutable references. push_str appends string (the default value remains).
    name.push_str("--last of the last item");

    // *** Not Important Note ***

    // The default value doesn't usually exist in the item's hashtable.
    // When a value doesn't exist in the table, Docchi considers it as the default value.
    // In other words, when you get the value from an item and if it doesn't exist,
    // Docchi returns the default value.

    // When you set the value into the item, the value is placed in the item's hashtable,
    // so we can get the value instead of the default value.

    // If you mutably borrow an unset value, Docchi sets the default value into the item's hashtable first,
    // and returns the mutable reference.
    // If you push_str to it, the pushed string is appended to the default value.

    // *** End Of Not Important Note ***


    // Iterates the MList
    for (id, item) in root.mlist().iter(){
        println!("Item's ID is {}, val is {}", id, item.val());

        // Gets the inner list from the item.
        let il = item.inner_list();
        println!("Inner List's len {}", il.len());

        // Iterates the inner list.
        for (_id, item) in il.iter(){
            println!("item name {}", item.name());
        }
    }
    Ok(())
}
// Output:
//
// Item's ID is 0, val is 1
// Inner List's len 3
// item name first of the first item
// item name p
// item name q
// Item's ID is 2, val is 100
// Inner List's len 0
// Item's ID is 1, val is 2
// Inner List's len 1
// item name x--last of the last item
```


[prev](a5_mlist_mil_root.json5.md)
[index](index.md)
[next](a6_table_table_root.json5.md)

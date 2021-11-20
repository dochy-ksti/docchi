[prev](root.md)
[index](index.md)
[next](split_src.md)

### 4-1. Use Table

```rust
use docchi::error::DpResult;
use docchi::core::structs::{RootObject, NullOr};
use docchi::core::json_dir_to_root;
use crate::a6_table::table_accessor::{RootIntf, Table2TableID, Table1TableID};


//#[test]
fn table_test() -> DpResult<()> {
    let root_obj : RootObject = json_dir_to_root("src/a6_table/table", false)?;

    let mut root = RootIntf::new(root_obj);

    let table1 = root.table1();

    // In tables, you can get items by ID
    println!("item1 val {}", table1.item1().val());

    let a_list = root.a_list();

    for item in a_list.iter(){
        let name = item.name();
        let id = item.ref_id_table1();
        let ref_val = item.ref_table1().val();
        println!("name {} ref_id {} ref_val {}", name, id, ref_val);
    }

    let table2 = root.table2();

    // Iterating tables is tiresome (because it's not for iteration...)

    let len = Table2TableID::len();
    for n in 0..len{
        // We can get ID's information from [table-name]TableID struct
        let id = Table2TableID::from_num(n);

        // from_num(n) can get individual ID info by the sequential order of the table items
        // from_num(0) gets the first items's ID info....

        // Gets ID string
        let id_str = id.to_str().to_owned();

        // Gets item from table by ID
        let item = table2.get_by_id(id);

        // Gets the ref ID of the item
        let ref_id = item.ref_id_table1();

        // Gets the ref item's val
        let val = item.ref_table1().val();
        println!("ID {} ref_id {} ref_item's val {}", id_str, ref_id, val);

        // We can also get ID info by ID string
        // let id = Table2TableID::from_str("item1");
        //
        // Gets the ID's num (sequential order)
        // let num = id.to_num()
    }

    let mlist = root.mlist();

    // MList's items have ID numbers, but it's not relevant now
    for (_id, item) in mlist.iter(){
        let t1_ref = item.ref_table1();
        match item.ref_table2(){
            // Nullable values may be null, so we use pattern match
            NullOr::Val(t2_ref) => println!("t1_ref val {} t2_ref's t1_ref val {}",
                                            t1_ref.val(), t2_ref.ref_table1().val()),
            NullOr::Null => println!("t1_ref val {} t2_ref Null", t1_ref.val())
        }
    }



    // Let's mutate MList
    let mut mlist = root.mlist_mut();

    // Let's collect ID numbers
    let ids : Vec<u64> = mlist.iter().map(|(id, _)| id).collect();

    // You can get an item from MList by the ID number
    // ids[1] is the second item's ID number, so we can get the second item by get_mut(ids[1])
    let mut item = mlist.get_mut(ids[1]).unwrap();

    // Obviously, the list hasn't been changed yet.
    // Initial values of the ID numbers are the same as the sequential order in default.
    // The first item's ID is 0, the second is 1...
    // ids[1] is 1, so you can write it as get_mut(1).

    // Sets the ref-table1
    item.set_ref_table1(Table1TableID::Item1);

    // ref_table2 is nullable, so NullOr::Val is needed
    item.set_ref_table2(NullOr::Val(Table2TableID::B));

    // Let's see the change
    for (_id, item) in mlist.iter(){
        let t1_ref = item.ref_table1();
        match item.ref_table2(){
            NullOr::Val(t2_ref) => println!("t1_ref val {} t2_ref's t1_ref val {}",
                                            t1_ref.val(), t2_ref.ref_table1().val()),
            NullOr::Null => println!("t1_ref val {} t2_ref Null", t1_ref.val())
        }
    }

    Ok(())
}

// Output
//
// item1 val 10
// name m ref_id item1 ref_val 10
// name n ref_id item2 ref_val 20
// ID a ref_id item2 ref_item's val 20
// ID b ref_id item1 ref_item's val 10
// t1_ref val 10 t2_ref's t1_ref val 20
// t1_ref val 20 t2_ref Null
// t1_ref val 10 t2_ref's t1_ref val 20
// t1_ref val 10 t2_ref's t1_ref val 10
```


[prev](root.md)
[index](index.md)
[next](split_src.md)

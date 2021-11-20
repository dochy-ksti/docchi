[prev](clist_and_mlist.md)
[index](index.md)

### 9-2. Undefined MList

Dochy doesn't have nullable/undefiable collections except undefiable MList.
```
{
  list! : [
    "MList",
    [{
      v : 1,
    }],
    {
      v : 2
    },
    {
      v : 3
    }
  ]
}
```
Undefiable lists can be separated too.
```
root.json5
{
}
list!.json5
[
	"MList",
	[{
		v: 1,
	}],
	{
		v: 2
	},
	{
		v: 3
	}
]
```
Let's test it.
 ```Rust
fn undefiable_list_test() -> DpResult<()> {
    let old = json_dir_to_root("src/b3_1_clist_and_mlist/jsons/no_data", true)?;
    let new = json_dir_to_root("src/b3_1_clist_and_mlist/jsons/undefiable_list_separated", true)?;

    // We call "adjust_versions" manually here.
    // "load_dochy_file"/"load_history_file" automatically adjusts versions, so calling this manually isn't necessary.
    let r = adjust_versions(new, old, true)?;

    let mut r = RootIntf::new(r);
    //"list" is undefined in the old version, therefore it returns None.
    assert!(r.list().is_none());
    let a = r.list_mut();
    // When "list_mut" is called for the first time, an empty list is created and inserted, and the list is returned.
    assert_eq!(a.len(), 0);
    Ok(())
}
```


[prev](clist_and_mlist.md)
[index](index.md)

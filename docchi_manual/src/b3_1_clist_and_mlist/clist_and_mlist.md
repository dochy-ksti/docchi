## CList and MList

Dochy has two types of lists, "CList" and "MList".

CList is immutable, and if it's changed in a new version,
the old CList is completely ignored and replaced by the new CList.
```
{
 list : [
  "CList",
  // every list must have the default object which is enclosed by "[{" and "}]"
  [{
   //Variables must be defined in the default object with its initial value
   foo : 0,
  }],
  {
   foo : 1,
  },
  {
   foo : 2,
  },
  {
   //if a value is not written, logically its value is the default value, 
   //and this object actually doesn't have the value. 
   //No memory is used and no storage space is consumed   
  }
 ] 
}
```
And the new version
```
{
 list : [
  "CList",
  [{
   foo : -1,
  }],
  {
   foo : 3,
  },
  {
   foo : 4,
  },
  {
   
  },
  {
   foo : 5,      
  }
 ] 
}

```
The old version's list's foo is 1,2,0
 ```Rust
fn clilst_old_test() -> DpResult<()> {
    let old = json_dir_to_root("src/b3_1_clist_and_mlist/jsons/clist_old", true)?;

    let r = RootIntf::new(old);
    let list = r.list();
    let mut iter = list.iter();

    assert_eq!(iter.next()?.foo(), 1);
    assert_eq!(iter.next()?.foo(), 2);
    assert_eq!(iter.next()?.foo(), 0);
    Ok(())
}
```
The adjusted data's foo is 3,4,5,-1.
The old list is completely replaced by the new list in the adjustment process.
```Rust
fn clilst_new_adjust_test() -> DpResult<()> {
    let old = json_dir_to_root("src/b3_1_clist_and_mlist/jsons/clist_old", true)?;
    let new = json_dir_to_root("src/b3_1_clist_and_mlist/jsons/clist_new", true)?;

    let r = adjust_versions(new, old, true)?;

    let r = RootIntf::new(r);
    let list = r.list();
    let mut iter = list.iter();

    assert_eq!(iter.next()?.foo(), 3);
    assert_eq!(iter.next()?.foo(), 4);
    assert_eq!(iter.next()?.foo(), -1);
    assert_eq!(iter.next()?.foo(), 5);
    Ok(())
}
```
MList is a mutable list.

Modifications in the old data must be preserved in the new version,
so initial objects in the new version's list is completely discarded in the adjustment process,
and replaced by old data's objects.

The old version is below.
```
{
 mlist : [
  "MList",
  [{
   bar : 0,
   baz : 1
  }]
  {
   bar : 1,
   baz : 2
  }
  {
   // not set
   baz : 3,
  }
  {
   bar : 3,
   // not set 
  }
 ]
}
```
The test code
```Rust
fn mlilst_old_test() -> DpResult<()> {
    let old = json_dir_to_root("src/b3_1_clist_and_mlist/jsons/mlist_old", true)?;

    let mut r = RootIntf::new(old);
    let mut list = r.mlist_mut();
    let mut iter = list.iter();

    if let Some((_id, item)) = iter.next(){
        assert_eq!(item.bar(), 1);
        assert_eq!(item.baz(), 2);
    }
    if let Some((_id, item)) = iter.next(){
        assert_eq!(item.bar(), 0); // default value
        assert_eq!(item.baz(), 3);
    }
    if let Some((_id, item)) = iter.next(){
        assert_eq!(item.bar(), 3);
        assert_eq!(item.baz(), 1); // default value
    }
    Ok(())
}
```
The new version...
```
{
 mlist : [
 "MList",
  [{
   //bar is removed,
   baz : 100, // baz's default is changed to 100
   qux : -1, //new value "qux" is defined
  }],
  {
   baz : 1001,
   qux : 2
  }
  {
   baz : 1002,
   qux : 3
  }
 ]
}
```
The new version is completely ignored in the adjustment process.
```Rust
fn mlilst_new_adjust_test() -> DpResult<()> {
    let old = json_dir_to_root("src/b3_1_clist_and_mlist/jsons/mlist_old", true)?;
    let new = json_dir_to_root("src/b3_1_clist_and_mlist/jsons/mlist_new", true)?;

    let r = adjust_versions(new, old, true)?;

    let r = RootIntf::new(r);
    let list = r.mlist();
    let mut iter = list.iter();

    if let Some((_id, item)) = iter.next(){
        //assert_eq!(item.bar(), 1); //bar is removed
        assert_eq!(item.baz(), 2); //old item's baz is preserved
        assert_eq!(item.qux(), -1); //qux is undefined, so the default value is returned
    }
    if let Some((_id, item)) = iter.next(){
        //assert_eq!(item.bar(), 0);
        assert_eq!(item.baz(), 3); //old item's baz is preserved
        assert_eq!(item.qux(), -1);
    }
    if let Some((_id, item)) = iter.next(){
        //assert_eq!(item.bar(), 0);
        assert_eq!(item.baz(), 100); // This baz was not set in the old data, so the default value of the new version returned
        assert_eq!(item.qux(), -1);
    }
    Ok(())
}
```
When the new version's type data is applied to the old data,
the variable "bar" is removed, and the variable "qux" is added.

All the objects in the initial list of the new version is discarded,
and replaced by the old version's objects with the new type data. 
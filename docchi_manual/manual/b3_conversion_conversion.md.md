[prev](b2_save_history_files_directory_composition_and_how_to_handle.md.md)
[index](index.md)
[next](b3_1_clist_and_mlist_clist_and_mlist.md.md)

### 9. Conversion

### Conversion

Docchi File System forces the diff files to be placed with 
 an archived source JSON5 file in the same directory.

When the source is modified, diff files will be invalid unless 
the source file at the time is preserved, and Docchi makes sure
the corresponding archive file is always preserved in the same directory.

When the source is modified, we'll have the current and old source files.
Since we can get the old and the current type data from them,
we can adjust the old data to be compatible with the current version.

In Docchi, if a variable is removed in a new version, 
the corresponding variable in the old data is also removed in the adjustment process, 
and it shouldn't cause any problem.

What about adding a variable?

In Docchi, when we try to get a value from a variable, 
and the value hasn't been modified, its default value will be returned.
When a variable is added in a new version, since the old data doesn't have the variable,
the default value will be returned. It's basically safe, but sometimes, it's not.

Let's pretend we need a variable which is ten times bigger than an old variable.

How should we implement the conversion. We can write,

"If the data file is the old version, multiply the variable by ten and update the variable."

But is that the right way? We have old and new type data, and we could take advantage of them.  

In Docchi, when a variable is undefined in an old data, 
the system can set the special value "undefined" to the variable in the adjustment process.

```json5
{
 oldValue : 10
}
```
 This is the old source file.
```
{
 oldValue? : ["Int", null], 
 
 newValue! : 100, // ! means "can be undefined". 
}
 ```
This is the new version. oldValue's default value has been changed into null
to distinguish if the default value is manually set. 

Since the old data doesn't have the variable "newValue", the content of the variable is 
changed to "undefined" in the adjustment process.

## How to use the value "undefined"?

Docchi generates a source code to access the data. 
The generated code from the new source is this. 
(You don't need to read. It's just a generated code)
```Rust
use docchi::core::intf::*;
use docchi::core::structs::*;
unsafe impl Send for RootIntf {}
unsafe impl Sync for RootIntf {}
#[derive(Debug)]
pub struct RootIntf{
 root : Box<RootObject>,
 ptr : RootObjectPtr,
}
impl RootIntf{
 pub fn new(obj : RootObject) -> RootIntf{
  let mut root = Box::new(obj);
  let ptr = RootObjectPtr::new(root.as_mut());
  RootIntf { root, ptr }
 }
 pub fn root_obj_ref(&self) -> &RootObject{ self.root.as_ref() }
 pub fn root_obj_ref_mut(&mut self) -> &mut RootObject{ self.root.as_mut() }
 pub fn deconstruct(self) -> RootObject{ *self.root }

 pub fn old_value(&self) -> NullOr<i64>{
  let qv = root::get_int(self.ptr, "oldValue").unwrap();
  NullOr::from_qv(qv).unwrap()
 }
 pub fn old_value_def_val(&self) -> NullOr<i64>{
  let qv = root::get_int_def(self.ptr, "oldValue").unwrap();
  NullOr::from_qv(qv).unwrap()
 }
 pub fn set_old_value(&mut self, old_value : NullOr<i64>){
  root::set_int(self.ptr, "oldValue", old_value.into_qv());
 }
 pub fn new_value(&self) -> UndefOr<i64>{
  let qv = root::get_int(self.ptr, "newValue").unwrap();
  UndefOr::from_qv(qv).unwrap()
 }
 pub fn new_value_def_val(&self) -> UndefOr<i64>{
  let qv = root::get_int_def(self.ptr, "newValue").unwrap();
  UndefOr::from_qv(qv).unwrap()
 }
 pub fn set_new_value(&mut self, new_value : UndefOr<i64>){
  root::set_int(self.ptr, "newValue", new_value.into_qv());
 }
}
```
You can use the generated code directly, but we suppose 
a handmade wrapper should be created for more convenience.
```Rust
use docchi::core::structs::{UndefOr, NullOr};
use crate::b3_conversion::new_accessor::RootIntf;
use std::ops::{Deref, DerefMut};

pub(crate) struct NewWrapper {
 root : RootIntf
}

impl NewWrapper {
 pub fn new(root: RootIntf) -> NewWrapper { NewWrapper { root } }

 pub fn new_value(&self) -> i64 {
  Self::new_value_impl(&self.root)
 }

 fn new_value_impl(root: &RootIntf) -> i64 {
  match root.new_value() {
   //If data is the old version, the variable "new_value" will be "undefined" because it was not defined at the time.
   UndefOr::Undefined => {
    match root.old_value(){
     NullOr::Null => root.new_value_def_val().into_value().unwrap(),
     NullOr::Val(v) => v * 10, //new_value is ten times bigger than the old value
    }
   },
   UndefOr::Val(v) => {
    v
   }
  }
 }
}
```
The wrapper doesn't allow to access the variable "oldValue", 
and when new_value is undefined (because the data is old),
the wrapper returns the value ten times bigger than the old_value.

If the newValue is undefined, and the oldValue is unmodified, it returns the default value of the new_value.

Let's pretend we need to define a value again. We name it new_value2. The value is three times bigger than the new_value,
and the default value is 280.
```
{
    oldValue? : ["Int", null],
    //!? means the value can be null or undefined
    newValue!? : ["Int", null],
    newValue2! : 280,
}
```
The wrapper is below.
```Rust
use crate::sample_test::sample_code::version_awareness_accessor2::RootIntf;
use docchi_core::structs::{UndefOr, NullOr, Qv};

pub(crate) struct VeraAccessorWrapper2 {
    root: RootIntf,
}

impl VeraAccessorWrapper2 {
    pub fn new(root: RootIntf) -> VeraAccessorWrapper2 { VeraAccessorWrapper2 { root } }

    pub fn new_value2(&self) -> i64 {
        Self::new_value2_impl(&self.root)
    }

    fn new_value_impl(root: &RootIntf) -> NullOr<i64> {
        match root.new_value() {
            // We call a value "Qv" which can be "null" or "undefined". Qv stands for "Questionable value"
            Qv::Undefined => {
                root.old_value().map(|&old| old * 10)
            },
            Qv::Null => NullOr::Null,
            Qv::Val(v) => { NullOr::Val(v) }
        }
    }

    fn new_value2_impl(root : &RootIntf) -> i64{
        match root.new_value2(){
            UndefOr::Undefined =>{
                match Self::new_value_impl(root){
                    //This value hasn't been modified yet. Returns the current default value.
                    NullOr::Null => root.new_value2_def_val().into_value().unwrap(),
                    //The value has been modified in the older version(s). Convert the value to the current version
                    NullOr::Val(v) =>{
                        v * 3
                    }
                }
            },
            UndefOr::Val(v) =>{
                v
            },
        }
    }
}
```
When new_value2 is undefined, new_value * 3 is returned if it's defined, and if new_value is undefined, old_value * 30 is returned,
but if old_value and new_value haven't been modified, the default value is returned.





[prev](b2_save_history_files_directory_composition_and_how_to_handle.md.md)
[index](index.md)
[next](b3_1_clist_and_mlist_clist_and_mlist.md.md)

[prev](a3_docchi_langs_basics_params_test.rs.md)
[index](index.md)
[next](a4_clist_cil_root.json5.md)

### 2. CList

```json5
{
  // Docchi has three collection types, "CList","MList", and "Table",
  // but the basic foundations are the same.

  clist : ["CList",[{
    name : "a",
    val : 3,
  }],
    {
      name : "b",
      val : 4,
    },
    {
      //name is omitted
      val : 5,
    }
  ],
}

// Explanation:
//
// Docchi's every collection must have "default object" to define the static type of the items of the collection.
//
//   clist : ["CList",[{
//    name : "a",
//    val : 3,
//  }],
//
// Docchi's default object is surrounded by "[{" and "}]".
// The default object defines the variables "name" (String) and "val" (Int).
// "name"'s default value is "a", and "val"'s is 3.
//
// {
//   name : "b",
//   val : 4,
// },
//
// This is the first item of the list.
// "name" is "b" and "val" is 4.
//
// {
//   //name is omitted
//   val : 5,
// }
//
// In the second object, the "name" variable is omitted.
// When a value is omitted, it's default value is used, so the value "name" will be "a".
//
// CList is the immutable list of Docchi.
// Why we need CList?
// We have MList which is the mutable list of Docchi.
// You can use MList and hide the mutable methods.
//
// It's because, these two have different strategies in conversion.
//
// When you need to modify older version's data,
// you can just rewrite clist in the current version's source file.
//
// In the conversion process, older data's CList is just replaced by the current source file's CList.
//
// MList is not replaced in the conversion process (because we need to retain the modified data).
//
// CList is implemented by std::Vec internally.
```


[prev](a3_docchi_langs_basics_params_test.rs.md)
[index](index.md)
[next](a4_clist_cil_root.json5.md)

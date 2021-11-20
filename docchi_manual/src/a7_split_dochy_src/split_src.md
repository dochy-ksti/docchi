Dochy Src can be split into multiple files.

The original source is...
```json5
{
  int : 10,
  list : [
    "CList",
    [{
      v : 0,
    }],
    {
      v : 1,
    }
  ],
  string : "aaaaa"
}
```
It can be splitted into

```
┌─root.json5
├─int.json5
├─list.json5
└─string.json5
```
"root.json5" is empty.
```json5
```
int.json5
```json5
10
```
list.json5
```json5
[
  "CList",
  [{
    v : 0,
  }],
  {
    v : 1,
  }
]
```
string.json5
```json5
"aaaaa"
```
All items in "root.json5" can be split into separated files with some exceptions.

The filename of a split file is considered as a variable name, 
and the content is considered as the value.

Nullable variables can't be separated because "?" can't be used in filenames.
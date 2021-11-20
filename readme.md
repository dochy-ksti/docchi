Dochy is a diff-based data management language to implement
unlimited undo, auto-save for games, and cloud-apps which needs to
retain every change. 

*[User's Manual](https://github.com/dochy-ksti/dochy/blob/master/dochy_manual/manual/index.md)

Dochy is a language, so [the API documentation](https://docs.rs/dochy/) is not very good to learn. 
You may want to read [User's Manual](https://github.com/dochy-ksti/dochy/blob/master/dochy_manual/manual/index.md).

*Demonstration

[Demo](https://github.com/dochy-ksti/dochy_bench)

Test Data
```JSON5
{
  "data0": "oiufsjdsj...", //1 MB of random string
  "data1": "abuisoehg...", //1 MB of random string
  //...
  "data9": "bhsiofdis...", //1 MB of random string
}
```
Pseudo-Code
```text
for i in 0..100{
    modify_one_string(json.get_data_mut(rand.gen_range(0..10)));
    let s = serde_json::to_string(&json);
    let mut file = std::fs::File::create(format!("{}/d{}.json", json_dir, i))?;
    file.write_all(s.as_bytes())?;
}
```
The JSON has ten random 1 MB strings, so the entire JSON file is about 10 MB.

We modified one string and saved as JSON format at a time, and it was repeated 100 times. 
The total amount of the files was about 1 GB.

It means 10 % of the data is modified each time. Data tends to be modified partially, 
so 10 % is not very uncommon setting, I think.

Equivalent Dochy data is created, modified, and saved 100 times.

Dochy saves "diff". Only 1 MB of the modified data is saved at best.

The result is below
```text
JSON
sum of file sizes 1021435957
1906 milliseconds

Dochy
sum of file sizes 173005171
604 milliseconds
```
JSON saved about 1 GB of data and took 1906 milliseconds.

Dochy saved about 173 MB of data and took 604 milliseconds.

Dochy took 17 % of the storage space and about one-third of the time.

Very nice?

Dochy only saved 17 % of the data so of course it's faster.

For comparison, we changed JSON strings to the length of 17 % and run the demo.
```text
JSON(short)
sum of file sizes 173570901
338 milliseconds
```
About the same file size, and JSON was twice as fast as Dochy.

Serde is very fast, so the result is comprehensible.

But I think Dochy's overhead is reasonable, and Dochy can save in non-blocking manner,
so saving time may not worsen user experience. 

*Load Demo

Loading is where Dochy pays the cost. Dochy creates "Diff Hierarchy".

```text
Diff Hierarchy Concept

Diff0(10 MB) - Diff00(1 MB) - Diff000(1 MB)
                                   │
                              Diff001(1 MB)
                                   │
                              Diff002(1 MB)
             - Diff01(5 MB) - Diff010(1 MB)
                                   │
                              Diff011(1 MB)
                                   │
                              Diff012(1 MB)
                                  ...
             - Diff02(10 MB)- Diff020(1 MB)
                                   │
                              Diff021(1 MB)
                                  ...
                  ...
 Diff1(10 MB)
   ... 
```
To load Dochy's diffs, we must load files hierarchically from top to bottom, and apply diffs repeatedly.

We used the default setting of Dochy, and it takes 13 files to load one data at most.

The total file size to load can be 4 times bigger than the biggest diff file (10 MB in this case),
so it's 40 MB.

We searched the deepest file from the hierarchy and loaded the data. 
```text
Dochy 
40 milliseconds

JSON
94 milliseconds

JSON(Short)
16 milliseconds
```
The Dochy's total amount of data is 4 times bigger than JSON's, but more than twice as fast as JSON.

Dochy is a binary data format and efficiently multi-threaded for loading, so it was able to beat Serde, I think.

*How can it be done?

Constructing diff is very costly process in nature, but Rust's Arc(Atomic-Reference-Count-Pointer) makes it very easy.

Dochy's data is cloned on saving, so non-blocking saving can be done. 
Dochy's data consists of Arcs, so the cloning can be done instantly.

Using Arc::make_mut, actual copy of the inner data happens when two different Arcs point to the same object, 
and one of them is modified. When it's modified, two Arcs point to the different objects, so 
comparing two pointers of Arcs is enough to confirm if it's modified. Comparing actual values is not necessary.

And actual copy happens on the part which is to be actually modified. Copying everything is also unnecessary.
Rust's Arc is really magical.

When comparing data and constructing diff, we compare the current object with the cloned object on save, and compare pointers.
it's very fast process.

On the other hand, we didn't do anything special on loading. If it's fast, it owes to Rayon.
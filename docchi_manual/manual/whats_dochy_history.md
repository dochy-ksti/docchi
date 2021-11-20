[prev](load_dochy_file.md)
[index](index.md)
[next](save_history_file_test.md)

### 7. Dochy History


Dochy File can be used like a normal data file,
while Dochy History is intended to handle sequential (often small) changes,
like "undo", "auto-save", etc.

Basically, if you want to use Dochy History, you should also use
Dochy File to save data permanently.
Otherwise, just retain Dochy History files forever.
If you don't remove, it's permanent (but storage drives have their limits...).

Dochy History files have dependencies on previously saved files,
so removing single file is basically impossible, and retaining only single file is also impossible.

You must remove all dependants, and retain all dependencies.
This library has a removing policy, "retain last N files and dependencies".
Besides, you can specify files to retain and remove all the other files without their dependencies.



[prev](load_dochy_file.md)
[index](index.md)
[next](save_history_file_test.md)

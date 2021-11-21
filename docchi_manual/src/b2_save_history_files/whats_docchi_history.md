
Docchi File can be used like a normal data file,
while Docchi History is intended to handle sequential (often small) changes,
like "undo", "auto-save", etc.

Basically, if you want to use Docchi History, you should also use
Docchi File to save data permanently.
Otherwise, just retain Docchi History files forever.
If you don't remove, it's permanent (but storage drives have their limits...).

Docchi History files have dependencies on previously saved files,
so removing single file is basically impossible, and retaining only single file is also impossible.

You must remove all dependants, and retain all dependencies.
This library has a removing policy, "retain last N files and dependencies".
Besides, you can specify files to retain and remove all the other files without their dependencies.

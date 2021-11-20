This is the typical directory composition.

─ proj_dir ┬─ history_dit
           ├─ save_dir
           └─ src_dir

Maybe you want to use multiple data set for your project.

─ proj_dir ┬─ data_a_dir ┬─ history_dir_a
           │             ├─ save_dir_a
           │             └─ src_dir_a
           └─ data_b_dir ┬─ history_dir_b
                         ├─ save_dir_b
                         └─ src_dir_b

We recommend these compositions, but you don't need to obey them.
There's no limitation about directory compositions in Dochy.

But history_dir, save_dir, and src_dir need to be dedicated directories,
and let Dochy manage them.

Other than API, you can create, modify and remove source JSON5 files in src_dir,
and remove .dochy files in save_dir.

In other words, you aren't allowed to add, modify, and remove files in save and history dirs,
except .dochy files in save_dir.

You can add and remove files in save and history dirs with API.
(Currently no functions to modify are available.)

You can remove entire hash_dirs in save and history dirs.
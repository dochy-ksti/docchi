## Docchi User's Manual

Docchi is a static JSON-like data format.

It can efficiently store "diff" of the data.

It's designed to implement auto-save, unlimited undo, and
applications which want to retain every change of the data, like cloud apps.

Docchi is a language, so [the API documentation](https://docs.rs/docchi/) is not very good to learn.
This user's manual should be good to go.

Every samplecode is in the [docchi_manual](https://github.com/dochy-ksti/docchi/tree/master/docchi_manual/src) crate.


### [0. Hello World](a1_hello_world_hello_world.md.md)

### 1. Docchi Lang

#### 　[1-1. Basic Params](a3_docchi_langs_basics_docchi_params_root.json5.md)

##### 　　[1-1-1. Generate Source Code To Access Params](a3_docchi_langs_basics_docchi_params_generate.rs.md)

##### 　　[1-1-2. Use Generated Source Code](a3_docchi_langs_basics_params_test.rs.md)

### [2. CList](a4_clist_clist_root.json5.md)

#### 　[2-1. Cil (Const Inner List)](a4_clist_cil_root.json5.md)

#### 　[2-2. Use Clist And Cil](a4_clist_use_cil.rs.md)

### [3. MList](a5_mlist_mlist_root.json5.md)

#### 　[3-1. Mil(Mut Inner List)](a5_mlist_mil_root.json5.md)

#### 　[3-2. Use Mil](a5_mlist_use_mil.rs.md)

### [4. Table](a6_table_table_root.json5.md)

#### 　[4-1. Use Table](a6_table_use_table.rs.md)

### [5. Split Docchi Src](a7_split_docchi_src_split_src.md.md)

### [6. Docchi File](b1_save_docchi_files_save_docchi_files.md.md)

#### 　[6-1. Save Docchi File](b1_save_docchi_files_save_docchi_file_test.rs.md)

#### 　[6-2. Load And Remove Docchi File](b1_save_docchi_files_load_docchi_file.rs.md)

### [7. Docchi History](b2_save_history_files_whats_docchi_history.md.md)

#### 　[7-1. Save History File](b2_save_history_files_save_history_file_test.rs.md)

#### 　[7-2. Load And Remove History File](b2_save_history_files_load_history_file_test.rs.md)

#### 　[7-3. Algorithm Of Docchi History](b2_save_history_files_algorithm_of_history.md.md)

### [8. Directory Composition And How To Handle](b2_save_history_files_directory_composition_and_how_to_handle.md.md)

### [9. Conversion](b3_conversion_conversion.md.md)

#### 　[9-1. Convert CList And MList](b3_1_clist_and_mlist_clist_and_mlist.md.md)

#### 　[9-2. Undefined MList](b3_1_clist_and_mlist_separate_undefined_list.md.md)


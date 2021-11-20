## Dochy User's Manual

Dochy is a static JSON-like data format.

It can efficiently store "diff" of the data.

It's designed to implement auto-save, unlimited undo, and
applications which want to retain every change of the data, like cloud apps.

Dochy is a language, so [the API documentation](https://docs.rs/dochy/) is not very good to learn.
This user's manual should be good to go.


### 1. Dochy Lang

#### 　[1-1. Basic Params](root.md)

##### 　　[1-1-1. Generate Source Code To Access Params](dochy_params_generate.md)

##### 　　[1-1-2. Use Generated Source Code](params_test.md)

### [2. CList](root.md)

#### 　[2-1. Cil (Const Inner List)](root.md)

#### 　[2-2. Use Clist And Cil](use_cil.md)

### [3. MList](root.md)

#### 　[3-1. Mil(Mut Inner List)](root.md)

#### 　[3-2. Use Mil](use_mil.md)

### [4. Table](root.md)

#### 　[4-1. Use Table](use_table.md)

### [5. Split Dochy Src](split_src.md)

### [6. Dochy File](save_dochy_files.md)

#### 　[6-1. Save Dochy File](save_dochy_file_test.md)

#### 　[6-2. Load And Remove Dochy File](load_dochy_file.md)

### [7. Dochy History](whats_dochy_history.md)

#### 　[7-1. Save History File](save_history_file_test.md)

#### 　[7-2. Load And Remove History File](load_history_file_test.md)

#### 　[7-3. Algorithm Of Dochy History](algorithm_of_history.md)

### [8. Directory Composition And How To Handle](directory_composition_and_how_to_handle.md)

### [9. Conversion](conversion.md)

#### 　[9-1. Convert CList And MList](clist_and_mlist.md)

#### 　[9-2. Undefined MList](separate_undefined_list.md)


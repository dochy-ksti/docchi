[prev](a3_docchi_langs_basics_docchi_params_root.json5.md)
[index](index.md)
[next](a3_docchi_langs_basics_params_test.rs.md)

### 1-1-1. Generate Source Code To Access Params

```rust
use docchi::core::structs::{RootObject};
use docchi::core::json_dir_to_root;
use docchi::intf::generate_interface;
use anyhow::Result;

#[test]
fn docchi_params_generate() -> Result<()> {

    // You can convert Docchi Src into RootObject with json_dir_to_root
    // RootObject is the object representation of Docchi Src
    let mut root_obj : RootObject = json_dir_to_root(
        /* dir_path */ "src/a3_docchi_langs_basics/docchi_params",
        /* validation */ true)?;

    // Validation is useful. You should validate your Docchi Src,
    // but do it twice is meaningless. I think the cost of validation is negligible, though.

    // "json_dir_to_root" returns CoreResult, which is the result type of the module "docchi_core".
    // It's automatically converted to DpResult with the "?" operator.
    // In Docchi, every result type is automatically converted to DpResult,
    // so basically, users of this library only need DpResult.

    // "generate_interface" analyzes RootObject and generate the source code to access the RootObject in Rust
    let source_file = generate_interface(&mut root_obj);

    // writes the source file as a Rust source file.
    std::fs::write(
        "src/a3_docchi_langs_basics/docchi_params_accessor.rs",
        &source_file.to_string(),
    ).unwrap();

    // Generating source code in [test] code is not sophisticated, but easy. I like it.

    // This entire function can be written with one function.

    // generate_accessor_from_json_dir("src/a3_docchi_langs_basics/docchi_params", "src/a3_docchi_langs_basics/docchi_params_accessor.rs", true)?;

    // We'll use it hereafter.
    Ok(())
}

```


[prev](a3_docchi_langs_basics_docchi_params_root.json5.md)
[index](index.md)
[next](a3_docchi_langs_basics_params_test.rs.md)



#[cfg(test)]
mod tests {
    use anyhow::Result;
    use crate::diff::util::generate_intf_src::generate_intf_src;


    #[test]
    fn generate_for_test_big_structure2() -> Result<()>{
        let json_dir_path = "src/diff/diff_big_structure/";
        let _root_obj = generate_intf_src(json_dir_path, "src/diff/generated_test_big_structure.rs")?;
        Ok(())
    }


}
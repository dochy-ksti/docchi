use docchi::core::{json_dir_to_root};
use docchi::intf::generate_interface;

#[test]
fn generate_version_awareness() {
    match json_dir_to_root("src/sample_test/sample_code_json/version_aware/new", true) {
        Ok(mut a) => {
            let ans = generate_interface(&mut a);
            std::fs::write(
                "src/sample_test/sample_code/version_awareness_accessor.rs",
                &ans.to_string(),
            )
            .unwrap();
        }
        Err(_e) => {
            assert!(false);
        }
    }
}
#[test]
fn generate_version_awareness2() {
    match json_dir_to_root("src/sample_test/sample_code_json/version_aware/new2", true) {
        Ok(mut a) => {
            let ans = generate_interface(&mut a);
            std::fs::write(
                "src/sample_test/sample_code/version_awareness_accessor2.rs",
                &ans.to_string(),
            )
                .unwrap();
        }
        Err(_e) => {
            assert!(false);
        }
    }
}


pub(crate) fn with_old(name : &str, is_old : bool) -> String {
    if is_old {
        format!("{}_old", name)
    } else {
        name.to_string()
    }
}

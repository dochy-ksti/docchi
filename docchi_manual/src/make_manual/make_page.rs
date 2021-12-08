use crate::make_manual::get_content::{get_content, get_md_filename};
use anyhow::Result;
use docchi::core::structs::NullOr;

pub(crate) fn make_page(prev_src: NullOr<&str>, next_src: NullOr<&str>, title : &str, src : &str)
                        -> Result<String>{

    let mut r = String::new();
    {
        let r = &mut r;
        if let NullOr::Val(prev) = prev_src {
            l(r, &format!("[prev]({})", get_md_filename(prev)?));
        }
        l(r, "[index](index.md)");
        if let NullOr::Val(next) = next_src {
            l(r, &format!("[next]({})", get_md_filename(next)?));
        }
        r.push('\n');
        l(r, &format!("### {}", title));
        r.push('\n');
        let content = get_content(src)?;
        l(r, &content);
        r.push('\n');
        r.push('\n');
        if let NullOr::Val(prev) = prev_src {
            l(r, &format!("[prev]({})", get_md_filename(prev)?));
        }
        l(r, "[index](index.md)");
        if let NullOr::Val(next) = next_src {
            l(r, &format!("[next]({})", get_md_filename(next)?));
        }
    }
    Ok(r)
}

pub(crate) fn l(r : &mut String, s : &str){
    r.push_str(s);
    r.push('\n');
}
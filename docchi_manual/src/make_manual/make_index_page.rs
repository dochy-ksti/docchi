use crate::make_manual::manual_builder::ManualBuilderItem;
use crate::make_manual::make_page::l;
use crate::make_manual::get_content::get_md_filename;
use anyhow::Result;
use docchi::core::structs::NullOr;

pub(crate) fn make_index_page(vec : &[ManualBuilderItem]) -> Result<String>{
    let mut r = String::new();
    {
        let r = &mut r;
        l(r, "## Docchi User's Manual");
        l(r, "");
        l(r, introduction());
        l(r, "");

        for item in vec {
            let src = item.src();
            if let NullOr::Val(src) = src {
                match get_indent(item.title()){
                    0 => l(r, &format!("### [{}]({})", item.title(), get_md_filename(src)?)),
                    1 => l(r, &format!("#### 　[{}]({})", item.title(), get_md_filename(src)?)),
                    2 => l(r, &format!("##### 　　[{}]({})", item.title(), get_md_filename(src)?)),
                    _ => unreachable!(),
                }

            } else{
                l(r, &format!("### {}", item.title()));
            }
            l(r, "");
        }
    }
    Ok(r)
}

fn introduction() -> &'static str{
r#"Docchi is a static JSON-like data format.

It can efficiently store "diff" of the data.

It's designed to implement auto-save, unlimited undo, and
applications which want to retain every change of the data, like cloud apps.

Docchi is a language, so [the API documentation](https://docs.rs/docchi/) is not very good to learn.
This user's manual should be good to go.

Every sample code is in the [docchi_manual](https://github.com/dochy-ksti/docchi/tree/master/docchi_manual/src) crate.
"#
}

fn get_indent(title : &str) -> usize{
    title.chars()
        .take_while(|c| *c != '.')
        .filter(|c| *c == '-')
        .count()
}
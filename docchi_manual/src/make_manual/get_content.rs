use anyhow::Result;
use std::path::Path;

pub(crate) fn get_content(src : &str) -> Result<String>{
    println!("{}", src);
    let s = std::fs::read_to_string(format!("src/{}", src))?;
    let path = Path::new(src);
    let ext = path.extension().unwrap().to_string_lossy().to_string();
    let s = match ext.as_str(){
        "json5" => format!("```json5\n{}\n```", s),
        "rs" => format!("```rust\n{}\n```", s),
        "md" => s,
        _ => unreachable!(),
    };
    Ok(s)
}



pub(crate) fn get_md_filename(src : &str) -> Result<String>{
    Ok(format!("{}.md", src.replace('/',"_")))
}
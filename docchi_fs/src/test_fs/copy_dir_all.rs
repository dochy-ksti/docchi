use std::fs;
use std::path::{Path, PathBuf};
use crate::error::FsResult;

/// ディレクトリとファイルをコピー。
/// ネットに落ちていたコードを微修正
/// 普通に再帰つかったほうがわかりやすそうだが・・・
pub fn copy_dir_all<U: AsRef<Path>, V: AsRef<Path>>(from: U, to: V) -> FsResult<()> {
    let mut stack = Vec::new();
    stack.push(PathBuf::from(from.as_ref()));

    let output_root = PathBuf::from(to.as_ref());
    let input_root = PathBuf::from(from.as_ref()).components().count();

    while let Some(working_path) = stack.pop() {
        //println!("process: {:?}", &working_path);

        // Generate a relative path
        let src: PathBuf = working_path.components().skip(input_root).collect();

        // Create a destination if missing
        let dest = if src.components().count() == 0 {
            output_root.clone()
        } else {
            output_root.join(&src)
        };
        if fs::metadata(&dest).is_err() {
            //println!(" mkdir: {:?}", dest);
            fs::create_dir_all(&dest)?;
        }

        for entry in fs::read_dir(working_path)? {
            let entry = entry?;
            let file_type = entry.file_type()?;
            let path = entry.path();

            if file_type.is_dir(){
                stack.push(path);
            } else if file_type.is_file(){
                match path.file_name() {
                    Some(filename) => {
                        let dest_path = dest.join(filename);
                        fs::copy(&path, &dest_path)?;
                    }
                    None => {
                        //Err("file has no name")?
                    }
                }
            }
        }
    }

    Ok(())
}
use crate::history::FileNameProps;
use std::sync::Weak;

#[derive(Debug, Clone)]
pub struct CurrentRootObjInfo {
    current_root_id: Weak<()>,
    current_base_file: FileNameProps,
}

impl CurrentRootObjInfo {
    pub fn new(current_root_id: Weak<()>, current_base_file: FileNameProps) -> CurrentRootObjInfo {
        CurrentRootObjInfo { current_root_id, current_base_file}
    }

    pub fn current_root_id(&self) -> &Weak<()>{ &self.current_root_id }
    pub fn current_base_file(&self) -> &FileNameProps{ &self.current_base_file }
}
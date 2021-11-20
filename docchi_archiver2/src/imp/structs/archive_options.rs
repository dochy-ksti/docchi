use std::collections::HashSet;
use crate::ArcResult;


#[derive(Debug, Clone)]
pub struct ArchiveOptions {
    pub(crate) extensions_archived : HashSet<String>,
    pub(crate) archive_subfolders : bool,
}

impl ArchiveOptions {

    /// If the file with the extension is archived. true if "extension" is empty
    pub fn is_archived(&self, extension : &str) -> bool{
        if self.extensions_archived.len() == 0{
            true
        } else{
            self.extensions_archived.contains(extension)
        }
    }

    /// If the subfolders are archived
    pub fn archive_subfolders(&self) -> bool{ self.archive_subfolders }

    /// construct with the default values.
    /// In the default configuration, every file is archived in the folder and the subfolders.
    pub fn new() -> ArchiveOptions {
        ArchiveOptionsBuilder::new().build().unwrap()
    }

    /// Construct ArchiveOptions from the builder
    pub fn from(builder : ArchiveOptionsBuilder) -> ArcResult<ArchiveOptions>{
        let extensions_archived : HashSet<String> = builder
            .extensions_archived.into_iter()
            .map(|s| s.to_string()).collect();

        let archive_subfolders = builder.archive_subfolders;
        Ok(ArchiveOptions {
            extensions_archived,
            archive_subfolders,
        })
    }
}

///Construct ArchiveOptions
#[derive(Debug, Clone)]
pub struct ArchiveOptionsBuilder {
    extensions_archived : HashSet<String>,
    archive_subfolders : bool,
}

impl ArchiveOptionsBuilder{
    pub fn new() -> ArchiveOptionsBuilder{
        ArchiveOptionsBuilder{
            extensions_archived : HashSet::new(),
            archive_subfolders : true
        }
    }
    pub fn add_extension(mut self, ext : &str) -> ArchiveOptionsBuilder{
        self.extensions_archived.insert(ext.to_string());
        self
    }
    pub fn archive_subfolders(mut self, archive_subfolders : bool) -> ArchiveOptionsBuilder{
        self.archive_subfolders = archive_subfolders;
        self
    }

    pub fn build(self) -> ArcResult<ArchiveOptions>{
        ArchiveOptions::from(self)
    }
}


/// The default name of generated archive files
pub const ARCHIVE_DEFAULT_NAME : &'static str = "src.archive";

/// the file name of the files which contains "created time"
pub const CREATED_TIME_FILE_NAME : &'static str = "created_time.dat";

pub(crate) fn is_reserved_filename(filename : &str) -> bool{
    filename == ARCHIVE_DEFAULT_NAME || filename == CREATED_TIME_FILE_NAME
}
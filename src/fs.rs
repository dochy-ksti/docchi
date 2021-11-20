pub mod common{
    pub use docchi_fs::common::CurrentSrc;

    pub use docchi_fs::common::hash_dir_path;
    pub use docchi_fs::common::hash_dir_paths;
    pub use docchi_fs::common::get_hash_times;
    pub use docchi_fs::common::remove_hash_dir;


    pub use docchi_fs::common::reserved_filename;

    pub use docchi_fs::common::FileData;

    //pub use docchi_fs::common::read_archive;
    //pub use docchi_fs::common::load_archive;


    // pub mod hash{
    //     pub use docchi_fs::common::hash::folder_name_to_hash;
    //     pub use docchi_fs::common::hash::hash_to_folder_name;
    // }
}

pub mod history{
    pub use docchi_fs::history::HistoryOptions;
    pub use docchi_fs::history::CumulativeOptions;
    pub use docchi_fs::history::HistoryOptionsBuilder;
    pub use docchi_fs::history::CumulativeOptionsBuilder;

    pub use docchi_fs::history::FileHistory;
    pub use docchi_fs::history::HistoryRemover;

    pub use docchi_fs::history::FileHistories;

    pub use docchi_fs::history::HistoryFileData;

    pub use docchi_fs::history::calc_filename;
    pub use docchi_fs::history::FileNameProps;

    pub use docchi_fs::history::save_history_file;
    pub use docchi_fs::history::save_history_file_nb;
    pub use docchi_fs::history::save_history_file_nb_if_vacant;

    pub use docchi_fs::history::list_histories;
    pub use docchi_fs::history::load_history_file;
    pub use docchi_fs::history::load_history_file_data;

    pub use docchi_fs::history::set_current_root_obj_info;
    pub use docchi_fs::history::get_peekable_info;
    pub use docchi_fs::history::init_dochy_cache_us;
    pub use docchi_fs::history::PeekableCacheInfo;
    pub use docchi_fs::history::CurrentRootObjInfo;

    pub use docchi_fs::history::HistoryInfo;
}

pub mod filesys{
    pub use docchi_fs::filesys::save_dochy_file;
    pub use docchi_fs::filesys::save_dochy_file_nb;
    pub use docchi_fs::filesys::list_dochy_files;
    pub use docchi_fs::filesys::load_dochy_file;
    pub use docchi_fs::filesys::remove_dochy_file;
    pub use docchi_fs::filesys::force_update_and_get_info_us;
    pub use docchi_fs::filesys::SaveDirInfo;
}


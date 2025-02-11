use once_cell::sync::OnceCell;
use std::env;
use std::path::PathBuf;

pub static MIN_HASH_LENGTH: usize = 3;
pub static VCS_FOLDER: &str = ".chak";

pub static VCS_IGNORE_FILE: &str = ".ignore";

pub static CURRENT_DIR: OnceCell<PathBuf> = OnceCell::new();

static TEST_FOLD: &str = "example";

pub fn get_project_dir() -> &'static PathBuf {
    CURRENT_DIR.get_or_init(|| {
        env::current_dir()
            .expect("Could not get current directory")
            .join(TEST_FOLD)
    })
}

pub fn get_vcs_fold() -> PathBuf {
    get_project_dir().join(VCS_FOLDER)
}

pub fn blob_fold() -> PathBuf {
    get_vcs_fold().join("blobs")
}

pub fn versions_fold() -> PathBuf {get_vcs_fold().join("versions")}

pub fn trees_fold() -> PathBuf {get_vcs_fold().join("trees") }

pub fn staging_area_fold() -> PathBuf {
    get_vcs_fold().join("staging_area")
}

pub fn commits_fold() -> PathBuf {
    get_vcs_fold().join("commits")
}

pub fn history_fold() -> PathBuf {
    get_vcs_fold().join("history")
}


pub fn essentials_folds() -> Vec<PathBuf> {
   vec![blob_fold(), versions_fold(), trees_fold(), staging_area_fold(), commits_fold(), history_fold()]
}
use std::env;
use once_cell::sync::OnceCell;
use std::path::{Path, PathBuf};

pub static  MIN_HASH_LENGTH: usize = 3;
pub static VCS_FOLDER: &str = ".chak";

pub static  VCS_IGNORE_FILE: &str = ".ignore";

pub static CURRENT_DIR: OnceCell<PathBuf> = OnceCell::new();

static  TEST_FOLD: &str = "example";

pub fn get_current_dir() -> &'static PathBuf {
    CURRENT_DIR.get_or_init(|| {
        env::current_dir().expect("Could not get current directory")
            .join(TEST_FOLD)
    })

}

pub fn blob_fold() -> PathBuf {
    get_current_dir().join(VCS_FOLDER).join("store").join("blobs")
}

pub fn version_fold() -> PathBuf {
    get_current_dir().join(VCS_FOLDER).join("store").join("blobs").join("versions")
}
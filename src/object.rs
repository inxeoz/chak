use std::path::PathBuf;
pub trait ObjectTraits {
 //   type Output: HashPointerCommonTraits; // Ensures the returned type implements the trait
    fn containing_folder() -> PathBuf;
    // fn containing_folder_() -> PathBuf;
}
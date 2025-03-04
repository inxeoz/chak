use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use serde::de::DeserializeOwned;
use crate::custom_error::ChakError;
pub trait HashPointerOwn {
    type Output: HashPointerTraits; // Ensures the returned type implements the trait
    fn own<T: HashPointerTraits>(hash_pointer: &T) -> Result<Self::Output, ChakError>;
}
pub trait HashPointerTraits {
    fn replace(&mut self, pointer: &Self);
    // fn update_hash(&mut self, content: String);
    fn get_fold_name(&self) -> String;
    fn get_file_name(&self) -> String;
    fn get_one_hash(&self) -> String;
    fn get_path(&self) -> PathBuf;
    fn set_fold_name(&mut self, fold_name: String);
    fn set_file_name(&mut self, file_name: String);
  }


#[macro_export] macro_rules! impl_hash_pointer_common_traits {
    ($t:ty) => {
        impl $t {
             fn _own<T: HashPointerTraits>(hash_pointer: &T) -> Self {
                Self {
                        fold_name: hash_pointer.get_fold_name(),
                        file_name: hash_pointer.get_file_name(),
                     }
            }

        }

        impl PartialEq for $t {
            fn eq(&self, other: &Self) -> bool {
                self.get_one_hash() == other.get_one_hash()
            }
        }

        impl PartialOrd for $t {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Ord for $t {
            fn cmp(&self, other: &Self) -> Ordering {
                self.get_fold_name()
                    .cmp(&other.get_fold_name())
                    .then_with(|| self.get_file_name().cmp(&other.get_file_name()))
            }
        }

        impl HashPointerTraits for $t {


            fn replace(&mut self, pointer: &Self) {
                self.set_fold_name(pointer.get_fold_name());
                self.set_file_name(pointer.get_file_name());
            }

            // fn update_hash(&mut self, content: String) {
            //     let hash_content:HashPointer  = hash_from_content(&content + &self.get_one_hash().as_str());
            //     self.replace(&hash_content);
            // }

            fn get_fold_name(&self) -> String {
                self.fold_name.clone()
            }

            fn get_file_name(&self) -> String {
                self.file_name.clone()
            }

            fn get_one_hash(&self) -> String {
                self.fold_name.clone() + &self.file_name
            }

            fn get_path(&self) -> PathBuf {

                PathBuf::from(&self.get_fold_name()).join(&self.get_file_name())
            }

            fn set_fold_name(&mut self, fold_name: String) {
                self.fold_name = fold_name;
            }

            fn set_file_name(&mut self, file_name: String) {
                self.file_name = file_name;
            }

        }
    };
}
#[derive(Serialize, Deserialize, Debug, Clone, Eq)]
pub struct HashPointer {
    fold_name: String,
    file_name: String,
}
impl_hash_pointer_common_traits!(HashPointer);

impl HashPointer {
    pub fn new(fold_name: String, file_name: String) -> Self {
        Self { fold_name, file_name }
    }
}

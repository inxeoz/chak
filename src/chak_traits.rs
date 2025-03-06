
use std::fs;
use crate::custom_error::ChakError;
use crate::hash_pointer::{HashPointer};
use crate::object::ObjectTraits;
use std::path::{Path, PathBuf};
use crate::restricted;

pub trait ChakPointerTraits:restricted::RestrictedNew {

    type CorrespondenceObject:ObjectTraits;
   // type Output: ChakPointerTraits + HashPointerTraits; // Ensures the returned type implements the trait
    // fn verify_and_own<T: HashPointerCommonTraits>(hash_pointer: &T) -> Result<Self::Output, ChakError>;
    fn own<T: HashPointerTraits>(hash_pointer: &T) -> Result<Self, ChakError> where Self: Sized {
        if Self::verified_path(hash_pointer).is_some() {
            Ok(
               Self::new(
                   hash_pointer.get_fold_name(),
                   hash_pointer.get_file_name(),
               )
            // Self::new(hash_pointer.get_fold_name(), hash_pointer.get_file_name())
            )
        } else {
            Err(ChakError::CustomError(" hash pointer not found".to_string()))
        }
    }


    fn verified_path<T: HashPointerTraits>(hash_pointer: &T) -> Option<PathBuf> {

        let to_be_verified = Self::CorrespondenceObject::containing_folder().join(hash_pointer.get_path());
        if to_be_verified.exists() {
            Some(to_be_verified)
        }else {
            None
        }
    }
    fn remove_pointer_existence<T: HashPointerTraits>(hash_pointer: &T) {
        if let Some(verified_path) = Self::verified_path(hash_pointer) {
            if let Err(e) = fs::remove_file(verified_path) {
                eprintln!("Error removing file: {:?}", e);
            }
        }
    }

    fn get_containing_folder() -> PathBuf  {
        Self::CorrespondenceObject::containing_folder()
    }

  }


pub trait HashPointerTraits {
    fn new(fold_name: String, file_name: String) -> HashPointer {
        HashPointer {
            fold_name,
            file_name,
        }
    }
    fn get_one_hash(&self) -> String {
        self.get_fold_name() + &self.get_file_name()
    }
    fn get_path(&self) -> PathBuf {
        PathBuf::from(&self.get_fold_name()).join(&self.get_file_name())
    }

    //////////////////////////////////
    fn get_fold_name(&self) -> String;
    fn get_file_name(&self) -> String;
}

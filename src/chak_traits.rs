use crate::config::get_commits_fold_path;
use crate::custom_error::ChakError;
use crate::hash_pointer::HashPointer;
use crate::restricted;
use crate::util::serialize_struct;
use clap::builder::Str;
use serde::Serialize;
use std::fs;
use std::path::PathBuf;

pub trait ChakPointerTraits: restricted::RestrictedNew {
    type CorrespondenceObject: ObjectCommonTraits;
    // type Output: ChakPointerTraits + HashPointerTraits; // Ensures the returned type implements the trait
    // fn verify_and_own<T: HashPointerCommonTraits>(hash_pointer: &T) -> Result<Self::Output, ChakError>;
    fn own<T: HashPointerTraits>(hash_pointer: &T) -> Result<Self, ChakError>
    where
        Self: Sized,
    {
        if Self::verified_path(hash_pointer).is_some() {
            Ok(
                Self::new(hash_pointer.get_fold_name(), hash_pointer.get_file_name()), // Self::new(hash_pointer.get_fold_name(), hash_pointer.get_file_name())
            )
        } else {
            Err(ChakError::HashPointerNotFound(format!(
                "hash pointer not found\nin path {}",
                Self::get_containing_folder().display()
            )))
        }
    }

    fn verified_path<T: HashPointerTraits>(hash_pointer: &T) -> Option<PathBuf> {
        let to_be_verified =
            Self::CorrespondenceObject::containing_folder().join(hash_pointer.get_path());
        if to_be_verified.exists() {
            Some(to_be_verified)
        } else {
            None
        }
    }
    fn remove_pointer_existence<T: HashPointerTraits>(hash_pointer: &T) -> Result<(), ChakError> {
        if let Some(verified_path) = Self::verified_path(hash_pointer) {
            fs::remove_file(verified_path)?;
        }
        Ok(())
    }

    fn get_containing_folder() -> PathBuf {
        Self::CorrespondenceObject::containing_folder()
    }
}

pub trait HashPointerTraits {
    fn get_one_hash(&self) -> String {
        self.get_fold_name() + &self.get_file_name()
    }
    fn get_path(&self) -> PathBuf {
        PathBuf::from(&self.get_fold_name()).join(&self.get_file_name())
    }

    fn projection_to_hash_pointer(&self) -> HashPointer {
        HashPointer {
            fold_name: self.get_fold_name(),
            file_name: self.get_file_name(),
        }
    }

    //////////////////////////////////
    fn get_fold_name(&self) -> String;
    fn get_file_name(&self) -> String;
}

pub trait ObjectCommonTraits: Serialize {

    fn serialize_self(&self) -> Result<String, ChakError> where Self: Sized {
        serialize_struct(self)
    }

    fn accumulated_hash_pointer(&self) -> Result<HashPointer, ChakError> where Self: Sized {
        let serialized_content = self.serialize_self()?;
        let hash_pointer = HashPointer::from_string(&serialized_content);
        Ok(hash_pointer)
    }

    fn serialized_and_hash(&self) -> Result<(String, HashPointer), ChakError> where Self: Sized {
        Ok((self.serialize_self()?, self.accumulated_hash_pointer()?))
    }
    fn containing_folder() -> PathBuf;
}

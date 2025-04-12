
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use crate::chak_traits::HashPointerTraits;

#[derive(Serialize, Deserialize, Debug, Clone, Eq)]
pub struct HashPointer {
    pub(crate) fold_name: String,
    pub(crate) file_name: String,
}

impl PartialEq for HashPointer {
    fn eq(&self, other: &Self) -> bool {
        self.get_one_hash() == other.get_one_hash()
    }
}

impl PartialOrd for HashPointer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HashPointer {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_fold_name()
            .cmp(&other.get_fold_name())
            .then_with(|| self.get_file_name().cmp(&other.get_file_name()))
    }
}

impl HashPointerTraits for HashPointer {
    fn get_fold_name(&self) -> String {
        self.fold_name.clone()
    }

    fn get_file_name(&self) -> String {
        self.file_name.clone()
    }
}


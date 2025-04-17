use crate::hash_pointer::HashPointer;


#[macro_export] macro_rules! impl_pointer_common_traits {
    ($t:ty) => {


        impl std::hash::Hash for $t {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.fold_name.hash(state);
                self.file_name.hash(state);
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
            fn get_fold_name(&self) -> String {
                     self.fold_name.clone()
            }

            fn get_file_name(&self) -> String {
                    self.file_name.clone()
            }

        }

    }

}


#[macro_export] macro_rules! impl_pointer_common_traits_ref_object {
    ($t:ty, $CorrespondenceObjectType:ty) => {
        impl ChakPointerTraits for $t {
                type CorrespondenceObject = $CorrespondenceObjectType;
        }

         impl restricted::RestrictedNew for $t {
            /// Enforces new(), but calls an internal private method
            fn new(fold_name: String, file_name: String) -> Self {
                Self::create(fold_name, file_name)
            }
        }


        impl $t {
            /// Private method: Only callable inside the struct or trait
            fn create(fold_name: String, file_name: String) -> Self {
                Self {
                    fold_name,
                    file_name,
                }
            }
        }
    }
}
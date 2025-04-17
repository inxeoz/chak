use crate::hash_pointer::HashPointer;


#[macro_export] macro_rules! impl_hash_pointer_common_traits {
    ($t:ty, $CorrespondenceObjectType:ty) => {

    //    use crate::hash_pointer::HashPointerTraits;

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

            // fn projection_to_hash_pointer(&self) -> HashPointer {
            //
            //          HashPointer {
            //             fold_name: self.get_fold_name(),
            //             file_name: self.get_file_name(),
            //             }
            //     }

        }

        impl ChakPointerTraits for $t {
                type CorrespondenceObject = $CorrespondenceObjectType;
              //  type Output = $t;
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
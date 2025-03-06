pub trait RestrictedNew {
    fn new(fold_name: String, file_name: String) -> Self;
}

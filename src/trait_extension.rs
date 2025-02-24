use std::path::Path;

pub trait PathExt {
     fn is_empty_dir(&self) -> bool;
}

impl PathExt for Path {
    fn is_empty_dir(&self) -> bool {
        std::fs::read_dir(self)
            .map(|mut d| d.next().is_none())
            .unwrap_or(false)
    }
}

use std::path::Path;

pub trait FsHandle {
    fn create_dir_if_not_exist(&self) -> bool;

    fn write_content(&self, contents: impl AsRef<[u8]>);
}

impl<T> FsHandle for T
where
    T: AsRef<Path>,
{
    fn create_dir_if_not_exist(&self) -> bool {
        if !self.as_ref().exists() {
            std::fs::create_dir_all(self).unwrap();
            true
        } else {
            false
        }
    }

    fn write_content(&self, contents: impl AsRef<[u8]>) {
        println!("{:#?}", self.as_ref().to_str());
        std::fs::write(self.as_ref(), contents).unwrap();
    }
}

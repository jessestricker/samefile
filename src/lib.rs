use std::fs::File;
use std::io;
use std::path::Path;

#[cfg(windows)]
pub mod windows;

#[cfg(windows)]
use windows as imp;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct FileId(imp::FileId);

pub fn file_id(path: impl AsRef<Path>) -> Result<FileId, io::Error> {
    let inner = imp::file_id(path.as_ref())?;
    Ok(FileId(inner))
}

pub fn file_id_from_file(file: &File) -> Result<FileId, io::Error> {
    let inner = imp::file_id_from_file(file)?;
    Ok(FileId(inner))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_id_with_same_path() -> Result<(), io::Error> {
        let file_id_one = dbg!(file_id("Cargo.toml")?);
        let file_id_two = dbg!(file_id("Cargo.toml")?);
        assert_eq!(file_id_one, file_id_two);
        Ok(())
    }

    #[test]
    fn test_file_id_with_different_paths() -> Result<(), io::Error> {
        let file_id_one = dbg!(file_id("Cargo.toml")?);
        let file_id_two = dbg!(file_id(".gitignore")?);
        assert_ne!(file_id_one, file_id_two);
        Ok(())
    }
}

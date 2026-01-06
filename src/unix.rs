use std::fs::{File, Metadata};
use std::io;
use std::os::unix::fs::MetadataExt;
use std::path::Path;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub(crate) struct FileId {
    device_id: u64,
    inode_number: u64,
}

pub(crate) fn file_id(path: &Path) -> Result<FileId, io::Error> {
    file_id_from_metadata(&path.metadata()?)
}

pub(crate) fn file_id_from_file(file: &File) -> Result<FileId, io::Error> {
    file_id_from_metadata(&file.metadata()?)
}

fn file_id_from_metadata(metadata: &Metadata) -> Result<FileId, io::Error> {
    Ok(FileId {
        device_id: metadata.dev(),
        inode_number: metadata.ino(),
    })
}

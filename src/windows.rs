use std::fs::File;
use std::io;
use std::os::windows::fs::OpenOptionsExt;
use std::os::windows::io::{AsHandle, AsRawHandle, BorrowedHandle};
use std::path::Path;

use windows_sys::Win32::Storage::FileSystem::{
    FILE_ID_128, FILE_ID_INFO, FileIdInfo, GetFileInformationByHandleEx,
};

pub trait FileIdExt {
    fn volume_serial_number(&self) -> u64;
    fn file_id(&self) -> u128;
}

impl FileIdExt for super::FileId {
    fn volume_serial_number(&self) -> u64 {
        self.0.volume_serial_number
    }

    fn file_id(&self) -> u128 {
        self.0.file_id
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub(crate) struct FileId {
    volume_serial_number: u64,
    file_id: u128,
}

pub(crate) fn file_id(path: &Path) -> Result<FileId, io::Error> {
    let file = File::options().access_mode(0).open(path)?;
    file_id_from_file(&file)
}

pub(crate) fn file_id_from_file(file: &File) -> Result<FileId, io::Error> {
    file_id_from_handle(file.as_handle())
}

fn file_id_from_handle(handle: BorrowedHandle) -> Result<FileId, io::Error> {
    // https://learn.microsoft.com/windows/win32/api/winbase/nf-winbase-getfileinformationbyhandleex

    let mut file_information = FILE_ID_INFO {
        VolumeSerialNumber: 0,
        FileId: FILE_ID_128 { Identifier: [0; _] },
    };
    let ret_val = unsafe {
        GetFileInformationByHandleEx(
            handle.as_raw_handle(),
            FileIdInfo,
            (&raw mut file_information).cast(),
            size_of::<FILE_ID_INFO>() as u32,
        )
    };
    if ret_val == 0 {
        return Err(io::Error::last_os_error());
    }

    Ok(FileId {
        volume_serial_number: file_information.VolumeSerialNumber,
        file_id: u128::from_ne_bytes(file_information.FileId.Identifier),
    })
}

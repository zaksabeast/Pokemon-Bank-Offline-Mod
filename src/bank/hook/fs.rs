use super::super::fs::OFFLINE_SAVE_PATH;
use super::stub::stub_success_result_code;
use crate::ctr::Handle;
use crate::ctr::fs::{Archive, ArchiveHandle, FsPath, fsuser};
use crate::ctr::res::CtrResult;
use crate::func_ptr;
use crate::hook::replace_fn;

const NORMAL_SAVE_PATH: &[u8] = b"/\0t\0u\0r\0t\0l\0e\0\0\0";

fn open_save_file() -> CtrResult<Handle> {
    let sd = Archive::sd()?;
    let file = sd.open_file(OFFLINE_SAVE_PATH)?;
    Ok(file.into_handle())
}

fn create_save_file() -> CtrResult<()> {
    let sd = Archive::sd()?;
    sd.create_file_with_attributes(OFFLINE_SAVE_PATH, 0, 0x200)?;
    let file = sd.open_file(OFFLINE_SAVE_PATH)?;
    let blank_save = [0u8; 0x200];
    file.write(&blank_save)?;
    Ok(())
}

pub extern "C" fn create_file(
    session: *const u32,
    _zero: u32,
    archive_handle: u64,
    path_type: u32,
    path_data: *const u8,
    path_size: u32,
    attributes: u32,
    file_size: u64,
) -> i32 {
    if session.is_null() {
        return -1;
    }

    let session = Handle(unsafe { *session });
    let archive = ArchiveHandle(archive_handle);
    let path = unsafe { FsPath::new_from_raw(path_data, path_type, path_size) };

    let is_save_file = &path.data() == &NORMAL_SAVE_PATH;
    let res = match is_save_file {
        true => create_save_file(),
        false => fsuser::create_file(Some(session), archive, &path, attributes, file_size),
    };

    match res {
        Ok(_) => 0,
        Err(code) => code,
    }
}

pub extern "C" fn open_file(
    session: *const u32,
    file_handle: *mut u32,
    _zero: u32,
    _size: u32,
    archive_handle: u64,
    path_type: u32,
    path_data: *const u8,
    path_size: u32,
    open_flags: u32,
    attributes: u32,
) -> i32 {
    if session.is_null() {
        return -1;
    }

    let session = Handle(unsafe { *session });
    let archive = ArchiveHandle(archive_handle);
    let path = unsafe { FsPath::new_from_raw(path_data, path_type, path_size) };

    let is_save_file = &path.data() == &NORMAL_SAVE_PATH;
    let res = match is_save_file {
        true => open_save_file(),
        false => fsuser::open_file(Some(session), archive, &path, open_flags, attributes),
    };

    match res {
        Ok(handle) => {
            unsafe { *file_handle = handle.0 };
            0
        }
        Err(code) => code,
    }
}

pub fn redirect_bank_save_data_to_sd() {
    replace_fn(0x165554, func_ptr!(stub_success_result_code)); // Delete file
    replace_fn(0x16569c, func_ptr!(stub_success_result_code)); // Format save data
    replace_fn(0x1654f8, func_ptr!(create_file)); // Create file
    replace_fn(0x1657ec, func_ptr!(open_file)); // Open file
}

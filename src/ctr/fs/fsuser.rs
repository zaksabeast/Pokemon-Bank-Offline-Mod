use crate::ctr::ipc::ipc_desc_cur_process_id;
use crate::ctr::srv::get_service_handle;

use super::super::ipc::{
    Handle, IpcBufferRights, get_thread_command_buffer, ipc_desc_buffer, ipc_make_header,
};
use super::super::res::{CtrResult, parse_res_u32};
use super::super::send_sync_request;
use super::common::{ArchiveHandle, FsArchiveId};
use super::path::FsPath;

static mut FSUSER_HANDLE: Handle = Handle(0);

pub fn fs_handle() -> Handle {
    unsafe { FSUSER_HANDLE }
}

fn initialize(session: Handle) -> CtrResult<()> {
    let cmd_buf = get_thread_command_buffer();
    cmd_buf[0] = ipc_make_header(0x801, 0, 2);
    cmd_buf[1] = ipc_desc_cur_process_id();

    send_sync_request(session)
}

pub fn init() -> CtrResult<()> {
    let session = get_service_handle("fs:USER")?;
    initialize(session)?;
    unsafe { FSUSER_HANDLE = session };
    Ok(())
}

pub fn open_archive(
    session: Option<Handle>,
    id: FsArchiveId,
    path: &FsPath,
) -> CtrResult<ArchiveHandle> {
    let cmd_buf = get_thread_command_buffer();
    cmd_buf[0] = ipc_make_header(0x80C, 3, 2);
    cmd_buf[1] = id as u32;
    cmd_buf[2] = path.path_type();
    cmd_buf[3] = path.len();
    cmd_buf[4] = path.buffer_desc();
    cmd_buf[5] = path.data_ptr();

    let session = session.unwrap_or_else(fs_handle);
    send_sync_request(session)?;
    parse_res_u32(cmd_buf[1])?;
    let archive_handle_low = cmd_buf[2];
    let archive_handle_high = cmd_buf[3];
    let handle = ArchiveHandle(((archive_handle_high as u64) << 32) | (archive_handle_low as u64));
    Ok(handle)
}

pub fn close_archive(archive: ArchiveHandle) -> CtrResult<()> {
    let cmd_buf = get_thread_command_buffer();
    cmd_buf[0] = ipc_make_header(0x80E, 2, 0);
    cmd_buf[1] = archive.low();
    cmd_buf[2] = archive.high();

    send_sync_request(fs_handle())?;
    parse_res_u32(cmd_buf[1])?;
    Ok(())
}

pub fn create_directory(archive: ArchiveHandle, path: &str, attributes: u32) -> CtrResult<()> {
    let path = FsPath::new(path);
    let cmd_buf = get_thread_command_buffer();
    cmd_buf[0] = ipc_make_header(0x809, 6, 2);
    cmd_buf[1] = 0;
    cmd_buf[2] = archive.low();
    cmd_buf[3] = archive.high();
    cmd_buf[4] = path.path_type();
    cmd_buf[5] = path.len();
    cmd_buf[6] = attributes;
    cmd_buf[7] = path.buffer_desc();
    cmd_buf[8] = path.data_ptr();

    send_sync_request(fs_handle())?;
    parse_res_u32(cmd_buf[1])?;
    Ok(())
}

pub fn create_file(
    session: Option<Handle>,
    archive: ArchiveHandle,
    path: &FsPath,
    attributes: u32,
    file_size: u64,
) -> CtrResult<()> {
    let cmd_buf = get_thread_command_buffer();
    cmd_buf[0] = ipc_make_header(0x808, 8, 2);
    cmd_buf[1] = 0;
    cmd_buf[2] = archive.low();
    cmd_buf[3] = archive.high();
    cmd_buf[4] = path.path_type();
    cmd_buf[5] = path.len();
    cmd_buf[6] = attributes;
    cmd_buf[7] = file_size as u32;
    cmd_buf[8] = (file_size >> 32) as u32;
    cmd_buf[9] = path.buffer_desc();
    cmd_buf[10] = path.data_ptr();

    let session = session.unwrap_or_else(fs_handle);
    send_sync_request(session)?;
    parse_res_u32(cmd_buf[1])?;
    Ok(())
}

pub fn open_file(
    session: Option<Handle>,
    archive: ArchiveHandle,
    path: &FsPath,
    open_flags: u32,
    attributes: u32,
) -> CtrResult<Handle> {
    let cmd_buf = get_thread_command_buffer();
    cmd_buf[0] = ipc_make_header(0x802, 7, 2);
    cmd_buf[1] = 0;
    cmd_buf[2] = archive.low();
    cmd_buf[3] = archive.high();
    cmd_buf[4] = path.path_type();
    cmd_buf[5] = path.len();
    cmd_buf[6] = open_flags;
    cmd_buf[7] = attributes;
    cmd_buf[8] = path.buffer_desc();
    cmd_buf[9] = path.data_ptr();

    let session = session.unwrap_or_else(fs_handle);
    send_sync_request(session)?;
    parse_res_u32(cmd_buf[1])?;
    let file_handle = Handle(cmd_buf[3]);
    Ok(file_handle)
}

pub fn close_file(file: Handle) -> CtrResult<()> {
    let cmd_buf = get_thread_command_buffer();
    cmd_buf[0] = ipc_make_header(0x808, 0, 0);

    send_sync_request(file)?;
    parse_res_u32(cmd_buf[1])?;
    Ok(())
}

pub fn write_file(file: Handle, offset: u64, buffer: &[u8], flags: u32) -> CtrResult<u32> {
    let cmd_buf = get_thread_command_buffer();
    cmd_buf[0] = ipc_make_header(0x803, 4, 2);
    cmd_buf[1] = offset as u32;
    cmd_buf[2] = (offset >> 32) as u32;
    cmd_buf[3] = buffer.len() as u32;
    cmd_buf[4] = flags;
    cmd_buf[5] = ipc_desc_buffer(buffer.len() as u32, IpcBufferRights::Read);
    cmd_buf[6] = buffer.as_ptr() as u32;

    send_sync_request(file)?;
    parse_res_u32(cmd_buf[1])?;
    Ok(cmd_buf[2])
}

pub fn read_file(file: Handle, offset: u64, buffer: &mut [u8]) -> CtrResult<usize> {
    let cmd_buf = get_thread_command_buffer();
    cmd_buf[0] = ipc_make_header(0x802, 3, 2);
    cmd_buf[1] = offset as u32;
    cmd_buf[2] = (offset >> 32) as u32;
    cmd_buf[3] = buffer.len() as u32;
    cmd_buf[4] = ipc_desc_buffer(buffer.len() as u32, IpcBufferRights::Write);
    cmd_buf[5] = buffer.as_mut_ptr() as u32;

    send_sync_request(file)?;
    parse_res_u32(cmd_buf[1])?;
    Ok(cmd_buf[2] as usize)
}

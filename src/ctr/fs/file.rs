use core::mem;

use super::super::ipc::Handle;
use super::super::res::CtrResult;
use super::common::{ArchiveHandle, FsOpenFlags, FsWriteFlags};
use super::fsuser;
use super::path::FsPath;

pub struct File {
    handle: Handle,
}

impl File {
    pub(super) fn open(
        archive: ArchiveHandle,
        path: &str,
        open_flags: FsOpenFlags,
        attributes: u32,
    ) -> CtrResult<Self> {
        let path = FsPath::new(path);
        let handle = fsuser::open_file(None, archive, &path, open_flags as u32, attributes)?;
        Ok(Self { handle })
    }

    fn close(&mut self) -> CtrResult<()> {
        fsuser::close_file(self.handle)
    }

    fn partial_write(&self, offset: u64, buffer: &[u8], flags: FsWriteFlags) -> CtrResult<u32> {
        fsuser::write_file(self.handle, offset, buffer, flags as u32)
    }

    fn partial_read(&self, offset: u64, buffer: &mut [u8]) -> CtrResult<usize> {
        fsuser::read_file(self.handle, offset, buffer)
    }

    pub fn write_offset(&self, offset: u64, buffer: &[u8], flags: FsWriteFlags) -> CtrResult<u32> {
        let mut offset = offset;
        let mut remaining = buffer;

        while !remaining.is_empty() {
            let bytes_written = self.partial_write(offset, remaining, flags)?;
            if bytes_written == 0 {
                return Err(-1); // Write failed
            }
            offset += bytes_written as u64;
            remaining = &remaining[bytes_written as usize..];
        }

        Ok(buffer.len() as u32)
    }

    pub fn write(&self, buffer: &[u8]) -> CtrResult<u32> {
        self.write_offset(0, buffer, FsWriteFlags::Flush)
    }

    pub fn read_offset(&self, offset: u64, buffer: &mut [u8]) -> CtrResult<usize> {
        let mut offset = offset;
        let mut total_bytes_read: usize = 0;
        let mut remaining = buffer;

        while !remaining.is_empty() {
            let bytes_read = self.partial_read(offset, remaining)?;
            total_bytes_read += bytes_read;

            if bytes_read == 0 {
                return Ok(total_bytes_read);
            }
            offset += bytes_read as u64;
            remaining = &mut remaining[bytes_read..];
        }

        Ok(total_bytes_read)
    }

    pub fn read(&self, buffer: &mut [u8]) -> CtrResult<usize> {
        self.read_offset(0, buffer)
    }

    pub fn into_handle(self) -> Handle {
        let handle = self.handle;
        mem::forget(self);
        handle
    }
}

impl Drop for File {
    fn drop(&mut self) {
        let _ = self.close();
    }
}

use crate::ctr::fs::FsPath;

use super::super::res::CtrResult;
use super::common::{ArchiveHandle, FsArchiveId, FsOpenFlags};
use super::{File, fsuser};

pub struct Archive {
    handle: ArchiveHandle,
}

impl Archive {
    pub fn open(id: FsArchiveId, path: &str) -> CtrResult<Self> {
        let path = FsPath::new(path);
        let handle = fsuser::open_archive(None, id, &path)?;
        Ok(Self { handle })
    }

    pub fn sd() -> CtrResult<Self> {
        Self::open(FsArchiveId::Sdmc, "")
    }

    pub fn create_directory_with_attributes(&self, path: &str, attributes: u32) -> CtrResult<()> {
        fsuser::create_directory(self.handle, path, attributes)
    }

    pub fn create_dir(&self, path: &str) -> CtrResult<()> {
        self.create_directory_with_attributes(path, 0)
    }

    pub fn create_file_with_attributes(
        &self,
        path: &str,
        attributes: u32,
        file_size: u64,
    ) -> CtrResult<()> {
        let path = FsPath::new(path);
        fsuser::create_file(None, self.handle, &path, attributes, file_size)
    }

    pub fn create_file(&self, path: &str) -> CtrResult<()> {
        self.create_file_with_attributes(path, 0, 0)
    }

    pub fn open_file_with_attributes(
        &self,
        path: &str,
        open_flags: FsOpenFlags,
        attributes: u32,
    ) -> CtrResult<File> {
        File::open(self.handle, path, open_flags, attributes)
    }

    pub fn open_file(&self, path: &str) -> CtrResult<File> {
        self.open_file_with_attributes(path, FsOpenFlags::ReadWrite, 0)
    }

    pub fn file_exists(&self, path: &str) -> bool {
        self.open_file(path).is_ok()
    }
}

impl Drop for Archive {
    fn drop(&mut self) {
        let _ = fsuser::close_archive(self.handle);
    }
}

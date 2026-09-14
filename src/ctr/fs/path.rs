use core::slice::from_raw_parts;

use super::super::ipc::ipc_desc_static_buffer;
use alloc::vec::Vec;
use alloc::{format, vec};
use num_enum::FromPrimitive;

#[derive(Default, Clone, Copy, PartialEq, Eq, FromPrimitive)]
#[repr(u32)]
pub enum FsPathType {
    #[default]
    Invalid = 0x0,
    Empty = 0x1,
    Binary = 0x2,
    Ascii = 0x3,
    Utf16 = 0x4,
}

#[repr(C)]
pub struct FsPath {
    path_type: FsPathType,
    path: Vec<u8>,
}

impl FsPath {
    pub fn empty() -> Self {
        Self {
            path_type: FsPathType::Empty,
            path: vec![0x00],
        }
    }

    pub fn new(path: &str) -> Self {
        if path.is_empty() {
            return Self::empty();
        }

        Self {
            path_type: FsPathType::Ascii,
            path: format!("{}\0", path).into_bytes().to_vec(),
        }
    }

    pub unsafe fn new_from_raw(path: *const u8, path_type: u32, len: u32) -> Self {
        Self {
            path_type: path_type.into(),
            path: from_raw_parts(path, len as usize).to_vec(),
        }
    }

    pub fn data(&self) -> &[u8] {
        &self.path
    }

    pub fn path_type(&self) -> u32 {
        self.path_type as u32
    }

    pub fn len(&self) -> u32 {
        self.path.len() as u32
    }

    pub fn buffer_desc(&self) -> u32 {
        ipc_desc_static_buffer(self.len(), 0)
    }

    pub fn data_ptr(&self) -> u32 {
        self.path.as_ptr() as u32
    }
}

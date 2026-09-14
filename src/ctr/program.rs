use crate::ctr::get_current_process_id;
use crate::ctr::res::{CtrResult, parse_res};

extern "C" {
    fn fsInit() -> i32;
    fn fsExit();
    fn FSUSER_GetProgramLaunchInfo(info: *mut FS_ProgramInfo, process_id: u32) -> i32;
    fn FSUSER_GetProductInfo(info: *mut FS_ProductInfo, process_id: u32) -> i32;
}

#[derive(Default)]
#[repr(C, packed)]
struct FS_ProgramInfo {
    title_id: u64,
    media_type: u8,
    padding: [u8; 7],
}

#[derive(Default)]
#[repr(C, packed)]
struct FS_ProductInfo {
    produce_code: [u8; 0x10],
    company_code: [u8; 0x2],
    revision_version: u16,
}

pub struct ProgramInfo {
    pub title_id: u64,
    pub revision_version: u16,
}

pub fn get_program_info() -> CtrResult<ProgramInfo> {
    unsafe {
        fsInit();
        let process_id = get_current_process_id()?;
        let mut program_info = FS_ProgramInfo::default();

        let res = FSUSER_GetProgramLaunchInfo(&mut program_info as *mut FS_ProgramInfo, process_id);
        parse_res(res)?;

        let mut product_info = FS_ProductInfo::default();
        let res = FSUSER_GetProductInfo(&mut product_info as *mut FS_ProductInfo, process_id);
        parse_res(res)?;

        fsExit();

        Ok(ProgramInfo {
            title_id: program_info.title_id,
            revision_version: product_info.revision_version,
        })
    }
}

use crate::ctr::Handle;
use crate::ctr::res::{CtrResult, parse_res};
use alloc::ffi::CString;

extern "C" {
    fn srvInit() -> i32;
    fn srvGetServiceHandle(out: *mut u32, name: *const u8) -> i32;
}

pub fn init() -> CtrResult<()> {
    let res = unsafe { srvInit() };
    parse_res(res)
}

pub fn get_service_handle(name: &str) -> CtrResult<Handle> {
    let mut out = Handle(0);
    let name = CString::new(name).map_err(|_| -1)?;
    let res = unsafe { srvGetServiceHandle(&mut out.0 as *mut u32, name.as_ptr()) };
    parse_res(res)?;
    Ok(out)
}

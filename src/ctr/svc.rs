use crate::ctr::ipc::Handle;
use crate::ctr::res::{CtrResult, parse_res};
use alloc::ffi::CString;

#[derive(Default)]
#[repr(C, packed)]
pub struct MemInfo {
    pub base_addr: u32,
    pub size: u32,
    pub perm: u32,
    pub state: u32,
}

#[derive(Default)]
#[repr(C, packed)]
pub struct PageInfo {
    pub flags: u32,
}

#[repr(u32)]
pub enum ArbitrationType {
    Signal = 0,
    WaitIfLessThan = 1,
    // DecrementAndWaitIfLessThan = 2,
    // WaitIfLessThanTimeout = 3,
    // DecrementAndWaitIfLessThanTimeout = 4,
}

extern "C" {
    fn svcGetProcessId(out: *mut u32, handle: u32) -> i32;
    fn svcSleepThread(nanos: i64);
    fn svcSendSyncRequest(session: u32) -> i32;
    fn svcFlushProcessDataCache(process: u32, addr: u32, size: u32) -> i32;
    fn svcMapMemoryBlock(memblock: u32, addr: u32, my_perm: u32, other_perm: u32) -> i32;
    fn svcQueryMemory(mem: *mut MemInfo, page: *mut PageInfo, addr: u32) -> i32;
    fn svcInvalidateEntireInstructionCache();
    fn svcGetSystemInfo(out: *mut i64, info_type: u32, param: i32) -> i32;
    fn svcArbitrateAddress(
        arbiter: u32,
        addr: u32,
        arbitration_type: u32,
        value: i32,
        timeout_ns: i64,
    ) -> i32;
    fn svcConnectToPort(out: *mut u32, port_name: *const u8) -> i32;
    fn svcExitThread() -> !;
    fn svcCreateThread(
        out: *mut u32,
        entrypoint: extern "C" fn(),
        arg: u32,
        stack_top: *mut u8,
        thread_priority: i32,
        processor_id: i32,
    ) -> i32;
}

pub fn get_current_process_id() -> CtrResult<u32> {
    let mut out: u32 = 0;
    let res = unsafe { svcGetProcessId(&mut out as *mut u32, Handle::CURRENT_PROCESS.0) };
    parse_res(res)?;
    Ok(out)
}

pub fn sleep_thread(nanos: i64) {
    unsafe { svcSleepThread(nanos) }
}

pub fn send_sync_request(handle: Handle) -> CtrResult<()> {
    let res = unsafe { svcSendSyncRequest(handle.0) };
    parse_res(res)
}

pub fn flush_current_process_data_cache(addr: u32, size: u32) -> CtrResult<()> {
    let res = unsafe { svcFlushProcessDataCache(Handle::CURRENT_PROCESS.0, addr, size) };
    parse_res(res)
}

pub fn map_memory_block(
    memblock: Handle,
    addr: u32,
    my_perm: u32,
    other_perm: u32,
) -> CtrResult<()> {
    let res = unsafe { svcMapMemoryBlock(memblock.0, addr, my_perm, other_perm) };
    parse_res(res)
}

pub fn query_memory(addr: u32) -> CtrResult<(MemInfo, PageInfo)> {
    let mut mem = MemInfo::default();
    let mut page = PageInfo::default();
    let res = unsafe { svcQueryMemory(&mut mem as *mut MemInfo, &mut page as *mut PageInfo, addr) };
    parse_res(res)?;
    Ok((mem, page))
}

pub fn invalidate_entire_instruction_cache() {
    unsafe { svcInvalidateEntireInstructionCache() }
}

pub fn is_citra() -> bool {
    let mut out = 0_i64;
    unsafe { svcGetSystemInfo(&mut out as *mut i64, 0x20000, 0) };
    out != 0
}

pub fn connect_to_port(port_name: &str) -> CtrResult<Handle> {
    let mut out = Handle(0);
    let port_name = CString::new(port_name).map_err(|_| -1)?;
    let res = unsafe { svcConnectToPort(&mut out.0 as *mut u32, port_name.as_ptr()) };
    parse_res(res)?;
    Ok(out)
}

pub(super) fn create_thread(
    entrypoint: extern "C" fn(),
    stack_top: *mut u8,
    thread_priority: i32,
    processor_id: i32,
) -> CtrResult<Handle> {
    let mut out = Handle(0);
    let res = unsafe {
        svcCreateThread(
            &mut out.0 as *mut u32,
            entrypoint,
            0x00,
            stack_top,
            thread_priority,
            processor_id,
        )
    };
    parse_res(res)?;
    Ok(out)
}

pub(super) fn exit_thread() -> ! {
    unsafe { svcExitThread() }
}

pub fn arbitrate_address(
    arbiter: Handle,
    addr: u32,
    arbitration_type: ArbitrationType,
    value: i32,
    timeout_ns: i64,
) -> CtrResult<()> {
    let res =
        unsafe { svcArbitrateAddress(arbiter.0, addr, arbitration_type as u32, value, timeout_ns) };
    parse_res(res)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Handle(pub u32);

impl Handle {
    pub const CURRENT_PROCESS: Handle = Handle(0xFFFF8001);
}

#[repr(C)]
struct ThreadLocalStorage {
    storage: [u32; 32],
    cmd_buf: [u32; 64],
    static_bufs: [u32; 32],
}

fn get_thread_local_storage() -> *mut ThreadLocalStorage {
    let mut ret: *mut u32 = core::ptr::null_mut();
    unsafe {
        core::arch::asm!("mrc p15, 0, {data}, c13, c0, 3", data = out(reg) ret);
    }
    ret as *mut ThreadLocalStorage
}

pub fn get_thread_command_buffer() -> &'static mut [u32; 64] {
    let tls = get_thread_local_storage();
    unsafe { &mut (*tls).cmd_buf }
}

pub const fn ipc_desc_static_buffer(size: u32, buffer_id: u32) -> u32 {
    (size << 14) | ((buffer_id & 0xF) << 10) | 0x2
}

#[repr(u32)]
pub enum IpcBufferRights {
    Read = 2,
    Write = 4,
}

pub const fn ipc_desc_buffer(size: u32, rights: IpcBufferRights) -> u32 {
    (size << 4) | 0x8 | (rights as u32)
}

pub const fn ipc_make_header(cmd_id: u16, normal_params: u32, translate_params: u32) -> u32 {
    ((cmd_id as u32) << 16) | ((normal_params & 0x3f) << 6) | (translate_params & 0x3f)
}

pub const fn ipc_desc_cur_process_id() -> u32 {
    0x20
}

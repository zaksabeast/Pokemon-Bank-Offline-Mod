#![no_std]
#![allow(static_mut_refs)]
#![feature(trivial_bounds)]

extern crate alloc;

#[cfg(target_os = "horizon")]
mod allocator {
    use libc_alloc::LibcAlloc;

    #[global_allocator]
    static ALLOCATOR: LibcAlloc = LibcAlloc;
}

mod app;
mod bank;
mod ctr;
mod hook;
mod ui;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const GIT_HASH: &str = env!("GIT_HASH");

use ctr::{fs, invalidate_entire_instruction_cache, plgldr, srv};

#[cfg(target_os = "horizon")]
#[panic_handler]
fn my_panic(info: &core::panic::PanicInfo) -> ! {
    if let Some(location) = info.location() {
        let file = location.file();
        let slice = &file[file.len() - 7..];

        // Since we're about to break, storing a few u32s in these registers won't break us further.
        // In the future it might be helpful to disable this for release builds.
        unsafe {
            // r9 and r10 aren't used as frequently as the lower registers, so in most situations
            // we'll get more useful information by storing the last 4 characters of the file name
            // and the line number where we broke.
            let partial_file_name = *(slice.as_ptr() as *const u32);
            core::arch::asm!("mov r9, {}", in(reg) partial_file_name);
            core::arch::asm!("mov r10, {}", in(reg) location.line());
        }
    }

    // svcBreak(USERBREAK_PANIC)
    unsafe { core::arch::asm!("svc 0x3C", in("r0") 0u32) };
    loop {}
}

unsafe extern "C" {
    static mut fake_heap_start: *mut u8;
    static mut fake_heap_end: *mut u8;

    static mut __ctru_heap: u32;
    static mut __ctru_linear_heap: u32;
}

#[no_mangle]
pub static mut __ctru_heap_size: u32 = 0;

#[no_mangle]
pub static mut __ctru_linear_heap_size: u32 = 0;

unsafe fn system_allocate_heaps() {
    let header = plgldr::get_header();
    __ctru_heap_size = header.heap_size;
    __ctru_heap = header.heap_va;

    // Set up newlib heap.
    fake_heap_start = __ctru_heap as *mut u8;
    fake_heap_end = fake_heap_start.add(__ctru_heap_size as usize);
}

/// Entrypoint, game will starts when you exit this function
#[no_mangle]
pub extern "C" fn main() {
    unsafe { system_allocate_heaps() };

    let _ = srv::init();
    let _ = fs::fsuser::init();

    if let Ok(_) = plgldr::init() {
        plgldr::start_ack_events_thread();
    }

    app::init();

    invalidate_entire_instruction_cache();
}

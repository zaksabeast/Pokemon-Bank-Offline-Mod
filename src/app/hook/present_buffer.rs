use crate::app::frame::run_frame;
use crate::ctr::hid::scan_input;
use crate::ctr::{draw_to_screen, flush_current_process_data_cache};
use core::mem::transmute;

const SCREEN_WIDTH: u32 = 400;
const SCREEN_HEIGHT: u32 = 240;

pub extern "C" fn present_buffer(
    screen_id: u32,
    swap: u32,
    fb_a: *mut u8,
    fb_b: *mut u8,
    stride: u32,
    format: u32,
    unk: u32,
) -> bool {
    let is_top_screen = screen_id == 0;
    if is_top_screen {
        scan_input();
        run_frame();
        draw_to_screen(screen_id, fb_a, stride, format);
    }

    let _ = flush_current_process_data_cache(fb_a as u32, SCREEN_WIDTH * SCREEN_HEIGHT);
    // Thanks to https://github.com/44670/NTR/blob/c764c0f68c08f3518a9f284f5fda1bf3b2636123/source/plg.c#L868-L870
    if is_top_screen && fb_a != fb_b && !fb_b.is_null() {
        let _ = flush_current_process_data_cache(fb_b as u32, SCREEN_WIDTH * SCREEN_HEIGHT);
    }

    let gsp_present_buffer: extern "C" fn(u32, u32, *mut u8, *mut u8, u32, u32, u32) -> bool =
        unsafe { transmute(0x12d488) };
    gsp_present_buffer(screen_id, swap, fb_a, fb_b, stride, format, unk)
}

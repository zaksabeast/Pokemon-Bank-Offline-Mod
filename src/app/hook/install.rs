use super::hid::map_input_hook;
use super::present_buffer::present_buffer;
use crate::ctr::res::CtrResult;
use crate::ctr::{mem, query_memory};
use crate::func_ptr;
use crate::hook::{branch_target, replace_fn};
use core::slice::from_raw_parts;

fn find_in_code(base_addr: u32, code: &[u8], bytes: &[u8]) -> CtrResult<u32> {
    let pos = code
        .windows(bytes.len())
        .position(|window| window == bytes)
        .ok_or(-1)?;

    Ok(base_addr + pos as u32)
}

fn hook_hid(base_addr: u32, code: &[u8]) -> CtrResult<()> {
    let map_input_block: &[u8] = &[
        0x01, 0x20, 0xa0, 0x13, 0x03, 0x20, 0xa0, 0x03, 0x01, 0x32, 0xa0, 0xe3, 0x1f, 0x00, 0x00,
        0xef, 0xa0, 0x1f, 0xb0, 0xe1, 0x01, 0x10, 0xa0, 0x03, 0x18, 0x10, 0xc4, 0x05,
    ];

    let addr = find_in_code(base_addr, code, map_input_block)?;

    mem::write(addr, &0xe51fe000_u32); //     ldr        lr,[pc + 0x8]
    mem::write(addr + 0x4, &0xe51ff000_u32); //     ldr        pc,[pc + 0x8]
    // 4 instructions * 4 bytes per instruction
    mem::write(addr + 0x8, &(addr + (4 * 4))); //     ldr        pc,[pc + 0x8]
    mem::write(addr + 0xc, &func_ptr!(map_input_hook)); //     ldr        pc,[pc + 0x8] 

    Ok(())
}

fn hook_present_framebuffer(base_addr: u32, code: &[u8]) -> CtrResult<()> {
    let present_buffer_caller_bytes: &[u8] = &[
        0x10, 0xc0, 0x90, 0xe5, 0x0c, 0xe0, 0x90, 0xe5, 0x04, 0x00, 0x90, 0xe5, 0x03, 0x30, 0x4c,
        0xe0, 0x0a, 0x30, 0x43, 0xe0, 0x9e, 0xb3, 0x23, 0xe0, 0x91, 0x03, 0x23, 0xe0, 0x4c, 0x11,
        0x95, 0xe5, 0x04, 0x00, 0xa0, 0xe1, 0xf0, 0x80, 0xcd, 0xe1, 0x08, 0x10, 0x8d, 0xe5,
    ];

    let addr = find_in_code(base_addr, code, present_buffer_caller_bytes)?;

    let branch_addr = addr + (11 * 4);
    let branch_inst = mem::read(branch_addr);
    let present_buffer_addr = branch_target(branch_addr, branch_inst);

    replace_fn(present_buffer_addr, func_ptr!(present_buffer));

    Ok(())
}

pub fn install_hooks() -> CtrResult<()> {
    let mut base_addr = 0x100000;
    let (mem, _page) = query_memory(base_addr)?;

    base_addr = mem.base_addr;
    let code = unsafe { from_raw_parts(mem.base_addr as *const u8, mem.size as usize) };

    hook_present_framebuffer(base_addr, code)?;
    hook_hid(base_addr, code)?;

    Ok(())
}

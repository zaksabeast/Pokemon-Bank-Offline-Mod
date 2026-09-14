pub fn branch_target(pc: u32, instruction: u32) -> u32 {
    let imm24 = instruction & 0x00FF_FFFF;
    let offset = ((imm24 << 8) as i32 >> 6) as u32;

    pc.wrapping_add(8).wrapping_add(offset)
}

use crate::ctr::mem;

pub fn replace_fn(original: u32, replacement: u32) {
    mem::write(original, &0xe51ff004_u32); // ldr pc,[pc,#-0x4]
    mem::write(original + 4, &replacement);
}

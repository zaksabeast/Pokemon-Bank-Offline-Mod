use core::ptr;

pub fn read_array<const SIZE: usize>(addr: u32) -> [u8; SIZE] {
    let addr = addr as *const [u8; SIZE];
    unsafe { ptr::read(addr) }
}

pub fn read<T: Copy>(addr: u32) -> T {
    unsafe { ptr::read_unaligned(addr as *const T) }
}

pub fn write<T: Copy>(addr: u32, buf: &T) {
    unsafe {
        ptr::write_unaligned(addr as *mut T, *buf);
    }
}

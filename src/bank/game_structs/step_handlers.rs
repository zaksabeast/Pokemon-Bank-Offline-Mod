use core::ffi::c_void;

#[repr(C)]
pub struct StateContainerVtable {
    // All of these have actual args and
    pub vtable_reset: extern "C" fn() -> *const c_void,
    pub destructor: extern "C" fn(),
    pub substep_0: extern "C" fn() -> bool,
    pub substep_1: extern "C" fn() -> bool,
    pub unk: extern "C" fn(),
    pub substep_2: extern "C" fn() -> bool,
    pub attach_stuff_to_state: extern "C" fn(),
    pub do_save_stuff: extern "C" fn() -> u32, // Waiting = 0, Error = 1, Success = 2,
    // This one handles a variety of tasks, including the user action in the logged in menu.
    // However, it's not strictly a menu handler. Most menu handling happens elsewhere, and
    // this is usually a null pointer.
    pub unk2: extern "C" fn(),
}

const _: () = assert!(core::mem::size_of::<StateContainerVtable>() == 0x24);

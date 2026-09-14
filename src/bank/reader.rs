use crate::bank::game_structs::AppState;
use crate::ctr::mem;
use core::mem::size_of;
use core::slice::from_raw_parts_mut;
use zerocopy::FromBytes;

struct BankAddresses {
    app_state: u32,
    main_singleton: u32,
}

const BANK_ADDRESSES: BankAddresses = BankAddresses {
    app_state: 0x8060550,
    main_singleton: 0x3ab90c,
};

pub struct BankReader {
    addrs: &'static BankAddresses,
}

impl BankReader {
    pub fn new() -> Self {
        Self {
            addrs: &BANK_ADDRESSES,
        }
    }

    pub fn app_state(&self) -> &mut AppState {
        let slice =
            unsafe { from_raw_parts_mut(self.addrs.app_state as *mut u8, size_of::<AppState>()) };
        AppState::mut_from_bytes(slice).unwrap()
    }

    pub fn bank_data_ptr(&self) -> *mut u8 {
        let main_singleton = mem::read::<u32>(self.addrs.main_singleton);
        let container = mem::read::<u32>(main_singleton + 0x1c);
        let manager = mem::read::<u32>(container + 0xcc);
        (manager + 8) as *mut u8
    }
}

use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::bank::game_structs::Bool;

#[derive(FromBytes, IntoBytes, Immutable, KnownLayout)]
#[repr(C, packed)]
pub struct SalesData {
    pub data: [u8; 0x20],
    pub some_enum: u8,
    pub data2: [u8; 7],
    // This is a decimal timestamp.
    // If you view the number in a dec representation, it's a human readable date/time.
    pub current_time: u64,
    pub data3: [u32; 3],
    pub days_diff: i32,
    pub hours_diff: i32,
    pub has_premium_features: Bool,
    pub data4: [u8; 0xa],
    pub is_trial_mode: Bool,
    // These are in a bit format.
    pub premium_start_ts: u64,
    pub premium_end_ts: u64,
}

const _: () = assert!(core::mem::size_of::<SalesData>() == 0x60);

use zerocopy::IntoBytes;

use super::super::game_structs::SalesData;
use crate::bank::game_structs::Bool;
use crate::ctr::mem;
use core::mem::size_of;
use core::slice::from_raw_parts_mut;

// Uses days and months that start with 1, not 0
fn build_bit_ts(year: u64, month: u64, day: u64, hour: u64, minute: u64, second: u64) -> u64 {
    (year << 26) | (month << 22) | (day << 17) | (hour << 12) | (minute << 6) | second
}

fn build_decimal_ts(year: u64, month: u64, day: u64, hour: u64, minute: u64, second: u64) -> u64 {
    (year * 10000000000)
        | (month * 100000000)
        | (day * 1000000)
        | (hour * 10000)
        | (minute * 100)
        | second
}

pub extern "C" fn parse_sales_data() -> bool {
    let step_handler = mem::read::<u32>(0x8060550 + 0x10);
    // result_code
    mem::write::<u8>(step_handler + 0x30, &0x4);

    let sales_data_ptr = mem::read::<u32>(step_handler + 0x28) as *mut u8;
    let sales_data_buf = unsafe { from_raw_parts_mut(sales_data_ptr, size_of::<SalesData>()) };

    let data = SalesData {
        // This is the only thing that matters
        // everything else is either for display or derives other data
        has_premium_features: Bool::new_true(),
        some_enum: 0x33,

        // Visual display
        days_diff: 400,
        hours_diff: 0,
        is_trial_mode: Bool::new_false(),

        // Used to derive displays
        // Meaningless to set here since the visual displays are set above,
        // but I'm setting for posterity
        premium_end_ts: build_bit_ts(2050, 12, 31, 23, 59, 59),
        premium_start_ts: build_bit_ts(2023, 2, 14, 00, 00, 00),
        current_time: build_decimal_ts(2026, 1, 1, 1, 1, 1),

        // Stuff I didn't want to label
        data: [0; 0x20],
        data2: [0; 7],
        data3: [0; 3],
        data4: [0; 0xa],
    };

    sales_data_buf.clone_from_slice(data.as_bytes());

    true
}

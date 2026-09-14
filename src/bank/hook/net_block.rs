use crate::ctr::sleep_thread;
use crate::func_ptr;
use crate::hook::replace_fn;

const FRD_HTTP_SOC_FNS: &[u32] = &[
    0x2458a8, 0x2458d8, 0x2456dc, 0x245640, 0x2459fc, 0x2459cc, 0x245a74, 0x245748, 0x23880c,
    0x23807c, 0x24a43c, 0x24a49c, 0x24a320, 0x24a688, 0x24a29c, 0x24a5b8, 0x24a3c4, 0x24a63c,
    0x24b12c, 0x24c020, 0x2077b8, 0x1243e0, 0x24c070, 0x24c144, 0x124420,
];

extern "C" fn block() -> ! {
    // todo: capture pc and display for debug info
    loop {
        sleep_thread(500_000_000)
    }
}

pub fn prevent_network_activity() {
    FRD_HTTP_SOC_FNS
        .iter()
        .for_each(|&addr| replace_fn(addr, func_ptr!(block)));
}

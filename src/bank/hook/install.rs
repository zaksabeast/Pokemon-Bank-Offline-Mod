use super::super::game_structs::StateContainerVtable;
use super::bank_server::{download_bank_data, save_bank_to_server};
use super::net_block::prevent_network_activity;
use super::sales_data::parse_sales_data;
use super::state_loop::get_next_step;
use super::stub::stub_true;
use crate::bank::hook::fs::redirect_bank_save_data_to_sd;
use crate::ctr::mem;
use crate::func_ptr;
use crate::hook::replace_fn;

pub fn install_offline_hooks() {
    prevent_network_activity();

    // Hook saving bank to server
    replace_fn(0x2b2320, func_ptr!(save_bank_to_server));

    // Hook get_next_step
    replace_fn(0x2a5580, func_ptr!(get_next_step));

    // has_time_elapsed always return true for instant loaders
    replace_fn(0x1d5bb0, func_ptr!(stub_true));

    let parse_sales_data_vtable = unsafe { &mut *(0x361fa4 as *mut StateContainerVtable) };
    parse_sales_data_vtable.substep_0 = stub_true;
    parse_sales_data_vtable.substep_1 = parse_sales_data;
    parse_sales_data_vtable.substep_2 = stub_true;

    let download_bank_data_vtable = unsafe { &mut *(0x361ec8 as *mut StateContainerVtable) };
    download_bank_data_vtable.substep_0 = stub_true;
    download_bank_data_vtable.substep_1 = download_bank_data;
    download_bank_data_vtable.substep_2 = stub_true;

    // Fixes saving bank
    replace_fn(0x1d5d74, func_ptr!(stub_true));
    replace_fn(0x2b24b4, func_ptr!(stub_true));
    mem::write(0x2b1e0c, &0xe320f000_u32); // nop out resetting success bool

    // Stub net handlers
    replace_fn(0x1df3e0, func_ptr!(stub_true));
    replace_fn(0x1d5c28, func_ptr!(stub_true));
    replace_fn(0x22e3e0, func_ptr!(stub_true));

    mem::write(0x2a95b4, &0xe3a00005_u32); // mov        param_1,#0x5
    mem::write(0x2a95b8, &0xe320f000_u32); // nop

    // Stub "Move To Home" and "Download Transporter" buttons
    mem::write(0x2a6d08, &0x2a6d88_u32);
    mem::write(0x2a6d00, &0x2a6d88_u32);

    redirect_bank_save_data_to_sd();
}

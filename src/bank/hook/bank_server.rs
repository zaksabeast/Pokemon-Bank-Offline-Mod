use super::super::bank_file::BankFile;
use super::super::game_structs::StepState;

#[repr(C, packed)]
pub struct SaveBoxesToServerAndQuitState {
    super_struct: StepState,
    data: [u8; 0x14],
    saved_successfully: bool,
    had_error: bool,
    data2: [u8; 6],
}

const _: () = assert!(core::mem::size_of::<SaveBoxesToServerAndQuitState>() == 0x50);

pub extern "C" fn save_bank_to_server(state: *mut SaveBoxesToServerAndQuitState) -> bool {
    // Todo: Make sure the game is saved before doing this
    let _ = BankFile::write_to_sd();
    let state = unsafe { &mut *state };

    state.saved_successfully = true;
    state.had_error = false;

    true
}

pub extern "C" fn download_bank_data() -> bool {
    let _ = BankFile::read_from_sd();
    true
}

use super::Bool;
use core::mem::size_of;
use core::slice::from_raw_parts_mut;
use num_enum::FromPrimitive;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

/// Every step's state container inherits from this
#[derive(FromBytes, IntoBytes, Immutable, KnownLayout)]
#[repr(C, packed)]
pub struct StepState {
    pub vtable: u32,
    // various data and memory managers
    pub data: [u8; 0x2c],
    pub result_code: u8,
    pub padding: [u8; 3],
}

const _: () = assert!(core::mem::size_of::<StepState>() == 0x34);

#[repr(u8)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, FromPrimitive)]
pub enum AppStateStep {
    #[default]
    None = 0xff,
    GameStart = 0x00,
    SelectLanguage = 0x01,
    TitleScreen = 0x02,
    CheckingGameCart = 0x03,
    LoggedIn = 0x04,
    ConnectingToInternet = 0x05,
    BankIsOutdated = 0x06,
    SaveBoxesToServerAndQuit = 0x07,
    ConnectedToInternet = 0x08,
    Case0x09 = 0x09,
    SelectGame = 0x0a,
    SelectedGame = 0x0b,
    PokeMileCheck = 0x0c,
    GetPokeMiles = 0x0d,
    Case0x0e = 0x0e,
    ParseSalesData = 0x0f,
    DownloadBankData = 0x10,
    SelectGame1 = 0x11,
    SelectGame2 = 0x12,
    QuitWithoutSaving = 0x13,
    Disconnect = 0x14,
    LoadToTitleScreen = 0x15,
    SaveBoxesToServer = 0x16,
    Case0x17 = 0x17,
    Case0x18 = 0x18,
    ViewBankBoxes = 0x19,
    Case0x1a = 0x1a,
    Case0x1b = 0x1b,
    MovePokemonToHome = 0x1c,
    Case0x1d = 0x1d,
}

#[derive(FromBytes, IntoBytes, Immutable, KnownLayout)]
#[repr(C, packed)]
pub struct AppState {
    pub vtable: u32,
    pub mem_manager: u32,
    pub data_manager: u32,
    pub struct2: u32,
    step_state_ptr: u32,
    pub sales_data: u32,
    pub substep: u8,
    step: u8,
    pub trigger_disconnect: Bool,
    pub has_selected_language: Bool,
    pub some_bool: Bool,
}

impl AppState {
    pub fn step(&self) -> AppStateStep {
        self.step.into()
    }

    pub fn step_state_mut(&mut self) -> &mut StepState {
        let slice =
            unsafe { from_raw_parts_mut(self.step_state_ptr as *mut u8, size_of::<StepState>()) };
        StepState::mut_from_bytes(slice).unwrap()
    }
}

const _: () = assert!(core::mem::size_of::<AppState>() == 0x1d);

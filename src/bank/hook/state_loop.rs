use core::mem::size_of;
use core::slice::from_raw_parts_mut;

use zerocopy::FromBytes;

use super::super::game_structs::{AppState, AppStateStep};

pub extern "C" fn get_next_step(state_ptr: *mut u8, current_step: AppStateStep) -> AppStateStep {
    use AppStateStep::*;

    let state_buf = unsafe { from_raw_parts_mut(state_ptr, size_of::<AppState>()) };
    let state = AppState::mut_from_bytes(state_buf).unwrap();

    if state.trigger_disconnect.get() {
        state.trigger_disconnect.set(false);
        return Disconnect;
    }

    let step_state = state.step_state_mut();

    let result_code = step_state.result_code;

    match (current_step, result_code) {
        (GameStart, 5) if state.has_selected_language.get() => TitleScreen,
        (GameStart, 5) => SelectLanguage,
        (GameStart, _) => Case0x18,
        (SelectLanguage, 5) => {
            state.has_selected_language.set(true);
            LoadToTitleScreen
        }
        (SelectLanguage, _) => Case0x18,
        (TitleScreen, 4) => CheckingGameCart,
        (TitleScreen, _) => Case0x18,
        (CheckingGameCart, 4) => {
            state.some_bool.set(false);
            // Original: ConnectingToInternet
            ParseSalesData
        }
        (CheckingGameCart, 0x17) => {
            state.some_bool.set(true);
            // Original: ConnectingToInternet
            ParseSalesData
        }
        (LoggedIn, 0xc) => SelectGame,
        (LoggedIn, 0xd) => Case0x0e,
        (LoggedIn, 0x17) => MovePokemonToHome,
        (ConnectingToInternet, 4) => ConnectedToInternet,
        (ConnectingToInternet, 0x12) => BankIsOutdated,
        (SaveBoxesToServerAndQuit, 0x14) => SaveBoxesToServer,
        (ConnectedToInternet, 4) => ParseSalesData,
        (Case0x09, 4) if state.some_bool.get() => MovePokemonToHome,
        (Case0x09, 4) => LoggedIn,
        (SelectGame, 5) => SelectedGame,
        (SelectGame, 0x13) => LoggedIn,
        (SelectedGame, 9) => SelectGame2,
        (SelectedGame, 10) => SelectGame1,
        (PokeMileCheck, 5) => GetPokeMiles,
        (PokeMileCheck, 6) => ViewBankBoxes,
        (PokeMileCheck, 0xc) => ViewBankBoxes,
        // Original: (PokeMileCheck, _) => QuitWithoutSaving,
        (GetPokeMiles, 5) => ViewBankBoxes,
        // Original: (GetPokeMiles, _) => QuitWithoutSaving,
        (Case0x09, _) => LoggedIn,
        (ParseSalesData, 4) => Case0x09,
        // Original: (DownloadBankData, 4) => PokeMileCheck,
        // Original: (DownloadBankData, 0x4) => SaveBoxesToServer,
        (DownloadBankData, _) => ViewBankBoxes,
        (SelectGame1, 4) => DownloadBankData,
        (SelectGame1, 0x14) => SaveBoxesToServer,
        (SelectGame2, 4) => SelectGame1,
        (SelectGame2, 0x16) => Case0x17,
        (Disconnect, _) => TitleScreen,
        (LoadToTitleScreen, _) => TitleScreen,
        (Case0x17, 4) => SelectGame1,
        (Case0x17, 9) => SelectGame2,
        (Case0x17, 0xe) => Case0x18,
        (ViewBankBoxes, 4) => SaveBoxesToServerAndQuit,
        // Original: (ViewBankBoxes, _) => QuitWithoutSaving,
        (Case0x1a, _) => GameStart,
        // Original: (Case0x1b, _) => QuitWithoutSaving,
        (MovePokemonToHome, 4) => Case0x1d,
        (Case0x1d, 4) => Case0x1b,
        // Original: _ => Disconnect
        _ => LoadToTitleScreen,
    }
}

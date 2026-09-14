use super::game_structs::AppStateStep;
use super::plugin_mode::{PluginMode, get_plugin_mode};
use super::reader::BankReader;
use crate::bank::bank_file::BankFile;
use crate::ctr;
use crate::ctr::{is_citra, set_print_max_len};
use crate::ui::{ShowView, draw_version};
use once_cell::unsync::Lazy;

struct PersistedState {
    show_debug_info: ShowView,
    has_dumped_bank_file: bool,
}

unsafe fn get_state() -> &'static mut PersistedState {
    static mut STATE: Lazy<PersistedState> = Lazy::new(|| PersistedState {
        show_debug_info: ShowView::new(true),
        has_dumped_bank_file: false,
    });
    Lazy::force_mut(&mut STATE)
}

fn draw_debug_info() {
    let reader = BankReader::new();
    let app_state = reader.app_state();

    ctr::println!("Step: {:?}", app_state.step());
    ctr::println!("Substep: {}", app_state.substep);
    ctr::println!("Is citra: {}", is_citra());
    draw_version();
}

fn run_uninitialized_mode_frame() {
    ctr::println!("Initializing plugin...");
}

fn run_failed_to_init_mode_frame(reason: &str) {
    ctr::println!("Failed to init!");
    ctr::println!("{}", reason);
}

fn run_offline_mode_frame() {
    ctr::println!("Offline mode");
}

fn run_migrate_mode_frame(state: &mut PersistedState) {
    let reader = BankReader::new();
    let app_state = reader.app_state();

    ctr::println!("Migrate mode");
    ctr::println!("");

    match state.has_dumped_bank_file {
        false => {
            ctr::println!("Migrate to offline mode");
            ctr::println!("by viewing your bank");
        }
        true => {
            ctr::println!("Bank file dumped!");
            ctr::println!("");
            ctr::println!("To enter offline mode:");
            ctr::println!("  1. Return to the home screen");
            ctr::println!("  2. Close Pokemon Bank");
            ctr::println!("  3. Restart Bank");
        }
    }

    if app_state.step() == AppStateStep::ViewBankBoxes && !state.has_dumped_bank_file {
        let res = BankFile::write_to_sd();
        ctr::println!("Res {:x?}", res);
        if res.is_ok() {
            state.has_dumped_bank_file = true;
        }
    }
}

pub fn run_frame() {
    set_print_max_len(28);
    let state = unsafe { get_state() };

    match get_plugin_mode() {
        PluginMode::Uninitialized => run_uninitialized_mode_frame(),
        PluginMode::FailedToInit { reason } => run_failed_to_init_mode_frame(reason),
        PluginMode::Offline => run_offline_mode_frame(),
        PluginMode::Migrate => run_migrate_mode_frame(state),
    }

    if state.show_debug_info.check() {
        return;
    }

    draw_debug_info();
}

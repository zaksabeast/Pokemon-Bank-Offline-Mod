use super::bank_file::BankFile;
use super::fs::ensure_sd_data_exists;
use super::hook::install_offline_hooks;
use super::plugin_mode::{PluginMode, set_plugin_mode};
use crate::ctr::is_citra;

pub fn init() {
    match ensure_sd_data_exists() {
        Err(_) => set_plugin_mode(PluginMode::FailedToInit {
            reason: "Could not create sd data",
        }),
        Ok(_) => {
            let bank_file_exists = BankFile::sd_file_exists();
            let is_physical_console = !is_citra();
            let plugin_mode = match (is_physical_console, bank_file_exists) {
                (true, false) => PluginMode::Migrate,
                _ => PluginMode::Offline,
            };
            set_plugin_mode(plugin_mode);

            if plugin_mode == PluginMode::Offline {
                install_offline_hooks();
            }
        }
    }
}

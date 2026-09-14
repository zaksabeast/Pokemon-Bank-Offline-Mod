#[derive(PartialEq, Eq, Clone, Copy)]
pub enum PluginMode {
    Uninitialized,
    Offline,
    Migrate,
    FailedToInit { reason: &'static str },
}

static mut PLUGIN_MODE: PluginMode = PluginMode::Uninitialized;

pub fn set_plugin_mode(mode: PluginMode) {
    unsafe { PLUGIN_MODE = mode };
}

pub fn get_plugin_mode() -> PluginMode {
    unsafe { PLUGIN_MODE }
}

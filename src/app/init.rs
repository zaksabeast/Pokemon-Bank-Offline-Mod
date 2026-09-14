use crate::app::{LoadedTitle, install_hooks, loaded_title};
use crate::bank;

pub fn init() -> Option<()> {
    let title = loaded_title().as_ref().ok()?;

    let _ = install_hooks();

    match title {
        LoadedTitle::Bank => bank::init(),
    }

    Some(())
}

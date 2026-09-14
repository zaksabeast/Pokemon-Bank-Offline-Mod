use crate::app::{LoadedTitle, TitleError, loaded_title};
use crate::bank;
use crate::ui::{draw_could_not_get_program_info, draw_unsupported_update};

fn run_loaded_title_frame(title: &LoadedTitle) {
    match title {
        LoadedTitle::Bank => bank::run_frame(),
    }
}

pub fn run_frame() {
    match loaded_title() {
        Ok(title) => run_loaded_title_frame(title),
        Err(TitleError::CouldNotGetProgramInfo) => draw_could_not_get_program_info(),
        Err(TitleError::InvalidUpdate {
            remaster_version,
            debug_info,
            is_citra,
        }) => draw_unsupported_update(*remaster_version, debug_info, *is_citra),
        Err(TitleError::InvalidTitle) => {}
    }
}

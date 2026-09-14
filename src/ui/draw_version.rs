use crate::{GIT_HASH, VERSION, ctr};
use alloc::string::String;

pub fn draw_version() {
    ctr::println!("Ver {} {}", VERSION, GIT_HASH);
}

pub fn draw_could_not_get_program_info() {
    ctr::println!("Could not get program info!");
    ctr::println!("Please report this!");
}

pub fn draw_unsupported_update(remaster_version: u16, debug_info: &Option<String>, is_citra: bool) {
    ctr::println!("Unsupported game update!");
    ctr::println!("");
    ctr::println!("Please update your game");
    ctr::println!("for this mod to run");
    ctr::println!("");
    ctr::println!("Detected info:");
    ctr::println!(
        "Playing on {}",
        match is_citra {
            true => "Citra",
            false => "Real hardware",
        }
    );
    ctr::println!("Update ver {}", remaster_version);
    ctr::println!("Debug: {}", debug_info.as_deref().unwrap_or_default());
    ctr::println!("");
    ctr::println!("this mod version:");
    ctr::println!("{} {}", VERSION, GIT_HASH);
}

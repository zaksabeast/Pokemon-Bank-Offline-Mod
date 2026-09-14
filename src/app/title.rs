use crate::ctr::program::get_program_info;
use crate::ctr::{is_citra, mem};
use alloc::format;
use alloc::string::String;
use num_enum::TryFromPrimitive;

#[derive(Clone, Copy, Debug, PartialEq, Eq, TryFromPrimitive)]
#[repr(u64)]
pub enum LoadedTitle {
    Bank = 0x00040000000C9B00,
}

#[derive(Debug, Clone)]
pub enum TitleError {
    CouldNotGetProgramInfo,
    InvalidTitle,
    InvalidUpdate {
        remaster_version: u16,
        debug_info: Option<String>,
        is_citra: bool,
    },
}

static mut LOADED: bool = false;
static mut LOAD_RESULT: Result<LoadedTitle, TitleError> = Err(TitleError::InvalidTitle);

// Older citra builds can't reliably report the game's version
// via the fs sysmodule. For that environment we detect whether
// the running game is the latest version by checking a known, unique
// 16-byte value located at a fixed address in the game's memory.
fn check_citra_title_version(addr: u32, expected: &'static [u8; 16], version: u16) -> UpdateInfo {
    let version_bytes = mem::read_array::<16>(addr);
    let version = match &version_bytes == expected {
        true => version,
        false => 0,
    };
    UpdateInfo {
        version,
        debug_info: Some(
            version_bytes
                .iter()
                .map(|byte| format!("{:02x}", byte))
                .collect::<String>(),
        ),
    }
}

struct UpdateInfo {
    version: u16,
    debug_info: Option<String>,
}

fn get_citra_title_version(title: LoadedTitle) -> UpdateInfo {
    match title {
        LoadedTitle::Bank => check_citra_title_version(0x2ac24c, b"vgBivYesOH9RS5I8", 6),
    }
}

fn get_update_version(title: LoadedTitle, version: u16) -> UpdateInfo {
    if is_citra() {
        return get_citra_title_version(title);
    }

    UpdateInfo {
        version,
        debug_info: None,
    }
}

pub fn loaded_title() -> &'static Result<LoadedTitle, TitleError> {
    // Reader is single-threaded, so this is safe.
    // Even then, title and update version will also always be the same values.
    unsafe {
        if LOADED {
            return &LOAD_RESULT;
        }

        LOADED = true;

        let program_info = match get_program_info() {
            Ok(info) => info,
            Err(_) => {
                LOAD_RESULT = Err(TitleError::CouldNotGetProgramInfo);
                return &LOAD_RESULT;
            }
        };

        let title = match program_info.title_id.try_into() {
            Ok(title) => title,
            Err(_) => {
                LOAD_RESULT = Err(TitleError::InvalidTitle);
                return &LOAD_RESULT;
            }
        };

        let update_info = get_update_version(title, program_info.revision_version);
        LOAD_RESULT = match (title, update_info.version) {
            (LoadedTitle::Bank, 6) => Ok(title),
            (_, remaster_version) => Err(TitleError::InvalidUpdate {
                remaster_version,
                debug_info: update_info.debug_info,
                is_citra: is_citra(),
            }),
        };

        &LOAD_RESULT
    }
}

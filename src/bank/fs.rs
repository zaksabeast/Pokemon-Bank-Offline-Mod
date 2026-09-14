use crate::ctr::fs::Archive;
use crate::ctr::res::CtrResult;

const OFFLINE_DATA_DIRECTORY: &str = "/3ds/PokemonBankOffline";
pub const OFFLINE_SAVE_PATH: &str = "/3ds/PokemonBankOffline/sav.bin";
pub const BANK_FILE_PATH: &str = "/3ds/PokemonBankOffline/bank_data.bin";

pub fn ensure_sd_data_exists() -> CtrResult<()> {
    let sd = Archive::sd()?;

    let _ = sd.create_dir("/3ds");
    let _ = sd.create_dir(OFFLINE_DATA_DIRECTORY);

    Ok(())
}

use super::fs::BANK_FILE_PATH;
use super::game_structs::Bool;
use super::reader::BankReader;
use crate::ctr::fs::{Archive, File};
use crate::ctr::res::CtrResult;
use core::iter::once;
use core::slice::{from_raw_parts, from_raw_parts_mut};
use zerocopy::{
    FromBytes, FromZeros, I32, Immutable, IntoBytes, KnownLayout, LE, U16, Unalign, Unaligned,
};

// Unaligned compatible
pub type U16Le = U16<LE>;
pub type I32Le = I32<LE>;

fn copy_buf<'a, Src: Iterator<Item = u16>, Dst: Iterator<Item = &'a mut U16Le>>(
    src: Src,
    dst: Dst,
) {
    src.zip(dst).for_each(|(src, dst)| {
        dst.set(src);
    });
}

fn encode_digit(num: u16, long_width: bool) -> u16 {
    match long_width {
        true => ((0x10 + num) << 8) | 0xff,
        false => 0x30 + num,
    }
}

fn encode_num(num: u16, long_width: bool) -> ([u16; 3], usize, usize) {
    let hundreds = num / 100;
    let tens = (num / 10) % 10;
    let ones = num % 10;

    let out = [
        encode_digit(hundreds, long_width),
        encode_digit(tens, long_width),
        encode_digit(ones, long_width),
    ];

    let skip = match (hundreds, tens) {
        (0, 0) => 2, // 0..=9
        (0, _) => 1, // 10..=99
        (_, _) => 0, // 100
    };
    let digits = 3 - skip;

    (out, skip, digits)
}

fn get_limit(long_width: bool) -> usize {
    match long_width {
        true => 8,
        false => 14,
    }
}

fn set_name(buf: &mut [U16Le], text: &str, num: u16, long_width: bool) {
    let limit = get_limit(long_width);
    let (num_chars, num_skip, digits) = encode_num(num, long_width);
    let num_iter = num_chars.into_iter().skip(num_skip);
    let nul = once(0x00_u16);

    let src = text
        .encode_utf16()
        .take(limit - digits)
        .chain(num_iter)
        .chain(nul);
    let dst = buf.iter_mut();
    copy_buf(src, dst);
}

#[derive(FromBytes, IntoBytes, Immutable, KnownLayout, Unaligned)]
#[repr(C)]
pub struct PokeCoreData {
    data: [u8; 0xe8],
}

const _: () = assert!(core::mem::size_of::<PokeCoreData>() == 0xe8);

impl PokeCoreData {
    fn empty() -> Self {
        Self {
            data: [
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7E, 0xE9, 0x71, 0x52,
                0xB0, 0x31, 0x42, 0x8E, 0xCC, 0xE2, 0xC5, 0xAF, 0xDB, 0x67, 0x33, 0xFC, 0x2C, 0xEF,
                0x5E, 0xFC, 0xC5, 0xCA, 0xD6, 0xEB, 0x3D, 0x99, 0xBC, 0x7A, 0xA7, 0xCB, 0xD6, 0x5D,
                0x78, 0x91, 0xA6, 0x27, 0x8D, 0x61, 0x92, 0x16, 0xB8, 0xCF, 0x5D, 0x37, 0x80, 0x30,
                0x7C, 0x40, 0xFB, 0x48, 0x13, 0x32, 0xE7, 0xFE, 0xA3, 0xDF, 0x69, 0x3D, 0x9E, 0x63,
                0x29, 0x1D, 0x8D, 0xEA, 0x96, 0x62, 0x68, 0x92, 0x97, 0xA3, 0x49, 0x1C, 0x03, 0x6E,
                0xAA, 0x31, 0x89, 0xAA, 0xC5, 0xD3, 0xEA, 0xC3, 0xD9, 0x82, 0xC6, 0xE0, 0x5C, 0x94,
                0x3B, 0x4E, 0x5F, 0x5A, 0x28, 0x24, 0xB3, 0xFB, 0xE1, 0xBF, 0x8E, 0x7B, 0x7F, 0x00,
                0xC4, 0x40, 0x48, 0xC8, 0xD1, 0xBF, 0xB6, 0x38, 0x3B, 0x90, 0x23, 0xFB, 0x23, 0x7D,
                0x34, 0xBE, 0x00, 0xDA, 0x6A, 0x70, 0xC5, 0xDF, 0x84, 0xBA, 0x14, 0xE4, 0xA1, 0x60,
                0x2B, 0x2B, 0x38, 0x8F, 0xA0, 0xB6, 0x60, 0x41, 0x36, 0x16, 0x09, 0xF0, 0x4B, 0xB5,
                0x0E, 0x26, 0xA8, 0xB6, 0x43, 0x7B, 0xCB, 0xF9, 0xEF, 0x68, 0xD4, 0xAF, 0x5F, 0x74,
                0xBE, 0xC3, 0x61, 0xE0, 0x95, 0x98, 0xF1, 0x84, 0xBA, 0x11, 0x62, 0x24, 0x80, 0xCC,
                0xC4, 0xA7, 0xA2, 0xB7, 0x55, 0xA8, 0x5C, 0x1C, 0x42, 0xA2, 0x3A, 0x86, 0x05, 0xAD,
                0xD2, 0x11, 0x19, 0xB0, 0xFD, 0x57, 0xE9, 0x4E, 0x60, 0xBA, 0x1B, 0x45, 0x2E, 0x17,
                0xA9, 0x34, 0x93, 0x2D, 0x66, 0x09, 0x2D, 0x11, 0xE0, 0xA1, 0x74, 0x42, 0xC4, 0x73,
                0x0B, 0x2B, 0x23, 0xF2, 0x43, 0x28, 0x54, 0xA6,
            ],
        }
    }
}

#[derive(FromBytes, IntoBytes, Immutable, KnownLayout, Unaligned)]
#[repr(C)]
pub struct BankBox {
    slots: [PokeCoreData; 30],
    box_name: [U16Le; 18],
    box_id: U16Le,
}

impl BankBox {
    fn set_info(&mut self, name: &str, box_id: u16, long_width: bool) {
        self.box_name.zero();
        set_name(&mut self.box_name, name, box_id + 1, long_width);
        self.box_id.set(box_id);
    }
}

const _: () = assert!(core::mem::size_of::<BankBox>() == 0x1b56);

#[derive(FromBytes, IntoBytes, Immutable, KnownLayout, Unaligned)]
#[repr(C)]
pub struct SaveAccomplishments {
    total_pokemon_caught: I32Le,
    pokemon_caught_by_fishing: I32Le,
    eggs_hatched: I32Le,
    pokemon_evolved: I32Le,
    fossils_restored: I32Le,
    wild_pokemon_encountered: I32Le,
    number_of_trades: I32Le,
    pokemon_caught_in_one_day: I32Le,
    pokemon_evolved_in_one_day: I32Le,
}

const _: () = assert!(core::mem::size_of::<SaveAccomplishments>() == 0x24);

#[derive(FromBytes, IntoBytes, Immutable, KnownLayout, Unaligned)]
#[repr(C)]
pub struct GameRecords {
    player_name: [U16Le; 13],
    // 1: data exists
    // 2: no data exists
    // other nums: ?
    update_flag: U16Le,
    tid: U16Le,
    sid: U16Le,
    accomplishments: SaveAccomplishments,
}

const _: () = assert!(core::mem::size_of::<GameRecords>() == 0x44);

#[derive(FromBytes, IntoBytes, Immutable, KnownLayout)]
#[repr(C)]
pub struct SomeStruct {
    magic: u32,
    unk1: u32,
    unk2: [u8; 0x7258],
}

const _: () = assert!(core::mem::size_of::<SomeStruct>() == 0x7260);

#[derive(FromBytes, IntoBytes, Immutable, KnownLayout)]
#[repr(C, packed)]
pub struct BankFile {
    uid: u64,
    group_names: Unalign<[[U16Le; 17]; 10]>,
    // 2 makes bank think most game records are updated already
    // any other number doesn't do this.
    game_record_update_flag: u16,
    box_count: u16,
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    unk: [u8; 22],
    boxes: Unalign<[BankBox; 100]>,
    transfer_box: [PokeCoreData; 30],
    is_bank_slot_gen_7_mon: [Bool; 3000],
    is_transfer_slot_gen_7_mon: [Bool; 30],
    unk2: u16,
    game_records: Unalign<[GameRecords; 8]>,
    unk3: SomeStruct,
    unk4: u32,
    more_bools: [Bool; 3000],
    timestamps: [u64; 3000],
    unk5: u8,
    // This is a struct I haven't labled yet
    unk6: [u8; 0xff],
}

const _: () = assert!(core::mem::size_of::<BankFile>() == 0xbb518);

impl BankFile {
    fn reinit(&mut self) {
        // Todo:
        // make these dynamic based on language
        let base_group_name = "Group ";
        let base_box_name = "Bank ";
        let long_width = false;

        self.group_names
            .iter_mut()
            .enumerate()
            .for_each(|(i, group_name)| {
                group_name.zero();
                let group_id = (i + 1) as u16;
                set_name(group_name, base_group_name, group_id, long_width)
            });

        self.boxes.iter_mut().enumerate().for_each(|(i, bank_box)| {
            bank_box.set_info(base_box_name, i as u16, long_width);
            bank_box
                .slots
                .iter_mut()
                .for_each(|slot| *slot = PokeCoreData::empty());
        });

        self.transfer_box
            .iter_mut()
            .for_each(|slot| *slot = PokeCoreData::empty());

        self.game_records.iter_mut().for_each(|record| {
            record.as_mut_bytes().zero();
            record.update_flag.set(2);
        });

        self.unk3 = SomeStruct::new_zeroed();
        self.unk3.magic = 0x545a4b4e;
        self.unk3.unk2[0x4fd0] = 1;

        self.uid = 0;
        self.game_record_update_flag = 0;
        self.box_count = 100;
        self.year = 2026;
        self.month = 1;
        self.day = 1;
        self.hour = 0;
        self.minute = 0;
        self.unk = [0; 22];
        self.is_bank_slot_gen_7_mon = [Bool::new_false(); 3000];
        self.is_transfer_slot_gen_7_mon = [Bool::new_false(); 30];
        self.unk2 = 0;
        self.unk4 = 0;
        self.more_bools = [Bool::new_false(); 3000];
        self.timestamps = [0; 3000];
        self.unk5 = 0;
        self.unk6 = [0; 0xff];
    }
}

impl BankFile {
    const DATA_LEN: usize = core::mem::size_of::<BankFile>();

    pub fn sd_file_exists() -> bool {
        Archive::sd()
            .map(|sd| sd.file_exists(BANK_FILE_PATH))
            .unwrap_or_default()
    }

    fn open_sd_file() -> CtrResult<File> {
        let sd = Archive::sd()?;
        sd.open_file(BANK_FILE_PATH)
    }

    fn create_sd_file() -> CtrResult<()> {
        let sd = Archive::sd()?;
        sd.create_file(BANK_FILE_PATH)
    }

    pub fn read_from_sd() -> CtrResult<()> {
        let ptr = BankReader::new().bank_data_ptr();
        if ptr.is_null() {
            return Err(-1);
        }

        let data = unsafe { from_raw_parts_mut(ptr, Self::DATA_LEN) };

        match Self::open_sd_file() {
            Ok(file) => {
                file.read(data)?;
            }
            Err(_) => {
                let bank_file = BankFile::mut_from_bytes(data).map_err(|_| -1)?;
                bank_file.reinit();
            }
        };

        Ok(())
    }

    pub fn write_to_sd() -> CtrResult<u32> {
        let ptr = BankReader::new().bank_data_ptr();
        if ptr.is_null() {
            return Err(-1);
        }

        let data = unsafe { from_raw_parts(ptr, Self::DATA_LEN) };

        if !Self::sd_file_exists() {
            Self::create_sd_file()?;
        }

        let file = Self::open_sd_file()?;
        file.write(data)
    }
}

use num_enum::FromPrimitive;

#[derive(Debug, Clone, Copy)]
pub struct ArchiveHandle(pub u64);

impl ArchiveHandle {
    pub fn low(&self) -> u32 {
        self.0 as u32
    }

    pub fn high(&self) -> u32 {
        (self.0 >> 32) as u32
    }
}

#[derive(Default, FromPrimitive)]
#[repr(u32)]
pub enum FsArchiveId {
    #[default]
    Invalid = 0x0,
    Romfs = 0x3,
    Savedata = 0x4,
    Extdata = 0x6,
    SharedExtdata = 0x7,
    SystemSavedata = 0x8,
    Sdmc = 0x9,
    SdmcWriteOnly = 0xA,
    BossExtdata = 0x12345678,
    CardSpifs = 0x12345679,
    ExtdataAndBossExtdata = 0x1234567B,
    SystemSAVEDATA2 = 0x1234567C,
    NandRw = 0x1234567D,
    NandRo = 0x1234567E,
    NandRoWriteAccess = 0x1234567F,
    SavedataAndContent = 0x2345678A,
    SavedataAndCONTENT2 = 0x2345678E,
    NandCtrFs = 0x567890AB,
    TwlPhoto = 0x567890AC,
    TwlSound = 0x567890AD,
    NandTwlFs = 0x567890AE,
    NandWFs = 0x567890AF,
    GamecardSavedata = 0x567890B1,
    UserSavedata = 0x567890B2,
    DemoSavedata = 0x567890B4,
}

#[derive(Default, Debug, Clone, Copy, FromPrimitive)]
#[repr(u32)]
pub enum FsOpenFlags {
    #[default]
    None = 0,
    Read = 1,
    Write = 2,
    ReadWrite = 3,
    Create = 4,
}

#[derive(Default, Debug, Clone, Copy, FromPrimitive)]
#[repr(u32)]
pub enum FsWriteFlags {
    #[default]
    Flush = 1,
    UpdateTime = 256,
}

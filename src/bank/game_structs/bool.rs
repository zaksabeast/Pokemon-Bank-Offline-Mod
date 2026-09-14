use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

/// Zerocopy bool
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromBytes, IntoBytes, Immutable, KnownLayout)]
pub struct Bool(u8);

impl Bool {
    pub fn new_true() -> Self {
        Self(1)
    }

    pub fn new_false() -> Self {
        Self(0)
    }

    pub fn get(&self) -> bool {
        self.0 != 0
    }

    pub fn set(&mut self, val: bool) {
        self.0 = val as u8;
    }
}

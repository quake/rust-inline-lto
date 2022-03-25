#![no_std]

pub struct Byte32Reader<'r>(pub &'r [u8]);

impl<'r> Byte32Reader<'r> {
    #[inline]
    pub fn unpack(&self) -> [u8; 32] {
        self.0.try_into().expect("unpack Byte32Reader")
    }
}

pub struct Byte32<'r>(pub &'r [u8]);

impl<'r> Byte32<'r> {
    #[inline]
    pub fn as_reader(&self) -> Byte32Reader {
        Byte32Reader(self.0)
    }

    #[inline]
    pub fn unpack(&self) -> [u8; 32] {
        self.as_reader().unpack()
    }

    pub fn unpack_noninline(&self) -> [u8; 32] {
        self.as_reader().unpack()
    }
}

pub fn input() -> [u8; 32] {
    [0u8; 32]
}

pub fn output(_: &[u8; 32]) {
}
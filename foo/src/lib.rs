#![no_std]

pub struct Byte32Reader<'r>(pub &'r [u8]);

impl<'r> Byte32Reader<'r> {
    #[cfg_attr(feature = "inline-unpack", inline)]
    pub fn unpack(&self) -> [u8; 32] {
        self.0.try_into().expect("unpack Byte32Reader")
    }
}

pub struct Byte32<'r>(pub &'r [u8]);

impl<'r> Byte32<'r> {
    pub fn as_reader(&self) -> Byte32Reader {
        Byte32Reader(self.0)
    }

    pub fn unpack(&self) -> [u8; 32] {
        self.as_reader().unpack()
    }
}
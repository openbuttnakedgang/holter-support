
use num_enum::IntoPrimitive;
use num_enum::TryFromPrimitive;


#[derive(Debug, Clone, Copy, PartialEq, Eq, )]
#[derive(IntoPrimitive, TryFromPrimitive)]
#[repr(u32)]
pub enum Error {
    OK = 0,
    LOGIC = 1,
    PRIV = 2,
    BUSY = 3,
    TIMEOUT = 4,
    PARAM = 5,
    NOSPACE = 6,
    NEEDERASE = 7,
    NOTSET = 8,
    NOTIMPL = 9,
}




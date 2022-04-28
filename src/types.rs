pub type Address = u16;
pub type RegData = u8;

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Register {
    V0,
    V1,
    V2,
    V3,
    V4,
    V5,
    V6,
    V7,
    V8,
    V9,
    Va,
    Vb,
    Vc,
    Vd,
    Ve,
    Vf,
}

impl From<u16> for Register {
    fn from(num: u16) -> Self {
        match num {
            0x0 => Register::V0,
            0x1 => Register::V1,
            0x2 => Register::V2,
            0x3 => Register::V3,
            0x4 => Register::V4,
            0x5 => Register::V5,
            0x6 => Register::V6,
            0x7 => Register::V7,
            0x8 => Register::V8,
            0x9 => Register::V9,
            0xa => Register::Va,
            0xb => Register::Vb,
            0xc => Register::Vc,
            0xd => Register::Vd,
            0xe => Register::Ve,
            0xf => Register::Vf,
            e => panic!("Error register {e} doesn't exist"),
        }
    }
}

impl std::fmt::Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Register::V0 => write!(f, "[Register V0]"),
            Register::V1 => write!(f, "[Register V1]"),
            Register::V2 => write!(f, "[Register V2]"),
            Register::V3 => write!(f, "[Register V3]"),
            Register::V4 => write!(f, "[Register V4]"),
            Register::V5 => write!(f, "[Register V5]"),
            Register::V6 => write!(f, "[Register V6]"),
            Register::V7 => write!(f, "[Register V7]"),
            Register::V8 => write!(f, "[Register V8]"),
            Register::V9 => write!(f, "[Register V9]"),
            Register::Va => write!(f, "[Register VA]"),
            Register::Vb => write!(f, "[Register VB]"),
            Register::Vc => write!(f, "[Register VC]"),
            Register::Vd => write!(f, "[Register VD]"),
            Register::Ve => write!(f, "[Register VE]"),
            Register::Vf => write!(f, "[Register VF]"),
        }
    }
}

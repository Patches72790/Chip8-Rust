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
    I, // used for writing to/from memory
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
            Register::I => write!(f, "[Register I]"),
        }
    }
}

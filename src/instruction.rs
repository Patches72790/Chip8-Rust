use crate::types::{Address, RegData, Register};

/// Instruction enum for Chip 8 instructions
/// are all prefixed with `i` for readability
/// and for compilation in rust.
#[derive(Clone, Copy)]
#[repr(u8)]
pub enum Instruction {
    i00E0,                              // Clears the display
    i00E1,                              // Sets all bits of display
    i1NNN(Address),                     // Jump to address NNN
    i6XNN(Register, RegData),           // store value NN at register X
    i7XNN(Register, RegData),           // Add data NN to register X
    iANNN(Address),                     // Store memory address NNN in Register i
    iBNNN(Address),                     // Jump to adresss NNN + V0
    iDXYN(Register, Register, RegData), // Draw at position (VX, VY) N bytes of sprite data starting at address stored in I
}



impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::i00E0 => write!(f, "00E0"),
            Instruction::i00E1 => write!(f, "00E1"),
            Instruction::i1NNN(addr) => write!(f, "1NNN | {}", addr),
            Instruction::i6XNN(reg, data) => write!(f, "6XNN | X={reg} | NN={data}"),
            Instruction::i7XNN(reg, data) => write!(f, "7XNN | X={reg} | NN={data}"),
            Instruction::iANNN(addr) => write!(f, "ANNN | NNN={addr}"),
            Instruction::iBNNN(addr) => write!(f, "BNNN | B={addr}"),
            Instruction::iDXYN(reg1, reg2, data) => {
                write!(f, "DXYN | X={reg1} | Y={reg2} | data={data}")
            }
        }
    }
}
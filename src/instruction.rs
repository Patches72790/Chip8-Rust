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
    i2NNN(Address),                     // Execute subroutine at address NNN
    i3XNN(Register, RegData),           // Skip following instruction if VX == NN
    i4XNN(Register, RegData),           // Skip following instruction if VX != NN
    i5XY0(Register, Register),          // Skip following instruction if VX == VY
    i6XNN(Register, RegData),           // store value NN at register X
    i7XNN(Register, RegData),           // Add data NN to register X
    i8XY0(Register, Register),          // VX = *VY
    i8XY1(Register, Register),          // VX = VX | VY
    i8XY2(Register, Register),          // VX = VX & VY
    i8XY3(Register, Register),          // VX = VX ^ VY
    i8XY4(Register, Register),          // VX = VX + VY -> VF = 1 if carry, 0 otherwise
    i8XY5(Register, Register),          // VX = VX - VY -> VF = 1 if carry, 0 otherwise
    i8XY6(Register, Register),          // VX = VY >> 1 -> VF = LSB of VY before shift
    i8XY7(Register, Register),          // VX = VY - VX -> CAREFUL !!! VF = 0 if carry, 1 otherwise
    i8XYE(Register, Register),          // VX = VY << 1 -> VF = MSB of VY before shift
    i9XY0(Register, Register),          // TODO
    iANNN(Address),                     // Store memory address NNN in Register i
    iBNNN(Address),                     // Jump to adresss NNN + V0
    iCXNN(Register, Address),           // TODO
    iDXYN(Register, Register, RegData), // Draw at position (VX, VY) N bytes of sprite data starting at address stored in I
    iEX9E(Register),                    // TODO
    iEXA1(Register),                    // TODO
    iFX07(Register),                    // TODO
    iFX0A(Register),                    // TODO
    iFX15(Register),                    // TODO
    iFX18(Register),                    // TODO
    iFX1E(Register),                    // TODO
    iFX29(Register),                    // TODO
    iFX33(Register),                    // TODO
    iFX55(Register),                    // TODO
    iFX65(Register),                    // TODO
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::i00E0 => write!(f, "00E0"),
            Instruction::i00E1 => write!(f, "00E1"),
            Instruction::i1NNN(addr) => write!(f, "1NNN | {}", addr),
            Instruction::i2NNN(addr) => write!(f, "2NNN | B={addr}"),
            Instruction::i3XNN(reg, data) => write!(f, "3XNN | X={reg} | NN={data}"),
            Instruction::i4XNN(reg, data) => write!(f, "4XNN | X={reg} | NN = {data}"),
            Instruction::i5XY0(reg1, reg2) => write!(f, "5XY0 | X={reg1} | Y={reg2}"),
            Instruction::i6XNN(reg, data) => write!(f, "6XNN | X={reg} | NN={data}"),
            Instruction::i7XNN(reg, data) => write!(f, "7XNN | X={reg} | NN={data}"),
            Instruction::i8XY0(reg1, reg2) => write!(f, "8XY0 | X={reg1} | Y={reg2}"),
            Instruction::i8XY1(reg1, reg2) => write!(f, "8XY1 | X={reg1} | Y={reg2}"),
            Instruction::i8XY2(reg1, reg2) => write!(f, "8XY2 | X={reg1} | Y={reg2}"),
            Instruction::i8XY3(reg1, reg2) => write!(f, "8XY3 | X={reg1} | Y={reg2}"),
            Instruction::i8XY4(reg1, reg2) => write!(f, "8XY4 | X={reg1} | Y={reg2}"),
            Instruction::i8XY5(reg1, reg2) => write!(f, "8XY5 | X={reg1} | Y={reg2}"),
            Instruction::i8XY6(reg1, reg2) => write!(f, "8XY6 | X={reg1} | Y={reg2}"),
            Instruction::i8XY7(reg1, reg2) => write!(f, "8XY7 | X={reg1} | Y={reg2}"),
            Instruction::i8XYE(reg1, reg2) => write!(f, "8XYE | X={reg1} | Y={reg2}"),
            Instruction::iANNN(addr) => write!(f, "ANNN | NNN={addr}"),
            Instruction::iBNNN(addr) => write!(f, "BNNN | B={addr}"),
            Instruction::iDXYN(reg1, reg2, data) => {
                write!(f, "DXYN | X={reg1} | Y={reg2} | data={data}")
            }
            _ => todo!("Unimplemented instruction"),
        }
    }
}

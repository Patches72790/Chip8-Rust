use crate::types::{Address, RegData, Register};
use fixedbitset::FixedBitSet;
use wasm_bindgen::prelude::*;

/// 16 General Purpose registers for use in arithmetic and
/// logical operations plus a memory addressing register `i`
/// for reading/writing memory.
struct Registers {
    pub v0: RegData,
    pub v1: RegData,
    pub v2: RegData,
    pub v3: RegData,
    pub v4: RegData,
    pub v5: RegData,
    pub v6: RegData,
    pub v7: RegData,
    pub v8: RegData,
    pub v9: RegData,
    pub va: RegData,
    pub vb: RegData,
    pub vc: RegData,
    pub vd: RegData,
    pub ve: RegData,
    pub vf: RegData,
    pub i: Address, // used for writing to/from memory
}

impl Registers {
    pub fn new() -> Self {
        Self {
            v0: 0,
            v1: 0,
            v2: 0,
            v3: 0,
            v4: 0,
            v5: 0,
            v6: 0,
            v7: 0,
            v8: 0,
            v9: 0,
            va: 0,
            vb: 0,
            vc: 0,
            vd: 0,
            ve: 0,
            vf: 0,
            i: 0,
        }
    }
}

/// Instruction enum for Chip 8 instructions
/// are all prefixed with `i` for readability
/// and for compilation in rust.
#[derive(Clone, Copy)]
#[repr(u16)]
pub enum Instruction {
    i00E0,                             // Clears the display
    i1NNN(Address),                    // Jump to address NNN
    iANNN(Address),                    // Store memory address NNN in Register i
    iBNNN(Address),                    // Jump to adresss NNN + V0
    i6XNN(Register, RegData),          // store value NN at register X
    i7XNN(Register, RegData),          // Add data NN to register X
    iDXYN(Register, Register, RegData), // Draw at position (VX, VY) N bytes of sprite data starting at address stored in I
}

#[wasm_bindgen]
pub struct Cpu {
    memory: [Option<Instruction>; 4096], // 12 KB of memory, instructions starting at 0x200
    registers: Registers,
    clock: u128,
    display: FixedBitSet, // display fixed at 64 * 32 pixels
    ip: usize,            // instruction pointer
}

#[wasm_bindgen]
impl Cpu {
    pub fn new() -> Cpu {
        println!("Creating a new cpu");
        Cpu {
            memory: [None; 4096],
            registers: Registers::new(),
            clock: 0,
            display: FixedBitSet::with_capacity(64 * 32),
            ip: 0,
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn tick(&mut self) {
        self.interpret();
    }

    /// Main interpreter loop for fetching, decoding, executing instructions.
    /// This is invoked each "cycle" from the public tick function in the cpu impl.
    fn interpret(&mut self) {
        while let Some(instruction) = self.fetch_instruction() {
            match instruction {
                Instruction::i00E0 => self.display.clear(),
                Instruction::i1NNN(address) => self.ip = address as usize,
                Instruction::iANNN(address) => self.registers.i = address,
                Instruction::iBNNN(address) => {
                    let reg_v0 = self.registers.v0;
                    self.ip = (address + (reg_v0 as u16)) as usize;
                }
                Instruction::i6XNN(reg, data) => self.store_at_register(reg, data),
                _ => todo!("Instruction not yet implemented"),
            }
        }
    }

    fn store_at_register(&mut self, reg: Register, data: RegData) {
        match reg {
            Register::V0 => self.registers.v0 = data,
            Register::V1 => self.registers.v1 = data,
            Register::V2 => self.registers.v2 = data,
            Register::V3 => self.registers.v3 = data,
            Register::V4 => self.registers.v4 = data,
            Register::V5 => self.registers.v5 = data,
            Register::V6 => self.registers.v6 = data,
            Register::V7 => self.registers.v7 = data,
            Register::V8 => self.registers.v8 = data,
            Register::V9 => self.registers.v9 = data,
            Register::Va => self.registers.va = data,
            Register::Vb => self.registers.vb = data,
            Register::Vc => self.registers.vc = data,
            Register::Vd => self.registers.vd = data,
            Register::Ve => self.registers.ve = data,
            Register::Vf => self.registers.vf = data,
            _ => panic!("Error no register of value {reg}"),
        }
    }

    fn fetch_instruction(&self) -> Option<Instruction> {
        match self.memory.get(self.ip) {
            Some(instr) => *instr,
            None => None,
        }
    }

    fn decode_instruction(&self) {
        todo!("Need to implement decode")
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.display.as_slice().chunks(8_usize) {
            for &pixel in line {
                let symbol = if pixel == 1 { '◼' } else { '◻' };
                write!(f, "{}", symbol)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

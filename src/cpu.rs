use std::ops::BitXor;

use crate::types::{Address, RegData, Register};
use fixedbitset::FixedBitSet;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::wasm_bindgen_test;

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
    i00E0,                              // Clears the display
    i1NNN(Address),                     // Jump to address NNN
    iANNN(Address),                     // Store memory address NNN in Register i
    iBNNN(Address),                     // Jump to adresss NNN + V0
    i6XNN(Register, RegData),           // store value NN at register X
    i7XNN(Register, RegData),           // Add data NN to register X
    iDXYN(Register, Register, RegData), // Draw at position (VX, VY) N bytes of sprite data starting at address stored in I
}

#[wasm_bindgen]
pub struct Cpu {
    memory: Vec<Instruction>, // 12 KB of memory, instructions starting at 0x200
    registers: Registers,
    clock: u128,
    display: FixedBitSet, // display fixed at 64 * 32 pixels
    ip: usize,            // instruction pointer
    height: usize,
    width: usize,
}

static INSTRUCTIONS_PER_SECOND: u16 = 700;

#[wasm_bindgen]
impl Cpu {
    pub fn new() -> Cpu {
        let height = 64;
        let width = 32;
        let size = height * width;
        let mut display = FixedBitSet::with_capacity(size);
        for i in 0..(64 * 32) {
            display.set(i, i % 2 == 0);
        }

        Cpu {
            memory: vec![],
            registers: Registers::new(),
            clock: 0,
            display,
            ip: 0,
            height,
            width,
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn tick(&mut self) {
        self.interpret();
        self.clock += 1;
    }

    fn get_index(&self, row: u8, col: u8) -> usize {
        (row * (self.width as u8) + col).into()
    }

    /// Main interpreter loop for fetching, decoding, executing instructions.
    /// This is invoked each "cycle" from the public tick function in the cpu impl.
    fn interpret(&mut self) {
        let mut instruction_count = 0;
        while let Some(instruction) = self.fetch_instruction() {
            // only run set instructions per tick of CPU
            if instruction_count >= INSTRUCTIONS_PER_SECOND {
                break;
            }
            instruction_count += 1;
            match instruction {
                Instruction::i00E0 => self.display.clear(),
                Instruction::i1NNN(address) => self.ip = address as usize,
                Instruction::iANNN(address) => self.registers.i = address,
                Instruction::iBNNN(address) => {
                    let reg_v0 = self.registers.v0;
                    self.ip = (address + (reg_v0 as u16)) as usize;
                }
                Instruction::i6XNN(reg, data) => self.store_at_register(reg, data),
                Instruction::iDXYN(reg_v0, reg_v1, data) => {
                    let base_addr = self.get_index(reg_v0 as u8, reg_v1 as u8);
                    for idx in 0..data {
                        self.display.set(base_addr + (idx as usize), true);
                    }
                }
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

    ///
    /// Fetch the instruction from memory at the CPU's instruction pointer.
    /// The fetch instruction also automatically increments the IP to point to the
    /// next instruction.
    fn fetch_instruction(&mut self) -> Option<Instruction> {
        let instr = self.memory.get(self.ip).copied();
        self.ip += 1;
        instr
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
        for row in 0..self.height {
            for col in 0..self.width {
                let index = row * self.width + col;
                if self.display[index] {
                    write!(f, "🦑")?;
                } else {
                    write!(f, "🐙")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[wasm_bindgen_test]
fn test_basic_display_commands() {
    let mut cpu = Cpu::new();
    cpu.memory = vec![
        Instruction::i00E0,
        Instruction::i6XNN(Register::V0, 0),
        Instruction::i6XNN(Register::V1, 0),
        Instruction::iDXYN(Register::V0, Register::V1, 1),
    ];

    cpu.tick();

    assert_eq!(cpu.display.count_ones(..), 1);
}

#[wasm_bindgen_test]
fn test_write_display() {
    let mut cpu = Cpu::new();
    cpu.memory = vec![
        Instruction::i00E0,
        Instruction::i6XNN(Register::V0, 0),
        Instruction::i6XNN(Register::V1, 0),
        Instruction::iDXYN(Register::V0, Register::V1, 5),
    ];

    cpu.tick();

    assert_eq!(cpu.display.count_ones(..), 5);
}

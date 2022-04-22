use crate::types::{Address, RegData};
use fixedbitset::FixedBitSet;

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
enum Instruction {
    i1NNN(Address), // Jump to address NNN
    iBNNN(Address), // Jump to adresss NNN + V0
    iANNN(Address), // Store memory address NNN in Register i
}

struct Cpu {
    memory: [Option<Instruction>; 4096], // 12 KB of memory, instructions starting at 0x200
    registers: Registers,
    clock: u128,
    display: FixedBitSet, // display fixed at 64 * 32 pixels
    ip: u16,              // instruction pointer
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            memory: [None; 4096],
            registers: Registers::new(),
            clock: 0,
            display: FixedBitSet::with_capacity(64 * 32),
            ip: 0,
        }
    }

    pub fn interpret(&self) {}
}

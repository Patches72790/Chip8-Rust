/// 16 General Purpose registers for use in arithmetic and
/// logical operations plus a memory addressing register `i`
/// for reading/writing memory.
struct Registers {
    v0: u8,
    v1: u8,
    v2: u8,
    v3: u8,
    v4: u8,
    v5: u8,
    v6: u8,
    v7: u8,
    v8: u8,
    v9: u8,
    va: u8,
    vb: u8,
    vc: u8,
    vd: u8,
    ve: u8,
    vf: u8,
    i: u16, // used for writing to/from memory
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
enum Instruction {
    i1NNN(u16), // Jump to address NNN
    iBNNN(u16), // Jump to adresss NNN + V0
    iANNN(u16), // Store memory address NNN in Register i
}

struct Cpu {
    memory: [Instruction; 4096], // 12 KB of memory, instructions starting at 0x200
    registers: Registers,
    clock: u128,
}

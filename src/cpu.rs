use crate::{
    types::{Address, RegData, Register},
    util::set_panic_hook,
};
use fixedbitset::FixedBitSet;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::wasm_bindgen_test;

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

#[wasm_bindgen]
pub struct Cpu {
    //memory: Vec<Instruction>, // 12 KB of memory, instructions starting at 0x200
    memory: [u8; 4096],
    registers: [RegData; 16],
    clock: u128,
    display: FixedBitSet, // display fixed at 64 * 32 pixels
    ip: usize,            // instruction pointer
    i: Address,           // special memory pointer I
    height: usize,
    width: usize,
}

/// Variable constant for determining cycles per second (hertz)
static INSTRUCTIONS_PER_SECOND: u16 = 1;

#[wasm_bindgen]
impl Cpu {
    pub fn new() -> Cpu {
        set_panic_hook();
        let height = 32;
        let width = 64;
        let size = height * width;
        let mut display = FixedBitSet::with_capacity(size);
        for i in 0..(64 * 32) {
            display.set(i, false); //i % 2 == 0);
        }

        Cpu {
            memory: Cpu::initialize_memory(),
            registers: [0u8; 16],
            i: 0,
            clock: 0,
            display,
            ip: 0x200, // Code section starts at 0x200 in memory
            height,
            width,
        }
    }

    /// Initialize memory with sprite fonts and
    /// any other possibilities. The font data is stored
    /// from 0x050 - 0x09F in memory before the code instructions
    /// which start at 0x200.
    fn initialize_memory() -> [u8; 4096] {
        let mut memory = [0u8; 4096];
        // 0
        memory[0x050] = 0xF0;
        memory[0x051] = 0x90;
        memory[0x052] = 0x90;
        memory[0x053] = 0x90;
        memory[0x054] = 0xF0;

        // 1
        memory[0x055] = 0x20;
        memory[0x056] = 0x60;
        memory[0x057] = 0x20;
        memory[0x058] = 0x20;
        memory[0x059] = 0x70;

        // 2
        memory[0x05a] = 0xF0;
        memory[0x05b] = 0x10;
        memory[0x05c] = 0xF0;
        memory[0x05d] = 0x80;
        memory[0x05e] = 0xF0;

        // todo 3..F
        memory
    }

    /// TODO! MUST CHANGE EVENTUALLY
    /// FOR NOW JUST USING FOR TESTING INSTRUCTIONS IN MEMORY
    pub fn load_instructions(&mut self) {
        let mut instructions = [0; 4096];
        instructions[0x200] = 0x00;
        instructions[0x201] = 0xE0;
        instructions[0x202] = 0x60;
        instructions[0x203] = 0x00;
        instructions[0x204] = 0x60;
        instructions[0x205] = 0x10;
        instructions[0x206] = 0xD0;
        instructions[0x207] = 0x11;
        instructions[0x200] = 0x00;
        instructions[0x201] = 0xE1;
        //instructions[0x202] = 0x00;
        //instructions[0x203] = 0xE0;

        self.memory = instructions;
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
                Instruction::i00E1 => {
                    self.display.set_range(.., true);
                }
                Instruction::i1NNN(address) => self.ip = address as usize,
                Instruction::iANNN(address) => self.i = address,
                Instruction::iBNNN(address) => {
                    let reg_v0 = self.registers[0];
                    self.ip = (address + (reg_v0 as u16)) as usize;
                }
                Instruction::i6XNN(reg, data) => self.store_at_register(reg, data),
                Instruction::iDXYN(reg_v0, reg_v1, num_rows) => {
                    // Draw sprites starting at pixel X, Y
                    // N bytes top - down starting with sprite data starting from reg I
                    //let base_addr = self.get_index(reg_v0 as u8, reg_v1 as u8);
                    //let base_sprite_addr = self.i;
                    //for idx in 0..data {
                    //    self.display.set(base_addr + (idx as usize), true);
                    //}
                    let starting_row = reg_v0 as u8;
                    let starting_col = reg_v1 as u8;
                }
                _ => panic!("Instruction not yet implemented"),
            }
        }
    }

    fn store_at_register(&mut self, reg: Register, data: RegData) {
        match reg {
            Register::V0 => self.registers[0] = data,
            Register::V1 => self.registers[1] = data,
            Register::V2 => self.registers[2] = data,
            Register::V3 => self.registers[3] = data,
            Register::V4 => self.registers[4] = data,
            Register::V5 => self.registers[5] = data,
            Register::V6 => self.registers[6] = data,
            Register::V7 => self.registers[7] = data,
            Register::V8 => self.registers[8] = data,
            Register::V9 => self.registers[9] = data,
            Register::Va => self.registers[0xa] = data,
            Register::Vb => self.registers[0xb] = data,
            Register::Vc => self.registers[0xc] = data,
            Register::Vd => self.registers[0xd] = data,
            Register::Ve => self.registers[0xe] = data,
            Register::Vf => self.registers[0xf] = data,
        }
    }

    /// Fetch the instruction from memory at the CPU's instruction pointer.
    /// The fetch instruction also automatically increments the IP to point to the
    /// next instruction.
    fn fetch_instruction(&mut self) -> Option<Instruction> {
        let instruction_byte_1 = self.memory.get(self.ip).copied();
        self.ip += 1;
        let instruction_byte_2 = self.memory.get(self.ip).copied();
        self.ip += 1;

        self.decode_instruction(instruction_byte_1, instruction_byte_2)
    }

    /// Decodes the two bytes of the fetched instruction from memory
    /// and produces an optional Instruction enum to be consumed
    /// by the CPU execute cycle.
    fn decode_instruction(&self, byte_1: Option<u8>, byte_2: Option<u8>) -> Option<Instruction> {
        let byte_1 = byte_1?;
        let byte_2 = byte_2?;

        // instructions are big-endian in memory
        let nibble_1: u16 = (((byte_1) & 0xF0) >> 4).into();
        let nibble_2: u16 = ((byte_1) & 0x0F).into();
        let nibble_3: u16 = (((byte_2) & 0xF0) >> 4).into();
        let nibble_4: u16 = ((byte_2) & 0x0F).into();

        //let whole_instruction_from_bytes = ((byte_1 as u16) << 8) | (byte_2 as u16);

        match (nibble_1, nibble_2, nibble_3, nibble_4) {
            (0x0, _, 0xE, 0x0) => Some(Instruction::i00E0),
            (0x0, _, 0xE, 0x1) => Some(Instruction::i00E1),
            (0x0, _, 0xE, 0xE) => todo!("Need to implement return from subroutine!"),
            (0x1, x, y, z) => {
                let reassembled_jump_address = (x << 8) | (y << 4) | z;
                Some(Instruction::i1NNN(reassembled_jump_address))
            }
            (0x6, x, n1, n2) => {
                let register = Register::from(x);
                let reg_data: u8 = ((n1 << 4) | n2)
                    .try_into()
                    .expect("Error casting u16 to u8 in decoder for i6XNN");
                Some(Instruction::i6XNN(register, reg_data))
            }
            (0x7, x, n1, n2) => {
                let register = Register::from(x);
                let reg_data: u8 = ((n1 << 4) | n2)
                    .try_into()
                    .expect("Error casting u16 to u8 in decoder for i6XNN");
                Some(Instruction::i7XNN(register, reg_data))
            }
            (0xA, x, y, z) => {
                let reassembled_jump_address = (x << 8) | (y << 4) | z;
                Some(Instruction::iANNN(reassembled_jump_address))
            }
            (0xB, x, y, z) => {
                let reassembled_jump_address = (x << 8) | (y << 4) | z;
                Some(Instruction::iBNNN(reassembled_jump_address))
            }
            (0xD, reg_1, reg_2, n) => {
                let register_1 = Register::from(reg_1);
                let register_2 = Register::from(reg_2);
                let data: u8 = n
                    .try_into()
                    .expect("Error casting u16 to u8 in decoder for iDXYN");
                Some(Instruction::iDXYN(register_1, register_2, data))
            }
            _ => panic!(
                "Unimplemented instruction [{},{},{},{}]\nIP: {}\nMemory: {:?}",
                nibble_1,
                nibble_2,
                nibble_3,
                nibble_4,
                self.ip,
                &self.memory[0x200..0x210],
            ),
        }
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
    //    cpu.memory = vec![
    //        Instruction::i00E0,
    //        Instruction::i6XNN(Register::V0, 0),
    //        Instruction::i6XNN(Register::V1, 0),
    //        Instruction::iDXYN(Register::V0, Register::V1, 1),
    //    ];
    let mut instructions = [0; 4096];
    instructions[0x200] = 0x00;
    instructions[0x201] = 0xE0;
    instructions[0x202] = 0x60;
    instructions[0x203] = 0x00;
    instructions[0x204] = 0x60;
    instructions[0x205] = 0x10;
    instructions[0x206] = 0xD0;
    instructions[0x207] = 0x11;

    cpu.memory = instructions;

    cpu.tick();

    assert_eq!(cpu.display.count_ones(..), 1);
}

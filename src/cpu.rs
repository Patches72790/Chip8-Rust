use crate::{
    instruction::Instruction,
    types::{Address, RegData, Register},
    types::{REG_V0, REG_VF},
    util::set_panic_hook,
    BITS_IN_BYTE, DEBUG_MODE, INSTRUCTIONS_PER_CYCLE,
};
use fixedbitset::FixedBitSet;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::{console_log, wasm_bindgen_test};

#[wasm_bindgen]
pub struct Cpu {
    // 12 KB of memory, instructions starting at 0x200
    memory: [u8; 4096],
    registers: [RegData; 16],
    stack: [Address; 16], // stack storing return address pointers for functions
    clock: u128,
    delay_timer: u8,
    sound_timer: u8,
    display: FixedBitSet, // display fixed at 64 * 32 pixels
    ip: usize,            // instruction pointer
    i: Address,           // special memory pointer I
    height: usize,
    width: usize,
    pixel_on: String,
    pixel_off: String,
}

#[wasm_bindgen]
impl Cpu {
    pub fn new() -> Cpu {
        set_panic_hook();
        let height = 32;
        let width = 64;
        let size = height * width;
        let mut display = FixedBitSet::with_capacity(size);
        for i in 0..(64 * 32) {
            display.set(i, false);
        }

        Cpu {
            memory: Cpu::initialize_memory(),
            registers: [0u8; 16],
            stack: [0u16; 16],
            delay_timer: 0,
            sound_timer: 0,
            i: 0,
            clock: 0,
            display,
            ip: 0x200, // Code section starts at 0x200 in memory
            height,
            width,
            pixel_on: "◽".to_string(),
            pixel_off: "◾".to_string(),
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn display(&self) -> *const u32 {
        self.display.as_slice().as_ptr()
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

        // 3
        memory[0x05f] = 0xF0;
        memory[0x060] = 0x10;
        memory[0x061] = 0xF0;
        memory[0x062] = 0x10;
        memory[0x063] = 0xF0;

        // 4
        memory[0x064] = 0x90;
        memory[0x065] = 0x90;
        memory[0x066] = 0xF0;
        memory[0x067] = 0x10;
        memory[0x068] = 0x10;

        // 5
        memory[0x069] = 0xF0;
        memory[0x06a] = 0x80;
        memory[0x06b] = 0xF0;
        memory[0x06c] = 0x10;
        memory[0x06d] = 0xf0;

        // 6
        memory[0x06e] = 0xF0;
        memory[0x06f] = 0x80;
        memory[0x070] = 0xF0;
        memory[0x071] = 0x90;
        memory[0x072] = 0xf0;

        // 7
        memory[0x073] = 0xF0;
        memory[0x074] = 0x10;
        memory[0x075] = 0x20;
        memory[0x076] = 0x40;
        memory[0x077] = 0x40;

        // 8
        memory[0x078] = 0xF0;
        memory[0x079] = 0x90;
        memory[0x07a] = 0xf0;
        memory[0x07b] = 0x90;
        memory[0x07c] = 0xf0;

        // 9
        memory[0x07d] = 0xF0;
        memory[0x07e] = 0x90;
        memory[0x07f] = 0xF0;
        memory[0x080] = 0x10;
        memory[0x081] = 0xF0;

        // A
        memory[0x082] = 0xF0;
        memory[0x083] = 0x90;
        memory[0x084] = 0xF0;
        memory[0x085] = 0x90;
        memory[0x086] = 0x90;

        // B
        memory[0x087] = 0xE0;
        memory[0x088] = 0x90;
        memory[0x089] = 0xE0;
        memory[0x08a] = 0x90;
        memory[0x08b] = 0xE0;

        // C
        memory[0x08c] = 0xF0;
        memory[0x08d] = 0x80;
        memory[0x08e] = 0x80;
        memory[0x08f] = 0x80;
        memory[0x090] = 0xF0;

        // D
        memory[0x091] = 0xE0;
        memory[0x092] = 0x90;
        memory[0x093] = 0x90;
        memory[0x094] = 0x90;
        memory[0x095] = 0xE0;

        // E
        memory[0x096] = 0xF0;
        memory[0x097] = 0x80;
        memory[0x098] = 0xF0;
        memory[0x099] = 0x80;
        memory[0x09a] = 0xF0;

        // F
        memory[0x09b] = 0xF0;
        memory[0x09c] = 0x80;
        memory[0x09d] = 0xF0;
        memory[0x09e] = 0x80;
        memory[0x09f] = 0x80;

        memory
    }

    /// TODO! MUST CHANGE EVENTUALLY
    /// FOR NOW JUST USING FOR TESTING INSTRUCTIONS IN MEMORY
    pub fn load_instructions(&mut self) {
        let mut instructions = self.memory;
        // draw sprite 0 at (0, 0)
        instructions[0x200] = 0xA0;
        instructions[0x201] = 0x50;
        instructions[0x202] = 0xD0;
        instructions[0x203] = 0x05;

        // draw sprite 1 at (0, 5)
        instructions[0x204] = 0xA0;
        instructions[0x205] = 0x55;
        instructions[0x206] = 0xD0;
        instructions[0x207] = 0x55;

        self.memory = instructions;
    }

    /// TODO! This doesn't work as it should right now.
    /// Setting properties of cpu at runtime violates borrowing
    /// rules in rust, so it is not allowed.
    /// How else could I setup instructions to be loaded at runtime
    /// without giving access to the global cpu in the JS code?
    pub fn load_instructions_from_file(&self, bytes_array: js_sys::Uint8Array) {
        let mut bytes_slice = vec![];
        bytes_array.copy_to(&mut bytes_slice);
        console_log!("Bytes array contents:\n{:?}", bytes_slice);
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    /// The main public API representing a singular cpu "cycle"
    /// This should be used each iteration of the main rendering loop.
    pub fn tick(&mut self) {
        self.interpret();
        self.clock += 1;
    }

    fn get_index(&self, row: usize, col: usize) -> usize {
        row * (self.width) + col
    }

    /// Disassembler utility for debugging the instructions in the
    /// front end code. The IP must be reset back to initial state
    /// as the code reuses `Cpu::fetch_instruction`.
    pub fn disassemble(&mut self) -> Vec<js_sys::JsString> {
        let mut instrs = vec![];
        while let Some(instruction) = self.fetch_instruction() {
            instrs.push(js_sys::JsString::from(instruction.to_string()));
        }

        self.ip = 0x200;
        instrs
    }

    /// Main interpreter loop for fetching, decoding, executing instructions.
    /// This is invoked each "cycle" from the public tick function in the cpu impl.
    fn interpret(&mut self) {
        let mut instruction_count = 0;
        while let Some(instruction) = self.fetch_instruction() {
            match instruction {
                Instruction::i00E0 => self.display.clear(),
                Instruction::i00E1 => {
                    self.display.set_range(.., true);
                }
                Instruction::i1NNN(address) => self.ip = address as usize,
                Instruction::i2NNN(address) => todo!("Haven't implemented function stack yet"),
                Instruction::i3XNN(reg, data) => {
                    let reg_value = self.get_from_register(reg);
                    // skip next instruction if regX equals data
                    if reg_value == data {
                        self.ip += 2;
                    }
                }
                Instruction::i4XNN(reg, data) => {
                    let reg_value = self.get_from_register(reg);
                    // skip next instruction if regX does NOT equal data
                    if reg_value != data {
                        self.ip += 2;
                    }
                }
                Instruction::i5XY0(reg_x, reg_y) => {
                    let reg_x_value = self.get_from_register(reg_x);
                    let reg_y_value = self.get_from_register(reg_y);
                    // skip next instruction if reg x ==  reg y
                    if reg_x_value == reg_y_value {
                        self.ip += 2;
                    }
                }
                Instruction::i6XNN(reg, data) => self.store_at_register(reg, data),
                Instruction::i7XNN(reg, data) => {
                    let reg_value = self.get_from_register(reg);
                    self.store_at_register(reg, reg_value.wrapping_add(data))
                }
                Instruction::iANNN(address) => self.i = address,
                Instruction::iBNNN(address) => {
                    let reg_v0 = self.registers[REG_V0];
                    self.ip = (address + (reg_v0 as u16)) as usize;
                }
                Instruction::iDXYN(reg_v0, reg_v1, num_rows) => {
                    // Draw sprites starting at pixel X, Y
                    // N bytes top -> down starting with sprite data at address in reg I
                    let mut current_row = (reg_v0 as u8) & ((self.height - 1) as u8);
                    let current_col = (reg_v1 as u8) & ((self.width - 1) as u8);
                    let base_sprite_addr: usize = self.i.into();

                    // set if a pixel is cleared from 1 to 0
                    let mut pixel_was_unset = false;

                    // this loop handles the sprite accesses from memory
                    for offset in 0..num_rows {
                        let sprite_byte = self.memory[base_sprite_addr + (offset as usize)];

                        // this loop loops through bits in byte of sprite
                        for bit in 0..BITS_IN_BYTE {
                            // TODO Need to check for wrapping around side of display
                            let index = self.get_index(
                                current_row.into(),
                                ((current_col + bit) & ((self.width - 1) as u8)).into(),
                            );
                            let current_pixel = self.display[index];

                            // First & is mask, then shift over to first bit position
                            let mask: u8 = 0x80 >> bit;
                            let shift_amt = 7 - bit;
                            let current_sprite_pixel_bit_is_set =
                                ((sprite_byte & mask) >> shift_amt) == 1;

                            // XOR display pixel with the sprite pixel
                            let new_pixel_value = current_pixel ^ current_sprite_pixel_bit_is_set;

                            if current_pixel && !new_pixel_value {
                                pixel_was_unset = true;
                            }

                            if DEBUG_MODE {
                                console_log!(
                                    "Setting ({},{})@index({}) to : {}",
                                    (current_row),
                                    (current_col + bit),
                                    index,
                                    new_pixel_value
                                );
                            }
                            self.display.set(index, new_pixel_value);
                        }
                        current_row = (current_row + 1) & ((self.height - 1) as u8);
                    }
                    // set VF to 0 unless any pixel is cleared
                    self.registers[REG_VF] = if pixel_was_unset { 1 } else { 0 };
                } //_ => panic!("Instruction not yet implemented"),
            }
            // only run set instructions per tick of CPU
            instruction_count += 1;
            if instruction_count >= INSTRUCTIONS_PER_CYCLE {
                break;
            }
        }
    }

    fn get_from_register(&self, reg: Register) -> RegData {
        let reg_idx: u16 = reg.into();
        self.registers[reg_idx as usize]
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

        match (nibble_1, nibble_2, nibble_3, nibble_4) {
            (0x0, _, 0xE, 0x0) => Some(Instruction::i00E0),
            (0x0, _, 0xE, 0x1) => Some(Instruction::i00E1),
            (0x0, _, 0xE, 0xE) => todo!("Need to implement return from subroutine!"),
            (0x1, x, y, z) => {
                let reassembled_jump_address = (x << 8) | (y << 4) | z;
                Some(Instruction::i1NNN(reassembled_jump_address))
            }
            (0x2, n1, n2, n3) => {
                let reassembled_jump_address = (n1 << 8) | (n2 << 4) | n3;
                Some(Instruction::i2NNN(reassembled_jump_address))
            }
            (0x3, x, n1, n2) => {
                let register = Register::from(x);
                let data = (n1 << 4) | n2;
                Some(Instruction::i3XNN(
                    register,
                    data.try_into().expect("Error casting u16 to u8"),
                ))
            }
            (0x4, x, n1, n2) => {
                let register = Register::from(x);
                let data = (n1 << 4) | n2;
                Some(Instruction::i4XNN(
                    register,
                    data.try_into().expect("Error casting u16 to u8"),
                ))
            }
            (0x5, x, y, _) => Some(Instruction::i5XY0(Register::from(x), Register::from(y))),
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
            _ => None,
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
                    //write!(f, "🦑")?;
                    write!(f, "{}", self.pixel_on)?;
                } else {
                    //write!(f, "🐙")?;
                    write!(f, "{}", self.pixel_off)?;
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

    assert_eq!(cpu.display.count_ones(..), 0);
}

#[wasm_bindgen_test]
fn test_draw_numbers() {
    let mut cpu = Cpu::new();
    cpu.load_instructions();
    cpu.tick();

    assert_eq!(cpu.display.count_ones(..), 22);
}

#[wasm_bindgen_test]
fn test_register_instructions() {
    let mut cpu = Cpu::new();
    let mut instructions = [0; 4096];

    instructions[0x200] = 0x00; // clear the screen
    instructions[0x201] = 0xE0;
    instructions[0x202] = 0x60; // store 255 in register 0
    instructions[0x203] = 0xFF;
    instructions[0x204] = 0x70; // add 1 to register 0
    instructions[0x205] = 0x01;
    instructions[0x206] = 0x30; // skip next instruction if reg 0 == 0
    instructions[0x207] = 0x00;
    instructions[0x208] = 0x70; // will be skipped -- adds 17 to register 0
    instructions[0x209] = 0x11;

    cpu.memory = instructions;
    cpu.tick();

    assert_eq!(cpu.registers[0], 0);
}

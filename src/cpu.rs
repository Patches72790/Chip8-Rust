use crate::{
    instruction::Instruction,
    keyboard::Keyboard,
    types::{Address, RegData, Register},
    types::{REG_V0, REG_VF},
    util::{make_instructions, set_panic_hook},
    BITS_IN_BYTE, DEBUG_MODE, INSTRUCTIONS_PER_CYCLE, KEY_0_ADDR, KEY_1_ADDR, KEY_2_ADDR,
    KEY_3_ADDR, KEY_4_ADDR, KEY_5_ADDR, KEY_6_ADDR, KEY_7_ADDR, KEY_8_ADDR, KEY_9_ADDR, KEY_A_ADDR,
    KEY_B_ADDR, KEY_C_ADDR, KEY_D_ADDR, KEY_E_ADDR, KEY_F_ADDR, STACK_MAX_SIZE,
};
use fixedbitset::FixedBitSet;
use js_sys::Math;
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
    sp: usize,            // stack pointer denoting current top of stack
    i: Address,           // special memory pointer I
    height: usize,
    width: usize,
    keyboard: Keyboard,
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

        let mut keyboard = Keyboard::new();
        keyboard.initialize_key_event_handlers();

        Cpu {
            memory: Cpu::initialize_memory(),
            registers: [0u8; 16],
            stack: [0u16; 16],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            i: 0,
            clock: 0,
            display,
            ip: 0x200, // Code section starts at 0x200 in memory
            height,
            width,
            keyboard,
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
        make_instructions!(
            instructions,
            0x200,
            vec![0x00E0, 0xA050, 0x6000, 0x6100, 0xD015, 0x7006, 0xA055, 0xD015]
        );
        self.memory = instructions;
    }

    /// Load instructions from a file input in the browser.
    /// The instructions are assumed to be in u8 chunks,
    /// so half of an instruction at each array index.
    pub fn load_instructions_from_file(&mut self, bytes_array: js_sys::Uint8Array) {
        let instructions_vec = bytes_array.to_vec();

        let mut new_memory = Cpu::initialize_memory();
        let base_addr = 0x200;
        for (i, instruction) in instructions_vec.iter().enumerate() {
            new_memory[base_addr + i] = *instruction;
        }

        self.memory = new_memory;
    }

    /// Returns the raw display data of the CPU as a Rust String
    pub fn render(&self) -> String {
        self.to_string()
    }

    /// The main public API representing a singular cpu "cycle"
    /// This should be used each iteration of the main rendering loop.
    pub fn tick(&mut self) {
        self.interpret();
        self.clock += 1;
    }

    fn decrement_delay_timer(&mut self) {
        self.delay_timer -= 60;
    }

    fn decrement_sound_timer(&mut self) {
        self.sound_timer -= 60;
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
        let mut instruction_count: u32 = 0;
        while let Some(instruction) = self.fetch_instruction() {
            match instruction {
                Instruction::i00E0 => self.display.clear(),
                Instruction::i00EE => {
                    // check for empty stack
                    if self.sp == 0 {
                        panic!("Error returning from subroutine with an empty stack");
                    }

                    let return_address = self.stack[self.sp];
                    self.sp -= 1;

                    // set instruction pointer to restored return addr
                    self.ip = return_address.into();
                }
                Instruction::i00E1 => {
                    self.display.set_range(.., true);
                }
                Instruction::i1NNN(address) => self.ip = address as usize,
                Instruction::i2NNN(address) => {
                    if self.sp == STACK_MAX_SIZE.into() {
                        panic!("Error attempting to push onto a full stack");
                    }

                    self.sp += 1;

                    // save ip of caller
                    self.stack[self.sp] = self.ip as u16;

                    // set new IP for callee function
                    self.ip = address as usize;
                }
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
                Instruction::i8XY0(reg1, reg2) => {
                    self.store_at_register(reg1, self.get_from_register(reg2))
                }
                Instruction::i8XY1(reg1, reg2) => {
                    let x_value = self.get_from_register(reg1);
                    let y_value = self.get_from_register(reg2);
                    self.store_at_register(reg1, x_value | y_value)
                }
                Instruction::i8XY2(reg1, reg2) => {
                    let x_value = self.get_from_register(reg1);
                    let y_value = self.get_from_register(reg2);
                    self.store_at_register(reg1, x_value & y_value)
                }
                Instruction::i8XY3(reg1, reg2) => {
                    let x_value = self.get_from_register(reg1);
                    let y_value = self.get_from_register(reg2);
                    self.store_at_register(reg1, x_value ^ y_value)
                }
                Instruction::i8XY4(reg1, reg2) => {
                    let x_value = self.get_from_register(reg1);
                    let y_value = self.get_from_register(reg2);

                    let (new_vx_val, did_overflow) = x_value.overflowing_add(y_value);
                    self.registers[REG_VF] = if did_overflow { 1 } else { 0 };
                    self.store_at_register(reg1, new_vx_val);
                }
                Instruction::i8XY5(reg1, reg2) => {
                    let x_value = self.get_from_register(reg1);
                    let y_value = self.get_from_register(reg2);

                    let (new_val, did_overflow) = x_value.overflowing_sub(y_value);
                    if did_overflow {
                        self.registers[0xf] = 1;
                    } else {
                        self.registers[0xf] = 0;
                    }
                    self.store_at_register(reg1, new_val)
                }
                Instruction::i8XY6(reg1, reg2) => {
                    let y_value = self.get_from_register(reg2);

                    let lsb = y_value & 0x01;
                    self.registers[0xf] = lsb;

                    self.store_at_register(reg1, y_value >> 1)
                }
                Instruction::i8XY7(reg1, reg2) => {
                    let x_value = self.get_from_register(reg1);
                    let y_value = self.get_from_register(reg2);

                    let (new_val, did_overflow) = y_value.overflowing_sub(x_value);
                    if did_overflow {
                        self.registers[0xf] = 0;
                    } else {
                        self.registers[0xf] = 1;
                    }
                    self.store_at_register(reg1, new_val)
                }
                Instruction::i8XYE(reg1, reg2) => {
                    let y_value = self.get_from_register(reg2);

                    let msb = (y_value & 0x80) >> 7;
                    self.registers[0xf] = msb;
                    self.store_at_register(reg1, y_value << 1)
                }
                Instruction::i9XY0(reg1, reg2) => {
                    let reg_1_val = self.get_from_register(reg1);
                    let reg_2_val = self.get_from_register(reg2);

                    if reg_1_val != reg_2_val {
                        self.ip += 2;
                    }
                }
                Instruction::iANNN(address) => self.i = address,
                Instruction::iBNNN(address) => {
                    let reg_v0 = self.registers[REG_V0];
                    self.ip = (address + (reg_v0 as u16)) as usize;
                }
                Instruction::iCXNN(reg, mask) => {
                    let rand = Math::floor(Math::random() * (u16::MAX as f64)) as u16;
                    self.store_at_register(reg, (rand & mask) as u8)
                }
                Instruction::iDXYN(reg_v0, reg_v1, num_rows) => {
                    // Draw sprites starting at pixel X, Y
                    // N bytes top -> down starting with sprite data at address in reg I
                    let x_coord = (self.get_from_register(reg_v0) as u8) & ((self.width - 1) as u8);
                    let mut y_coord =
                        (self.get_from_register(reg_v1) as u8) & ((self.height - 1) as u8);
                    let base_sprite_addr: usize = self.i.into();

                    // set if a pixel is cleared from 1 to 0
                    let mut pixel_was_unset = false;

                    // this loop handles the sprite accesses from memory
                    for offset in 0..num_rows {
                        let sprite_byte = self.memory[base_sprite_addr + (offset as usize)];

                        // this loop loops through bits in byte of sprite
                        for bit in 0..BITS_IN_BYTE {
                            // TODO Need to check for wrapping around side of display or stop
                            let index = self.get_index(y_coord.into(), (x_coord + bit).into());
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
                                    (x_coord),
                                    (y_coord),
                                    index,
                                    new_pixel_value
                                );
                            }
                            self.display.set(index, new_pixel_value);
                        }
                        // TODO Currently wraps around screen -- should this just stop if overflow?
                        y_coord = (y_coord + 1) & ((self.height - 1) as u8);
                    }
                    // set VF to 0 unless any pixel is cleared
                    self.registers[REG_VF] = if pixel_was_unset { 1 } else { 0 };
                }
                Instruction::iEX9E(reg) => {
                    let reg_val = self.get_from_register(reg);
                    // Mask the 4 least significant bits only (bits 0 - F)
                    let key_is_pressed = self.keyboard.get_key(reg_val & 0x0F);
                    // skip next instruction if key corresponding to register value is pressed
                    if key_is_pressed {
                        self.ip += 2;
                    }
                }
                Instruction::iEXA1(reg) => {
                    let reg_val = self.get_from_register(reg);
                    let key_is_pressed = self.keyboard.get_key(reg_val);
                    // skip next instruction if key corresponding to register value is not pressed
                    if !key_is_pressed {
                        self.ip += 2;
                    }
                }
                Instruction::iFX07(reg) => self.store_at_register(reg, self.delay_timer),
                Instruction::iFX0A(reg) => {
                    // wait for keypress and store result in reg VX
                    if let Some(key) = self.keyboard.get_registered_key() {
                        todo!("Need to set register VX to key that is registered");
                    } else {
                        //otherwise simulate wait -- decrement tick and counter
                        self.ip -= 2;
                        if let None = instruction_count.checked_sub(1) {
                            panic!("Error cannot reduce instruction count below 0 in FX0A")
                        }
                    }
                }
                Instruction::iFX15(reg) => {
                    self.delay_timer = self.get_from_register(reg);
                }
                Instruction::iFX18(reg) => {
                    self.sound_timer = self.get_from_register(reg);
                }
                Instruction::iFX1E(reg) => {
                    let reg_x_val = self.get_from_register(reg);

                    self.i += reg_x_val as u16;
                }
                Instruction::iFX29(reg) => {
                    let ls_nibble = self.get_from_register(reg) & 0x000F;
                    self.i = match ls_nibble {
                        0x0 => KEY_0_ADDR,
                        0x1 => KEY_1_ADDR,
                        0x2 => KEY_2_ADDR,
                        0x3 => KEY_3_ADDR,
                        0x4 => KEY_4_ADDR,
                        0x5 => KEY_5_ADDR,
                        0x6 => KEY_6_ADDR,
                        0x7 => KEY_7_ADDR,
                        0x8 => KEY_8_ADDR,
                        0x9 => KEY_9_ADDR,
                        0xa => KEY_A_ADDR,
                        0xb => KEY_B_ADDR,
                        0xc => KEY_C_ADDR,
                        0xd => KEY_D_ADDR,
                        0xe => KEY_E_ADDR,
                        0xf => KEY_F_ADDR,
                        _ => panic!("Cannot assign i to key greater than 16 (0xF)"),
                    }
                }
                Instruction::iFX33(reg) => todo!("TODO FX33"),
                Instruction::iFX55(reg) => todo!("TODO FX55"),
                Instruction::iFX65(reg) => todo!("TODO FX65"),
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
            (0x0, _, 0xE, 0xE) => Some(Instruction::i00EE),
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
            (0x8, x, y, 0) => {
                let register1 = Register::from(x);
                let register2 = Register::from(y);
                Some(Instruction::i8XY0(register1, register2))
            }
            (0x8, x, y, 1) => {
                let register1 = Register::from(x);
                let register2 = Register::from(y);
                Some(Instruction::i8XY0(register1, register2))
            }
            (0x8, x, y, 2) => {
                let register1 = Register::from(x);
                let register2 = Register::from(y);
                Some(Instruction::i8XY0(register1, register2))
            }
            (0x8, x, y, 3) => {
                let register1 = Register::from(x);
                let register2 = Register::from(y);
                Some(Instruction::i8XY0(register1, register2))
            }
            (0x8, x, y, 4) => {
                let register1 = Register::from(x);
                let register2 = Register::from(y);
                Some(Instruction::i8XY0(register1, register2))
            }
            (0x8, x, y, 5) => {
                let register1 = Register::from(x);
                let register2 = Register::from(y);
                Some(Instruction::i8XY0(register1, register2))
            }
            (0x8, x, y, 6) => {
                let register1 = Register::from(x);
                let register2 = Register::from(y);
                Some(Instruction::i8XY0(register1, register2))
            }
            (0x8, x, y, 7) => {
                let register1 = Register::from(x);
                let register2 = Register::from(y);
                Some(Instruction::i8XY0(register1, register2))
            }
            (0x8, x, y, 0xe) => {
                let register1 = Register::from(x);
                let register2 = Register::from(y);
                Some(Instruction::i8XY0(register1, register2))
            }
            (0x9, x, y, _) => Some(Instruction::i9XY0(Register::from(x), Register::from(y))),
            (0xA, x, y, z) => {
                let reassembled_jump_address = (x << 8) | (y << 4) | z;
                Some(Instruction::iANNN(reassembled_jump_address))
            }
            (0xB, x, y, z) => {
                let reassembled_jump_address = (x << 8) | (y << 4) | z;
                Some(Instruction::iBNNN(reassembled_jump_address))
            }
            (0xC, x, n1, n2) => Some(Instruction::iCXNN(Register::from(x), (n1 << 4) | n2)),
            (0xD, reg_1, reg_2, n) => {
                let register_1 = Register::from(reg_1);
                let register_2 = Register::from(reg_2);
                let data: u8 = n
                    .try_into()
                    .expect("Error casting u16 to u8 in decoder for iDXYN");
                Some(Instruction::iDXYN(register_1, register_2, data))
            }
            (0xE, x, 0x9, _) => Some(Instruction::iEX9E(Register::from(x))),
            (0xE, x, 0xa, _) => Some(Instruction::iEXA1(Register::from(x))),
            (0xF, reg, 0x0, 0x7) => Some(Instruction::iFX07(Register::from(reg))),
            (0xF, reg, 0x0, 0xa) => Some(Instruction::iFX0A(Register::from(reg))),
            (0xF, reg, 0x1, 0x5) => Some(Instruction::iFX15(Register::from(reg))),
            (0xF, reg, 0x1, 0x8) => Some(Instruction::iFX18(Register::from(reg))),
            (0xF, reg, 0x1, 0xe) => Some(Instruction::iFX1E(Register::from(reg))),
            (0xF, reg, 0x2, 0x9) => Some(Instruction::iFX29(Register::from(reg))),
            (0xF, reg, 0x3, 0x3) => Some(Instruction::iFX33(Register::from(reg))),
            (0xF, reg, 0x5, 0x5) => Some(Instruction::iFX55(Register::from(reg))),
            (0xF, reg, 0x6, 0x5) => Some(Instruction::iFX65(Register::from(reg))),
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
                    write!(f, "{}", self.pixel_on)?;
                } else {
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

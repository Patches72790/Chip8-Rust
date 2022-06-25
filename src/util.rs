extern crate console_error_panic_hook;

pub fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

/// Helper macro to set a 2 bytes instruction
/// at the memory address and address + 1
/// starting at address given
macro_rules! make_instruction {
    ($memory:ident, $addr:literal, $instr:literal) => {
        $memory[$addr] = (($instr & 0xFF00) >> 8) as u8;
        $memory[$addr + 1] = ($instr & 0x00FF) as u8;
    };
}

macro_rules! make_instructions {
    ($memory:ident, $addr:literal, $instrs:expr) => {
        for (i, instr) in $instrs.iter().enumerate() {
            $memory[$addr + (2 * i)] = ((instr & 0xFF00) >> 8) as u8;
            $memory[$addr + (2 * i) + 1] = (instr & 0x00FF) as u8;
        }
    };
}

pub(crate) use make_instruction;
pub(crate) use make_instructions;

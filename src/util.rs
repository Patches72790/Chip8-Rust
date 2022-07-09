extern crate console_error_panic_hook;
use wasm_bindgen_test::wasm_bindgen_test;

pub fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

pub fn hex2decimal(mut hex_number: u8) -> [u8; 3] {
    let mut decimal_array = [0u8; 3];

    for index in (0..=2).rev() {
        decimal_array[index as usize] = hex_number % 10;
        hex_number /= 10;
    }

    decimal_array
}

#[wasm_bindgen_test]
fn test_hex2decimal() {
    let num = 0x23;

    let actual = hex2decimal(num);
    assert_eq!(actual, [0, 3, 5]);
}

#[wasm_bindgen_test]
fn test_hex2decimal_2() {
    let num = 0xff;

    let actual = hex2decimal(num);
    assert_eq!(actual, [2, 5, 5]);
}

#[wasm_bindgen_test]
fn test_hex2decimal_3() {
    let num = 0x00;

    let actual = hex2decimal(num);
    assert_eq!(actual, [0, 0, 0]);
}

#[wasm_bindgen_test]
fn test_hex2decimal_4() {
    let num = 0x01;

    let actual = hex2decimal(num);
    assert_eq!(actual, [0, 0, 1]);
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

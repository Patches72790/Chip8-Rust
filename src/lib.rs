mod cpu;
mod instruction;
mod types;
mod ui;
mod util;

use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Constant used to avoid magic numbers
pub static BITS_IN_BYTE: u8 = 8;

/// Variable constant for determining cycles per second (hertz)
pub static INSTRUCTIONS_PER_CYCLE: u16 = 10;

pub static DEBUG_MODE: bool= true;

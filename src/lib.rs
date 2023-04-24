mod cpu;
mod instruction;
mod keyboard;
mod rom;
mod types;
mod ui;
mod util;

use types::Address;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Constant used to avoid magic numbers
pub static BITS_IN_BYTE: u8 = 8;

/// Variable constant for determining cycles per second (hertz)
pub static INSTRUCTIONS_PER_CYCLE: u32 = 4;

pub static DEBUG_MODE: bool = true;

pub const STACK_MAX_SIZE: u8 = 16;

pub const KEY_0_ADDR: Address = 0x050;
pub const KEY_1_ADDR: Address = 0x055;
pub const KEY_2_ADDR: Address = 0x05a;
pub const KEY_3_ADDR: Address = 0x05f;
pub const KEY_4_ADDR: Address = 0x064;
pub const KEY_5_ADDR: Address = 0x069;
pub const KEY_6_ADDR: Address = 0x06e;
pub const KEY_7_ADDR: Address = 0x073;
pub const KEY_8_ADDR: Address = 0x078;
pub const KEY_9_ADDR: Address = 0x07d;
pub const KEY_A_ADDR: Address = 0x082;
pub const KEY_B_ADDR: Address = 0x087;
pub const KEY_C_ADDR: Address = 0x08c;
pub const KEY_D_ADDR: Address = 0x091;
pub const KEY_E_ADDR: Address = 0x096;
pub const KEY_F_ADDR: Address = 0x09b;

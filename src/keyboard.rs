use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_test::console_log;
use web_sys::KeyboardEvent;

use crate::DEBUG_MODE;

static mut KEYS: Keys = Keys::new();
/// Represents the key that was just pressed and released
/// Will be Some on release, but is set to None when another
/// key is pressed
static mut REGISTERED_KEY: Option<u8> = None;

///
/// Traditional Chip-8 keyboard uses these mappings for
/// keys 0-F.
///   ╔═══╦═══╦═══╦═══╗
///   ║ 1 ║ 2 ║ 3 ║ C ║
///   ╠═══╬═══╬═══╬═══╣
///   ║ 4 ║ 5 ║ 6 ║ D ║
///   ╠═══╬═══╬═══╬═══╣
///   ║ 7 ║ 8 ║ 9 ║ E ║
///   ╠═══╬═══╬═══╬═══╣
///   ║ A ║ 0 ║ B ║ F ║
///   ╚═══╩═══╩═══╩═══╝
///
/// My keyboard implementation uses the QWERTY keyboard equivalent mapping:
///  
///   ╔═══╦═══╦═══╦═══╗
///   ║ 1 ║ 2 ║ 3 ║ 4 ║
///   ╠═══╬═══╬═══╬═══╣
///   ║ Q ║ W ║ E ║ R ║
///   ╠═══╬═══╬═══╬═══╣
///   ║ A ║ S ║ D ║ F ║
///   ╠═══╬═══╬═══╬═══╣
///   ║ Z ║ X ║ C ║ V ║
///   ╚═══╩═══╩═══╩═══╝
///
#[wasm_bindgen]
#[derive(Debug)]
pub struct Keyboard {
    // references must be kept in struct
    // to prevent JS closures being deallocated
    keydown_handler: Closure<dyn FnMut(KeyboardEvent)>,
    keyup_handler: Closure<dyn FnMut(KeyboardEvent)>,
}

impl Keyboard {
    pub fn new() -> Self {
        let keydown_handler = Closure::wrap(Box::new(|_| {}) as Box<dyn FnMut(KeyboardEvent)>);
        let keyup_handler = Closure::wrap(Box::new(|_| {}) as Box<dyn FnMut(KeyboardEvent)>);
        Keyboard {
            keydown_handler,
            keyup_handler,
        }
    }

    pub fn initialize_key_event_handlers(&mut self) {
        let onkeydown_closure = Closure::wrap(Box::new(|event: KeyboardEvent| {
            if DEBUG_MODE {
                console_log!("keydown event: {}", event.code());
            }
            unsafe {
                match event.key_code() {
                    KEY0 => KEYS.set_key(0),
                    KEY1 => KEYS.set_key(1),
                    KEY2 => KEYS.set_key(2),
                    KEY3 => KEYS.set_key(3),
                    KEY4 => KEYS.set_key(4),
                    KEY5 => KEYS.set_key(5),
                    KEY6 => KEYS.set_key(6),
                    KEY7 => KEYS.set_key(7),
                    KEY8 => KEYS.set_key(8),
                    KEY9 => KEYS.set_key(9),
                    KEYA => KEYS.set_key(10),
                    KEYB => KEYS.set_key(11),
                    KEYC => KEYS.set_key(12),
                    KEYD => KEYS.set_key(13),
                    KEYE => KEYS.set_key(14),
                    KEYF => KEYS.set_key(15),
                    _ => {}
                }
            }
        }) as Box<dyn FnMut(KeyboardEvent)>);

        let onkeyup_closure = Closure::wrap(Box::new(|event: KeyboardEvent| {
            if DEBUG_MODE {
                console_log!("keyup event: {}", event.code());
            }
            unsafe {
                match event.key_code() {
                    KEY0 => KEYS.clear_key(0),
                    KEY1 => KEYS.clear_key(1),
                    KEY2 => KEYS.clear_key(2),
                    KEY3 => KEYS.clear_key(3),
                    KEY4 => KEYS.clear_key(4),
                    KEY5 => KEYS.clear_key(5),
                    KEY6 => KEYS.clear_key(6),
                    KEY7 => KEYS.clear_key(7),
                    KEY8 => KEYS.clear_key(8),
                    KEY9 => KEYS.clear_key(9),
                    KEYA => KEYS.clear_key(10),
                    KEYB => KEYS.clear_key(11),
                    KEYC => KEYS.clear_key(12),
                    KEYD => KEYS.clear_key(13),
                    KEYE => KEYS.clear_key(14),
                    KEYF => KEYS.clear_key(15),
                    _ => {}
                }
            }
        }) as Box<dyn FnMut(KeyboardEvent)>);

        web_sys::window()
            .expect("Error getting window element when initializing keydown events")
            .set_onkeydown(Some(onkeydown_closure.as_ref().unchecked_ref()));

        web_sys::window()
            .expect("Error getting window element when initializing keydown events")
            .set_onkeyup(Some(onkeyup_closure.as_ref().unchecked_ref()));

        self.keydown_handler = onkeydown_closure;
        self.keyup_handler = onkeyup_closure;
    }

    /// TODO -- need to find way to map keys of PC keyboard to the Chip8 keys
    /// stored in memory
    pub fn get_key(&self, key: u8) -> bool {
        if key > 0xf {
            false
        } else {
            unsafe { KEYS.get_key(key.into()) }
        }
    }

    pub fn get_registered_key(&self) -> Option<u8> {
        unsafe { REGISTERED_KEY }
    }
}

impl Default for Keyboard {
    fn default() -> Self {
        Keyboard {
            keydown_handler: Closure::wrap(Box::new(|_| {}) as Box<dyn FnMut(KeyboardEvent)>),
            keyup_handler: Closure::wrap(Box::new(|_| {}) as Box<dyn FnMut(KeyboardEvent)>),
        }
    }
}

// See doc comment on Keyboard for mappings in PC keyboard
const KEY1: u32 = 0x31;
const KEY2: u32 = 0x32;
const KEY3: u32 = 0x33;
const KEY4: u32 = 0x34; // 4 == C
const KEY5: u32 = 0x35;
const KEY6: u32 = 0x36;
const KEY7: u32 = 0x37;
const KEY8: u32 = 0x38;
const KEY9: u32 = 0x39;
const KEY0: u32 = 0x30;
const KEYA: u32 = 0x41;
const KEYB: u32 = 0x42;
const KEYC: u32 = 0x43;
const KEYD: u32 = 0x44;
const KEYE: u32 = 0x45;
const KEYF: u32 = 0x46;

/// TODO -- Come up with a better way to represent the 16 Keys
/// that would support mappings to different keys on the keyboard
/// Use the default key names for Chip 8, but represent them with
/// default mappings for a QWERTY keyboard?
#[derive(Debug)]
pub enum Key {
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    KeyA,
    KeyB,
    KeyC,
    KeyD,
    KeyE,
    KeyF,
}

#[derive(Debug, Default)]
#[wasm_bindgen]
pub struct Keys {
    _keys: [bool; 16],
}

impl Keys {
    pub const fn new() -> Self {
        Keys { _keys: [false; 16] }
    }

    pub fn set_key(&mut self, key: usize) {
        unsafe {
            REGISTERED_KEY = None;
        }
        self._keys[key] = true;
    }

    pub fn clear_key(&mut self, key: usize) {
        unsafe {
            REGISTERED_KEY = Some(key as u8);
        }
        self._keys[key] = false;
    }

    pub fn get_key(&mut self, key: usize) -> bool {
        self._keys[key]
    }

    pub fn any_key_pressed(&self) -> bool {
        self._keys.iter().any(|e| *e)
    }
}

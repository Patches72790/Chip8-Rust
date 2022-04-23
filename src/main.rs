use wasm_bindgen::prelude::*;
use web_sys::HtmlInputElement;

#[wasm_bindgen]
pub fn load_file(file_input: HtmlInputElement) {
    let files = file_input
        .files()
        .expect("Could not read file list from html input element");
}

fn main() {
    println!("Hello, world!");
}

extern crate console_error_panic_hook;
use wasm_bindgen::prelude::*;

use wasm_bindgen_test::console_log;
use web_sys::HtmlInputElement;

pub fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub async fn load_file(file_input: HtmlInputElement) -> Result<String, JsValue> {
    let files = file_input
        .files()
        .expect("Could not read file list from html input element");

    let file = files.get(0).unwrap_throw();

    let file_text = file.text();
    let result = wasm_bindgen_futures::JsFuture::from(file_text).await?;
    let rust_string = result
        .as_string()
        .expect("Error getting rust string from file text");

    console_log!("File text: {}", rust_string);
    Ok(rust_string)
}

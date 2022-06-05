extern crate console_error_panic_hook;
use base64;
use js_sys::{ArrayBuffer, JsString, Uint8Array};
use wasm_bindgen::prelude::*;

use wasm_bindgen_test::console_log;
use web_sys::HtmlInputElement;

pub fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

//#[wasm_bindgen]
//pub async fn load_file(file_input: ArrayBuffer) -> Result<Uint8Array, JsValue> {
//    //    let files = file_input
//    //        .files()
//    //        .expect("Could not read file list from html input element");
//    //    let file = files.get(0).unwrap_throw();
//    //
//    //    let file_text = file.text();
//    //    let result = wasm_bindgen_futures::JsFuture::from(file_text).await?;
//    //
//    //    let rust_string = result
//    //        .as_string()
//    //        .expect("Error getting rust string from file text");
//    //
//    //    console_log!("{:?}", rust_string.as_bytes());
//    //
//    //    match base64::decode(rust_string.clone()) {
//    //        Ok(v) => console_log!("Decoded hex: {:?}", v),
//    //        Err(v) => console_log!("Error decoding rust string {}", v),
//    //    };
//    //
//    //    Ok(rust_string)
//
//    let js_val = JsValue::from(file_input);
//    let array = Uint8Array::new(&js_val);
//    let bytes = array.to_vec();
//
//    console_log!("{:?}", bytes);
//    Ok(bytes)
//}

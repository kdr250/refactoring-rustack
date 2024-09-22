pub mod mandel;
mod utils;

use std::io::Cursor;

use refactoring_rustack::{Parser, VirtualMachine};
use wasm_bindgen::prelude::*;
use web_sys::js_sys::Uint8Array;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn set_panic_hook() {
    utils::set_panic_hook();
}

#[wasm_bindgen]
pub fn evaluate(code: &str) -> String {
    let mut virtual_machine = VirtualMachine::new();
    let mut parser = Parser::new();

    for line in code.lines() {
        for element in parser.parse(line.to_string()) {
            virtual_machine.evaluate(element);
        }
    }

    virtual_machine
        .outputs()
        .into_iter()
        .map(|o| format!("puts: {o}\n"))
        .collect()
}

#[wasm_bindgen]
pub fn image_mandelbrot() -> Uint8Array {
    // マンデルブロ集合
    let buffer = mandel::mandelbrot();

    let mut result: Cursor<Vec<u8>> = Cursor::new(Vec::new());
    buffer
        .write_to(&mut result, image::ImageFormat::Png)
        .expect("Error occurs when writing to buffer");

    Uint8Array::new(&unsafe { Uint8Array::view(&result.into_inner()) }.into())
}

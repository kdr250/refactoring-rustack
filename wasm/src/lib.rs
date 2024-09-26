mod utils;

use std::io::Cursor;

use image::GrayImage;
use refactoring_rustack::{Parser, VirtualMachine};
use wasm_bindgen::prelude::*;
use web_sys::js_sys::Uint8Array;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

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
        .map(|out| format!("puts: {out}\n"))
        .collect()
}

#[wasm_bindgen]
pub fn evaluate_image(code: &str) -> Uint8Array {
    let mut virtual_machine = VirtualMachine::new();
    let mut parser = Parser::new();

    for line in code.lines() {
        for element in parser.parse(line.to_string()) {
            virtual_machine.evaluate(element);
        }
    }

    let (width_and_height, pixels) = virtual_machine.outputs().split_at(2);

    let width = width_and_height[0] as u32;
    let height = width_and_height[1] as u32;
    let pixels: Vec<_> = pixels.into_iter().map(|&i| i as u8).collect();

    let image = GrayImage::from_vec(width, height, pixels).expect("failed to convert pixels");

    let mut result: Cursor<Vec<u8>> = Cursor::new(Vec::new());
    image
        .write_to(&mut result, image::ImageFormat::Png)
        .expect("Error occurs when writing to buffer");

    Uint8Array::new(&unsafe { Uint8Array::view(&result.into_inner()) }.into())
}

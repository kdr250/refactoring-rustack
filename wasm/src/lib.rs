mod utils;

use refactoring_rustack::{Parser, VirtualMachine};
use wasm_bindgen::prelude::*;

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

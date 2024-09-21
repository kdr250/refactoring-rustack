mod utils;

use refactoring_rustack::{Parser, VirtualMachine};
use wasm_bindgen::prelude::*;
use web_sys::console::log_1;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

// Export a `greet` function from Rust to JavaScript, that alerts a
// hello message.
#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen(start)]
pub fn evaluate() {
    let mut virtual_machine = VirtualMachine::new();
    let mut parser = Parser::new();

    let lines = r#"
/x 10 def
/y 20 def
{ x y < } { x } { y } if
"#;

    for line in lines.lines() {
        for element in parser.parse(line.to_string()) {
            virtual_machine.evaluate(element);
        }
    }

    let log_str = format!("{:?}", virtual_machine.stack().list());

    log_1(&JsValue::from_str(log_str.as_str())); // [Number(10)]
}

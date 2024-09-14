use super::{stack::Stack, Element};

/// スタックベース仮想マシン
#[derive(Debug)]
pub struct VirtualMachine {
    stack: Stack,
}

impl VirtualMachine {
    /// スタックベース仮想マシンを生成する
    pub fn new() -> Self {
        Self {
            stack: Stack::new(),
        }
    }

    /// パースした要素を入れる
    pub fn push(&mut self, element: Element) {
        match element {
            Element::Number(num) => self.stack.push(num),
            Element::Operation(operation) => self.stack.execute(operation),
        }
    }
}

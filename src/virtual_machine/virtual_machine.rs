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

    ///　要素を処理する
    pub fn process(&mut self, element: Element) {
        self.stack.process(element);
    }
}

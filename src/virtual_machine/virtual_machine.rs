use super::super::element::Element;
use super::stack::Stack;

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

    ///　要素を評価する
    pub fn evaluate(&mut self, element: Element) {
        self.stack.evaluate(element);
    }

    /// スタックを返す
    pub fn stack(&self) -> &Stack {
        &self.stack
    }
}

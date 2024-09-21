use super::super::element::Element;
use super::stack::Stack;

/// 仮想マシン
#[derive(Debug)]
pub struct VirtualMachine {
    stack: Stack,
}

impl VirtualMachine {
    /// 仮想マシンを生成する
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

    /// 出力
    pub fn output(&self) -> Vec<String> {
        self.stack.list().iter().map(|e| e.to_string()).collect()
    }
}

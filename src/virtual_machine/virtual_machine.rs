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

    /// 出力を返す
    pub fn outputs(&self) -> &Vec<f32> {
        self.stack.outputs()
    }

    /// 出力をプリントする
    pub fn print_outputs(&self) {
        self.outputs().iter().for_each(|out| println!("{out}"));
    }
}

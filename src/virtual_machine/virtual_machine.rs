use super::stack::Stack;

/// スタックベース仮想マシン
#[derive(Debug)]
pub struct VirtualMachine {
    stack: Stack,
}

impl VirtualMachine {
    /// スタックベース仮想マシンを生成する
    pub fn new() -> Self {
        let mut stack = Stack::new();

        // FIXME
        stack.push(42);
        stack.push(36);
        stack.push(22);

        Self { stack }
    }

    /// 加算を行う
    pub fn add(&mut self) {
        self.stack.add();
    }
}

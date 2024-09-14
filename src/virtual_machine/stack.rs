/// スタック
#[derive(Debug)]
pub struct Stack {
    list: Vec<i32>,
}

impl Stack {
    /// スタックを生成する
    pub fn new() -> Self {
        Self { list: vec![] }
    }

    /// スタックに要素を入れる
    pub fn push(&mut self, element: i32) {
        self.list.push(element);
    }

    /// 加算を行う
    pub fn add(&mut self) {
        let rhs = self.list.pop().unwrap();
        let lhs = self.list.pop().unwrap();
        self.list.push(lhs + rhs);
    }
}

#[cfg(test)]
mod tests {
    use super::Stack;

    #[test]
    fn test_add() {
        let mut stack = Stack::new();
        stack.push(45);
        stack.push(55);

        stack.add();

        assert_eq!(stack.list[0], 100);
    }
}

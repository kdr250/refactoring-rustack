use super::{parser::Operation, Element};

/// スタック
#[derive(Debug)]
pub struct Stack {
    list: Vec<Element>,
}

impl Stack {
    /// スタックを生成する
    pub fn new() -> Self {
        Self { list: vec![] }
    }

    ///　要素を処理する
    pub fn process(&mut self, element: Element) {
        match element {
            Element::Number(_) | Element::Block(_) => self.push(element),
            Element::Operation(operation) => self.execute(operation),
        }
    }

    ///　要素を複数処理する
    fn process_multiple(&mut self, elements: Vec<Element>) {
        for element in elements {
            self.process(element);
        }
    }

    /// スタックに要素を入れる
    fn push(&mut self, element: Element) {
        self.list.push(element);
    }

    /// 演算を実行する
    fn execute(&mut self, operation: Operation) {
        match operation {
            Operation::Add => self.add(),
            Operation::Subtract => self.subtract(),
            Operation::Multiply => self.multiply(),
            Operation::Divide => self.divide(),
            Operation::If => self.operation_if(),
        }
    }

    /// 加算を行う
    pub fn add(&mut self) {
        let rhs = self.list.pop().unwrap().as_number();
        let lhs = self.list.pop().unwrap().as_number();
        self.list.push(Element::Number(lhs + rhs));
    }

    /// 減算を行う
    fn subtract(&mut self) {
        let rhs = self.list.pop().unwrap().as_number();
        let lhs = self.list.pop().unwrap().as_number();
        self.list.push(Element::Number(lhs - rhs));
    }

    /// 乗算を行う
    fn multiply(&mut self) {
        let rhs = self.list.pop().unwrap().as_number();
        let lhs = self.list.pop().unwrap().as_number();
        self.list.push(Element::Number(lhs * rhs));
    }

    /// 除算を行う
    fn divide(&mut self) {
        let rhs = self.list.pop().unwrap().as_number();
        let lhs = self.list.pop().unwrap().as_number();
        self.list.push(Element::Number(lhs / rhs));
    }

    /// 条件分岐を行う
    fn operation_if(&mut self) {
        let false_branch = self.list.pop().unwrap().to_block_vec();
        let true_branch = self.list.pop().unwrap().to_block_vec();
        let condition = self.list.pop().unwrap().to_block_vec();

        self.process_multiple(condition);

        let condition_result = self.list.pop().unwrap().as_number();

        match condition_result {
            0 => self.process_multiple(false_branch),
            _ => self.process_multiple(true_branch),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::parser::{tests::helper_create_block, Element, Parser};
    use super::Stack;

    fn parse(parser: &mut Parser) -> Stack {
        let mut stack = Stack::new();
        while let Some(element) = parser.next() {
            stack.process(element);
        }
        stack
    }

    #[test]
    fn test_add() {
        let mut stack = Stack::new();
        stack.push(Element::Number(45));
        stack.push(Element::Number(55));

        stack.add();

        assert_eq!(stack.list[0], Element::Number(100));
    }

    #[test]
    fn test_process() {
        let mut parser = Parser::new("1 2 + { 3 4 }");
        let stack = parse(&mut parser);

        assert_eq!(
            stack.list,
            vec![
                Element::Number(3),
                Element::Block(helper_create_block(vec![
                    Element::Number(3),
                    Element::Number(4)
                ]))
            ]
        )
    }

    #[test]
    fn test_if_true() {
        let mut parser = Parser::new("{ 1 -1 + } { 100 } { -100 } if");
        let stack = parse(&mut parser);

        assert_eq!(stack.list, vec![Element::Number(-100)])
    }

    #[test]
    fn test_if_false() {
        let mut parser = Parser::new("{ 1 1 + } { 100 } { -100 } if");
        let stack = parse(&mut parser);

        assert_eq!(stack.list, vec![Element::Number(100)])
    }
}

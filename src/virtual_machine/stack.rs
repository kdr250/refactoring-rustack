use std::collections::HashMap;

use super::{stack_helper::impl_operation, Element, NativeOperation};

/// スタック
#[derive(Debug)]
pub struct Stack {
    list: Vec<Element>,
    variables: HashMap<String, Element>,
}

impl Stack {
    /// スタックを生成する
    pub fn new() -> Self {
        let functions: [(&str, fn(&mut Stack)); 12] = [
            ("+", Stack::add),
            ("-", Stack::subtract),
            ("*", Stack::multiply),
            ("/", Stack::divide),
            ("<", Stack::less_than),
            ("if", Stack::operation_if),
            ("def", Stack::operation_define),
            ("puts", Stack::puts),
            ("pop", Stack::pop),
            ("dup", Stack::duplicate),
            ("exch", Stack::exchange),
            ("index", Stack::index),
        ];
        Self {
            list: vec![],
            variables: functions
                .into_iter()
                .map(|(name, function)| {
                    (
                        name.to_string(),
                        Element::NativeOperation(NativeOperation(function)),
                    )
                })
                .collect(),
        }
    }

    pub fn list(&self) -> &Vec<Element> {
        &self.list
    }

    ///　要素を処理する
    pub fn process(&mut self, element: Element) {
        match element {
            Element::Number(_) | Element::Block(_) | Element::Symbol(_) => self.push(element),
            Element::Operation(operation) => self.execute(operation),
            _ => panic!("Invalid element type"),
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
    fn execute(&mut self, operation: String) {
        let element = self
            .variables
            .get(&operation)
            .expect(&format!("{operation:?} is undefined"))
            .clone();

        match element {
            Element::Block(block) => {
                for inner_element in block.to_vec() {
                    self.process(inner_element);
                }
            }
            Element::NativeOperation(operation) => (operation.0)(self),
            _ => self.list.push(element),
        }
    }

    // 加算を行う
    impl_operation!(add, +);

    // 減算を行う
    impl_operation!(subtract, -);

    // 乗算を行う
    impl_operation!(multiply, *);

    // 除算を行う
    impl_operation!(divide, /);

    // 小なり大小比較を行う
    impl_operation!(less_than, <);

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

    /// 変数定義を行う
    fn operation_define(&mut self) {
        let element = self.list.pop().unwrap();
        self.process(element);
        let element = self.list.pop().unwrap();
        let symbol = self.list.pop().unwrap().as_symbol();

        self.variables.insert(symbol, element);
    }

    /// スタックの先頭を取り出して表示する
    fn puts(&mut self) {
        let element = self.list.pop().unwrap();
        println!("{}", element.to_string());
    }

    /// スタックの先頭を取り出す
    fn pop(&mut self) {
        self.list.pop().unwrap();
    }

    /// スタックの先頭を複製する
    fn duplicate(&mut self) {
        let element = self.list.last().unwrap();
        self.list.push(element.clone());
    }

    /// スタックの先頭と先頭から2番目を交換する
    fn exchange(&mut self) {
        let last = self.list.pop().unwrap();
        let second = self.list.pop().unwrap();
        self.list.push(last);
        self.list.push(second);
    }

    /// インデックス
    fn index(&mut self) {
        let index = self.list.pop().unwrap().as_number() as usize;
        let element = self.list[self.list.len() - index - 1].clone();
        self.list.push(element);
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
        let mut parser = Parser::new();
        parser.parse(String::from("1 2 + { 3 4 }"));
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
        let mut parser = Parser::new();
        parser.parse(String::from("{ 1 -1 + } { 100 } { -100 } if"));
        let stack = parse(&mut parser);

        assert_eq!(stack.list, vec![Element::Number(-100)])
    }

    #[test]
    fn test_if_false() {
        let mut parser = Parser::new();
        parser.parse(String::from("{ 1 1 + } { 100 } { -100 } if"));
        let stack = parse(&mut parser);

        assert_eq!(stack.list, vec![Element::Number(100)])
    }

    #[test]
    fn test_var() {
        let mut parser = Parser::new();
        parser.parse(String::from("/x 10 def /y 20 def x y *"));
        let stack = parse(&mut parser);

        assert_eq!(stack.list, vec![Element::Number(200)]);
    }

    #[test]
    fn test_var_if() {
        let mut parser = Parser::new();
        parser.parse(String::from("/x 10 def /y 20 def { x y < } { x } { y } if"));
        let stack = parse(&mut parser);

        assert_eq!(stack.list, vec![Element::Number(10)]);
    }

    #[test]
    fn test_multiline() {
        let mut stack = Stack::new();
        let mut parser = Parser::new();
        let lines = r#"
/x 10 def
/y 20 def

{ x y < }
{ x }
{ y }
if
"#;
        for line in lines.lines() {
            parser.parse(line.to_string());
            while let Some(element) = parser.next() {
                stack.process(element);
            }
        }

        assert_eq!(stack.list, vec![Element::Number(10)]);
    }

    #[test]
    fn test_function() {
        let mut stack = Stack::new();
        let mut parser = Parser::new();
        let lines = r#"
/double { 2 * } def
10 double
"#;

        for line in lines.lines() {
            parser.parse(line.to_string());
            while let Some(element) = parser.next() {
                stack.process(element);
            }
        }

        assert_eq!(stack.list, vec![Element::Number(20)]);
    }
}

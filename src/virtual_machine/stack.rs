use std::collections::HashMap;

use crate::element::{Element, NativeOperation};

use super::stack_helper::impl_operation;

/// スタック
#[derive(Debug)]
pub struct Stack {
    list: Vec<Element>,
    variables: Vec<HashMap<String, Element>>,
    outputs: Vec<i32>,
}

impl Stack {
    /// スタックを生成する
    pub fn new() -> Self {
        let functions: [(&str, fn(&mut Stack)); 14] = [
            ("+", Stack::add),
            ("-", Stack::subtract),
            ("*", Stack::multiply),
            ("/", Stack::divide),
            ("<", Stack::less_than),
            ("if", Stack::operate_if),
            ("def", Stack::operate_define),
            ("for", Stack::operate_for),
            ("while", Stack::operate_while),
            ("puts", Stack::puts),
            ("pop", Stack::pop),
            ("dup", Stack::duplicate),
            ("exch", Stack::exchange),
            ("index", Stack::index),
        ];
        Self {
            list: vec![],
            variables: vec![functions
                .into_iter()
                .map(|(name, function)| {
                    (
                        name.to_string(),
                        Element::NativeOperation(NativeOperation(function)),
                    )
                })
                .collect()],
            outputs: vec![],
        }
    }

    pub fn outputs(&self) -> &Vec<i32> {
        &self.outputs
    }

    pub fn list(&self) -> &Vec<Element> {
        &self.list
    }

    /// 要素を評価する
    pub fn evaluate(&mut self, element: Element) {
        match element {
            Element::Number(_) | Element::Block(_) | Element::Symbol(_) => self.push(element),
            Element::Operation(operation) => self.execute(operation),
            Element::NativeOperation(_) => panic!("Native operation is not allowed!"),
        }
    }

    /// 複数の要素を評価する
    fn evaluate_multiple(&mut self, elements: Vec<Element>) {
        for element in elements {
            self.evaluate(element);
        }
    }

    /// スタックに要素を入れる
    fn push(&mut self, element: Element) {
        self.list.push(element);
    }

    /// 変数を見つける
    fn find_variable(&self, name: &str) -> Option<Element> {
        self.variables
            .iter()
            .rev()
            .find_map(|vars| vars.get(name).map(|var| var.to_owned()))
    }

    /// 演算を実行する
    fn execute(&mut self, operation: String) {
        let element = self
            .find_variable(&operation)
            .expect(&format!("{operation:?} is undefined"))
            .clone();

        match element {
            Element::Block(block) => {
                self.variables.push(HashMap::new());
                self.evaluate_multiple(block.to_vec());
                self.variables.pop();
            }
            Element::NativeOperation(operation) => (operation.0)(self),
            _ => self.push(element),
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
    fn operate_if(&mut self) {
        let false_branch = self.list.pop().unwrap().to_block_vec();
        let true_branch = self.list.pop().unwrap().to_block_vec();
        let condition = self.list.pop().unwrap().to_block_vec();

        self.evaluate_multiple(condition);

        let condition_result = self.list.pop().unwrap().as_number();

        match condition_result {
            0 => self.evaluate_multiple(false_branch),
            _ => self.evaluate_multiple(true_branch),
        }
    }

    /// 変数定義を行う
    fn operate_define(&mut self) {
        let element = self.list.pop().unwrap();
        self.evaluate(element);
        let element = self.list.pop().unwrap();
        let symbol = self.list.pop().unwrap().as_symbol();

        self.variables.last_mut().unwrap().insert(symbol, element);
    }

    /// for文による繰り返し操作を行う
    fn operate_for(&mut self) {
        let loop_block = self.list.pop().unwrap().to_block_vec();
        let end = self.list.pop().unwrap().as_number();
        let start = self.list.pop().unwrap().as_number();

        for _ in start..=end {
            self.evaluate_multiple(loop_block.clone());
        }
    }

    /// while文による繰り返し操作を行う
    fn operate_while(&mut self) {
        let loop_block = self.list.pop().unwrap().to_block_vec();
        let condition = self.list.pop().unwrap().to_block_vec();
        self.evaluate_multiple(condition.clone());
        let mut condition_result = self.list.pop().unwrap().as_number();

        while condition_result != 0 {
            self.evaluate_multiple(loop_block.clone());
            self.evaluate_multiple(condition.clone());
            condition_result = self.list.pop().unwrap().as_number();
        }
    }

    /// スタックの先頭を取り出して表示する
    fn puts(&mut self) {
        let element = self.list.pop().unwrap();
        self.outputs.push(element.as_number());
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
    use super::Stack;
    use crate::element::tests::create_block;
    use crate::element::Element;
    use crate::parser::{Parser, ParserIterator};

    fn parse(parser: &mut ParserIterator) -> Stack {
        let mut stack = Stack::new();
        for element in parser {
            stack.evaluate(element);
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
        let mut iter = parser.parse(String::from("1 2 + { 3 4 }"));
        let stack = parse(&mut iter);

        assert_eq!(
            stack.list,
            vec![
                Element::Number(3),
                Element::Block(create_block(vec![Element::Number(3), Element::Number(4)]))
            ]
        )
    }

    #[test]
    fn test_if_true() {
        let mut parser = Parser::new();
        let mut iter = parser.parse(String::from("{ 1 -1 + } { 100 } { -100 } if"));
        let stack = parse(&mut iter);

        assert_eq!(stack.list, vec![Element::Number(-100)])
    }

    #[test]
    fn test_if_false() {
        let mut parser = Parser::new();
        let mut iter = parser.parse(String::from("{ 1 1 + } { 100 } { -100 } if"));
        let stack = parse(&mut iter);

        assert_eq!(stack.list, vec![Element::Number(100)])
    }

    #[test]
    fn test_var() {
        let mut parser = Parser::new();
        let mut iter = parser.parse(String::from("/x 10 def /y 20 def x y *"));
        let stack = parse(&mut iter);

        assert_eq!(stack.list, vec![Element::Number(200)]);
    }

    #[test]
    fn test_var_if() {
        let mut parser = Parser::new();
        let mut iter = parser.parse(String::from("/x 10 def /y 20 def { x y < } { x } { y } if"));
        let stack = parse(&mut iter);

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
            for element in parser.parse(line.to_string()) {
                stack.evaluate(element);
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
            for element in parser.parse(line.to_string()) {
                stack.evaluate(element);
            }
        }

        assert_eq!(stack.list, vec![Element::Number(20)]);
    }

    #[test]
    fn test_for() {
        let mut parser = Parser::new();
        let mut iter = parser.parse(String::from("/x 0 def 1 100 { /x x 1 + def } for x"));
        let stack = parse(&mut iter);

        assert_eq!(stack.list, vec![Element::Number(100)]);
    }

    #[test]
    fn test_while() {
        let mut parser = Parser::new();
        let mut iter = parser.parse(String::from(
            "/x 0 def { x 3 < } { x 1 + /x exch def } while x",
        ));
        let stack = parse(&mut iter);

        assert_eq!(stack.list, vec![Element::Number(3)]);
    }
}

/// パーサー
pub struct Parser<'a> {
    words: Vec<&'a str>,
}

impl<'a> Parser<'a> {
    pub fn new(mut words: Vec<&'a str>) -> Self {
        words.reverse();
        Self { words }
    }

    pub fn next(&mut self) -> Option<Element> {
        Element::parse(self.words.pop()?)
    }
}

/// パースした要素
#[derive(Debug)]
pub enum Element {
    /// 数値
    Number(i32),
    /// 演算子
    Operation(Operation),
}

impl Element {
    /// パースする
    fn parse(word: &str) -> Option<Element> {
        if let Ok(num) = word.parse::<i32>() {
            Some(Element::Number(num))
        } else {
            let operation = Operation::parse(word)?;
            Some(Element::Operation(operation))
        }
    }
}

/// 演算の種類
#[derive(Debug)]
pub enum Operation {
    /// 加算
    Add,
    /// 減算
    Subtract,
    /// 乗算
    Multiply,
    /// 除算
    Divide,
}

impl Operation {
    /// パースする
    fn parse(word: &str) -> Option<Operation> {
        match word {
            "+" => Some(Operation::Add),
            "-" => Some(Operation::Subtract),
            "*" => Some(Operation::Multiply),
            "/" => Some(Operation::Divide),
            _ => None,
        }
    }
}

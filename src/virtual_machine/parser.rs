use std::vec::IntoIter;

/// パーサー
pub struct Parser<'a> {
    iter: IntoIter<&'a str>,
}

impl<'a> Parser<'a> {
    pub fn new(line: &'a str) -> Self {
        let words: Vec<_> = line.split(" ").collect();
        let iter = words.into_iter();
        Self { iter }
    }

    pub fn next(&mut self) -> Option<Element> {
        Element::parses(&mut self.iter)
    }
}

/// パースした要素
#[derive(Debug, PartialEq)]
pub enum Element {
    /// 数値
    Number(i32),
    /// 演算子
    Operation(Operation),
    /// ブロック
    Block(Block),
}

impl Element {
    /// パースする
    fn parses<'a>(iter: &mut IntoIter<&'a str>) -> Option<Element> {
        let word = iter.next()?;
        if word.is_empty() {
            return None;
        } else if word == "{" {
            let block = Block::parse(iter)?;
            Some(Element::Block(block))
        } else if let Ok(parsed) = word.parse::<i32>() {
            Some(Element::Number(parsed))
        } else {
            let operation = Operation::parse(word)?;
            Some(Element::Operation(operation))
        }
    }

    pub fn as_number(&self) -> i32 {
        match self {
            Element::Number(num) => *num,
            _ => panic!("Element is not a number"),
        }
    }
}

/// 演算の種類
#[derive(Debug, PartialEq, Eq)]
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

/// ブロック要素を表す構造体
#[derive(Debug, PartialEq)]
pub struct Block {
    tokens: Vec<Element>,
}

impl Block {
    /// パースする
    fn parse<'a>(iter: &mut IntoIter<&'a str>) -> Option<Block> {
        let mut tokens = vec![];

        while let Some(word) = iter.next() {
            if word.is_empty() {
                return None;
            } else if word == "{" {
                let block = Block::parse(iter)?;
                tokens.push(Element::Block(block));
            } else if word == "}" {
                return Some(Block { tokens });
            } else if let Ok(parsed) = word.parse::<i32>() {
                tokens.push(Element::Number(parsed))
            } else {
                let operation = Operation::parse(word)?;
                tokens.push(Element::Operation(operation))
            }
        }

        Some(Block { tokens })
    }
}

#[cfg(test)]
pub mod tests {
    use super::{Block, Element, Operation, Parser};

    pub fn helper_create_block(tokens: Vec<Element>) -> Block {
        Block { tokens }
    }

    #[test]
    fn test_block() {
        let mut parser = Parser::new("{ 3 4 }");
        let actual = parser.next().unwrap();

        assert_eq!(
            actual,
            Element::Block(Block {
                tokens: vec![Element::Number(3), Element::Number(4)]
            })
        );
    }

    #[test]
    fn test_group() {
        let mut parser = Parser::new("1 2 + { 3 4 }");
        let actual = vec![
            parser.next().unwrap(),
            parser.next().unwrap(),
            parser.next().unwrap(),
            parser.next().unwrap(),
        ];

        assert_eq!(
            actual,
            vec![
                Element::Number(1),
                Element::Number(2),
                Element::Operation(Operation::Add),
                Element::Block(Block {
                    tokens: vec![Element::Number(3), Element::Number(4)]
                })
            ]
        );
    }
}

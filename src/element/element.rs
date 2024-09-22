use std::{cell::RefCell, rc::Rc, vec::IntoIter};

use super::{block::Block, native_operation::NativeOperation};

/// 言語を構成する要素
#[derive(Debug, PartialEq, Clone)]
pub enum Element {
    /// 整数
    Integer(i32),
    /// 数値
    Number(f32),
    /// 演算
    Operation(String),
    /// シンボル
    Symbol(String),
    /// ブロック
    Block(Block),
    /// 組み込みの演算
    NativeOperation(NativeOperation),
}

impl Element {
    /// パースする
    pub fn parse(
        iter: &mut IntoIter<String>,
        blocks: &mut Rc<RefCell<Vec<Block>>>,
    ) -> Option<Element> {
        let mut borrowed = blocks.borrow_mut();

        if !borrowed.is_empty() {
            let block = Block::parse(iter, &mut borrowed)?;
            return Some(Element::Block(block));
        }

        let word = iter.next()?;
        if word.is_empty() {
            return None;
        } else if word == "{" {
            let block = Block::parse(iter, &mut borrowed)?;
            Some(Element::Block(block))
        } else if let Ok(parsed) = word.parse::<i32>() {
            Some(Element::Integer(parsed))
        } else if let Ok(parsed) = word.parse::<f32>() {
            Some(Element::Number(parsed))
        } else if word.starts_with("/") {
            Some(Element::Symbol(word[1..].to_owned()))
        } else {
            Some(Element::Operation(word.to_string()))
        }
    }

    pub fn as_integer(&self) -> i32 {
        match self {
            Element::Integer(num) => *num,
            Element::Number(num) => *num as i32,
            _ => panic!("Element is not a number"),
        }
    }

    pub fn as_number(&self) -> f32 {
        match self {
            Element::Integer(num) => *num as f32,
            Element::Number(num) => *num,
            _ => panic!("Element is not a number"),
        }
    }

    pub fn as_symbol(&self) -> String {
        match self {
            Element::Symbol(symbol) => symbol.clone(),
            _ => panic!("Element is not a symbol"),
        }
    }

    pub fn to_block_vec(&self) -> Vec<Element> {
        match self {
            Element::Block(block) => block.to_vec(),
            _ => panic!("Value is not a block"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Element;
    use crate::element::tests::create_block;
    use crate::Parser;

    #[test]
    fn test_group() {
        let mut parser = Parser::new();
        let iter = parser.parse(String::from("1.0 2.0 + { 3.0 4.0 }"));
        let actual: Vec<Element> = iter.collect();

        assert_eq!(
            actual,
            vec![
                Element::Number(1.0),
                Element::Number(2.0),
                Element::Operation("+".to_string()),
                Element::Block(create_block(vec![
                    Element::Number(3.0),
                    Element::Number(4.0)
                ]))
            ]
        );
    }
}

use std::{cell::RefCell, rc::Rc, vec::IntoIter};

use super::{block::Block, native_operation::NativeOperation};

/// 言語を構成する要素
#[derive(Debug, PartialEq, Clone)]
pub enum Element {
    /// 数値
    Number(i32),
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
            Some(Element::Number(parsed))
        } else if word.starts_with("/") {
            Some(Element::Symbol(word[1..].to_owned()))
        } else {
            Some(Element::Operation(word.to_string()))
        }
    }

    pub fn as_number(&self) -> i32 {
        match self {
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

    pub fn to_string(&self) -> String {
        match self {
            Self::Number(num) => num.to_string(),
            Self::Operation(operation) => operation.to_string(),
            Self::Symbol(s) => s.clone(),
            Self::Block(_) => "<Block>".to_string(),
            Self::NativeOperation(_) => "<NativeOp>".to_string(),
        }
    }
}

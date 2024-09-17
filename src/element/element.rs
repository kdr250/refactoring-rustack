use std::vec::IntoIter;

use crate::virtual_machine::Stack;

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
    pub fn parse(iter: &mut IntoIter<String>, blocks: &mut Vec<Block>) -> Option<Element> {
        if !blocks.is_empty() {
            let block = Block::parse(iter, blocks)?;
            return Some(Element::Block(block));
        }

        let word = iter.next()?;
        if word.is_empty() {
            return None;
        } else if word == "{" {
            let block = Block::parse(iter, blocks)?;
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

/// 組み込みの演算
#[derive(Debug, PartialEq, Clone)]
pub struct NativeOperation(pub fn(&mut Stack));

/// ブロック要素を表す構造体
#[derive(Debug, PartialEq, Clone)]
pub struct Block {
    tokens: Vec<Element>,
}

impl Block {
    fn new() -> Self {
        Self { tokens: vec![] }
    }

    pub fn from(tokens: Vec<Element>) -> Self {
        Self { tokens }
    }

    fn add(&mut self, element: Element) {
        self.tokens.push(element);
    }

    /// パースする
    fn parse(iter: &mut IntoIter<String>, blocks: &mut Vec<Block>) -> Option<Block> {
        if blocks.is_empty() {
            blocks.push(Block::new());
        }

        let index = blocks.len() - 1;

        while let Some(word) = iter.next() {
            if word.is_empty() {
                continue;
            } else if word == "{" {
                blocks.push(Block::new());
                let inner_block = Block::parse(iter, blocks)?;
                if blocks.is_empty() {
                    return Some(inner_block);
                } else {
                    blocks[index].add(Element::Block(inner_block));
                }
            } else if word == "}" {
                let block = blocks.pop().unwrap();
                if blocks.is_empty() {
                    return Some(block);
                } else {
                    blocks[index - 1].add(Element::Block(block));
                    return Block::parse(iter, blocks);
                }
            } else if let Ok(parsed) = word.parse::<i32>() {
                blocks[index].add(Element::Number(parsed))
            } else if word.starts_with("/") {
                blocks[index].add(Element::Symbol(word[1..].to_owned()))
            } else {
                blocks[index].add(Element::Operation(word.to_string()))
            }
        }

        None
    }

    pub fn to_vec(&self) -> Vec<Element> {
        self.tokens.clone()
    }
}

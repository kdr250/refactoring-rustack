use std::{cell::RefMut, vec::IntoIter};

use super::Element;

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
    pub fn parse(iter: &mut IntoIter<String>, blocks: &mut RefMut<Vec<Block>>) -> Option<Block> {
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

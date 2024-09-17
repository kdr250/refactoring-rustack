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

#[cfg(test)]
pub mod tests {
    use crate::Parser;

    use super::{Block, Element};

    pub fn create_block(tokens: Vec<Element>) -> Block {
        Block { tokens }
    }

    #[test]
    fn test_block() {
        let mut parser = Parser::new();
        let mut iter = parser.parse(String::from("{ 3 4 }"));
        let actual = iter.next().unwrap();

        assert_eq!(
            actual,
            Element::Block(create_block(vec![Element::Number(3), Element::Number(4)]))
        );
    }

    #[test]
    fn test_group2() {
        let mut parser = Parser::new();
        let iter = parser.parse(String::from("{ { 3 } 4 }"));
        let actual: Vec<Element> = iter.collect();

        assert_eq!(
            actual,
            vec![Element::Block(create_block(vec![
                Element::Block(create_block(vec![Element::Number(3)])),
                Element::Number(4)
            ]))]
        );
    }

    #[test]
    fn test_multiline() {
        let mut parser = Parser::new();
        let mut actual = vec![];
        let lines = r#"
{ { 3
{ 5
}
}
}
"#;
        for line in lines.lines() {
            for element in parser.parse(String::from(line)) {
                actual.push(element);
            }
        }

        assert_eq!(
            actual,
            vec![Element::Block(create_block(vec![Element::Block(
                create_block(vec![
                    Element::Number(3),
                    Element::Block(create_block(vec![Element::Number(5)]))
                ])
            )]))]
        );
    }
}

use std::{cell::RefCell, rc::Rc, vec::IntoIter};

use crate::element::{Block, Element};

/// パーサー
#[derive(Debug)]
pub struct Parser {
    blocks: Rc<RefCell<Vec<Block>>>,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            blocks: Rc::new(RefCell::new(vec![])),
        }
    }

    pub fn parse(&mut self, line: String) -> ParserIterator {
        let words: Vec<String> = line.split(" ").map(str::to_string).collect();

        ParserIterator {
            iter: words.into_iter(),
            blocks: self.blocks.clone(),
        }
    }
}

#[derive(Debug)]
pub struct ParserIterator {
    iter: IntoIter<String>,
    blocks: Rc<RefCell<Vec<Block>>>,
}

impl Iterator for ParserIterator {
    type Item = Element;

    fn next(&mut self) -> Option<Self::Item> {
        Element::parse(&mut self.iter, &mut self.blocks)
    }
}

#[cfg(test)]
pub mod tests {
    use super::{Block, Element, Parser};

    #[test]
    fn test_block() {
        let mut parser = Parser::new();
        let mut iter = parser.parse(String::from("{ 3 4 }"));
        let actual = iter.next().unwrap();

        assert_eq!(
            actual,
            Element::Block(Block::from(vec![Element::Number(3), Element::Number(4)]))
        );
    }

    #[test]
    fn test_group() {
        let mut parser = Parser::new();
        let mut iter = parser.parse(String::from("1 2 + { 3 4 }"));
        let actual = vec![
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
        ];

        assert_eq!(
            actual,
            vec![
                Element::Number(1),
                Element::Number(2),
                Element::Operation("+".to_string()),
                Element::Block(Block::from(vec![Element::Number(3), Element::Number(4)]))
            ]
        );
    }

    #[test]
    fn test_group2() {
        let mut parser = Parser::new();
        let mut iter = parser.parse(String::from("{ { 3 } 4 }"));
        let actual = vec![iter.next().unwrap()];

        assert_eq!(
            actual,
            vec![Element::Block(Block::from(vec![
                Element::Block(Block::from(vec![Element::Number(3)])),
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
            vec![Element::Block(Block::from(vec![Element::Block(
                Block::from(vec![
                    Element::Number(3),
                    Element::Block(Block::from(vec![Element::Number(5)]))
                ])
            )]))]
        );
    }
}

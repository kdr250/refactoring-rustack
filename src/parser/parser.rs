use std::vec::IntoIter;

use crate::element::{Block, Element};

/// パーサー
#[derive(Debug)]
pub struct Parser {
    iter: IntoIter<String>,
    blocks: Vec<Block>,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            iter: Default::default(),
            blocks: vec![],
        }
    }

    pub fn parse(&mut self, line: String) {
        let words: Vec<String> = line.split(" ").map(str::to_string).collect();
        let iter = words.into_iter();
        self.iter = iter;
    }

    pub fn next(&mut self) -> Option<Element> {
        Element::parse(&mut self.iter, &mut self.blocks)
    }
}

#[cfg(test)]
pub mod tests {
    use super::{Block, Element, Parser};

    #[test]
    fn test_block() {
        let mut parser = Parser::new();
        parser.parse(String::from("{ 3 4 }"));
        let actual = parser.next().unwrap();

        assert_eq!(
            actual,
            Element::Block(Block::from(vec![Element::Number(3), Element::Number(4)]))
        );
    }

    #[test]
    fn test_group() {
        let mut parser = Parser::new();
        parser.parse(String::from("1 2 + { 3 4 }"));
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
                Element::Operation("+".to_string()),
                Element::Block(Block::from(vec![Element::Number(3), Element::Number(4)]))
            ]
        );
    }

    #[test]
    fn test_group2() {
        let mut parser = Parser::new();
        parser.parse(String::from("{ { 3 } 4 }"));
        let actual = vec![parser.next().unwrap()];

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
            parser.parse(String::from(line));
            while let Some(element) = parser.next() {
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

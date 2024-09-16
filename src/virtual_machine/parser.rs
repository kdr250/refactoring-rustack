use std::vec::IntoIter;

use super::stack::Stack;

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

/// パースした要素
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
    fn parse(iter: &mut IntoIter<String>, blocks: &mut Vec<Block>) -> Option<Element> {
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
    use super::{Block, Element, Parser};

    pub fn helper_create_block(tokens: Vec<Element>) -> Block {
        Block { tokens }
    }

    #[test]
    fn test_block() {
        let mut parser = Parser::new();
        parser.parse(String::from("{ 3 4 }"));
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
                Element::Block(Block {
                    tokens: vec![Element::Number(3), Element::Number(4)]
                })
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
            vec![Element::Block(Block {
                tokens: vec![
                    Element::Block(Block {
                        tokens: vec![Element::Number(3)]
                    }),
                    Element::Number(4)
                ]
            })]
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
            vec![Element::Block(Block {
                tokens: vec![Element::Block(Block {
                    tokens: vec![
                        Element::Number(3),
                        Element::Block(Block {
                            tokens: vec![Element::Number(5)]
                        })
                    ]
                })]
            })]
        );
    }
}

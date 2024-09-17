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

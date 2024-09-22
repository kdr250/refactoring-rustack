use std::io::BufRead;
use std::io::BufReader;

use parser::Parser;
use virtual_machine::VirtualMachine;

mod element;
mod parser;
mod virtual_machine;

fn main() {
    if let Some(file) = std::env::args()
        .nth(1)
        .and_then(|f| std::fs::File::open(f).ok())
    {
        parse_batch(BufReader::new(file));
    } else {
        parse_interactive();
    }
}

/// 一括でパースして処理する
fn parse_batch(source: impl BufRead) {
    let mut virtual_machine = VirtualMachine::new();
    let mut parser = Parser::new();

    for line in source.lines().flatten() {
        for element in parser.parse(line) {
            virtual_machine.evaluate(element);
        }
    }

    virtual_machine.print_outputs();
}

/// 標準入力をインタラクティブにパースして処理する
fn parse_interactive() {
    let mut virtual_machine = VirtualMachine::new();
    let mut parser = Parser::new();

    for line in std::io::stdin().lines().flatten() {
        for element in parser.parse(line) {
            virtual_machine.evaluate(element);
        }

        println!("stack: {:?}", virtual_machine.stack().list());
    }
}

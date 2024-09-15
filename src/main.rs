use std::io::BufRead;
use std::io::BufReader;

use virtual_machine::Parser;
use virtual_machine::VirtualMachine;

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

fn parse_batch(source: impl BufRead) {
    let mut virtual_machine = VirtualMachine::new();
    let mut parser = Parser::new();

    for line in source.lines().flatten() {
        parser.parse(line);
        while let Some(element) = parser.next() {
            virtual_machine.process(element);
        }
    }
}

fn parse_interactive() {
    let mut virtual_machine = VirtualMachine::new();
    let mut parser = Parser::new();

    for line in std::io::stdin().lines().flatten() {
        parser.parse(line);
        while let Some(element) = parser.next() {
            virtual_machine.process(element);
        }

        println!("stack: {:?}", virtual_machine.stack());
    }
}

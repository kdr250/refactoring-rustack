use virtual_machine::Parser;
use virtual_machine::VirtualMachine;

mod virtual_machine;

fn main() {
    for line in std::io::stdin().lines().flatten() {
        let words: Vec<_> = line.split(" ").collect();
        let mut parser = Parser::new(words);
        let mut virtual_machine = VirtualMachine::new();

        while let Some(element) = parser.next() {
            virtual_machine.push(element);
        }

        println!("{:?}", virtual_machine);
    }
}

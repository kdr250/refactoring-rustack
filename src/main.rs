use virtual_machine::Parser;
use virtual_machine::VirtualMachine;

mod virtual_machine;

fn main() {
    for line in std::io::stdin().lines().flatten() {
        let mut parser = Parser::new(&line);
        let mut virtual_machine = VirtualMachine::new();

        while let Some(element) = parser.next() {
            virtual_machine.process(element);
        }

        println!("{:?}", virtual_machine);
    }
}

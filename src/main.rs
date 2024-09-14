mod virtual_machine;
use crate::virtual_machine::*;

fn main() {
    let mut vm = VirtualMachine::new();

    println!("{:?}", vm);

    vm.add();
    vm.add();

    println!("{:?}", vm);
}

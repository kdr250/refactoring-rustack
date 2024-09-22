macro_rules! impl_operation {
    {$name:ident, $op:tt} => {
        fn $name(&mut self) {
            let rhs = self.list.pop().unwrap().as_number();
            let lhs = self.list.pop().unwrap().as_number();
            self.list.push(Element::Number((lhs $op rhs) as f32));
        }
    }
}

macro_rules! impl_operation_integer {
    {$name:ident, $op:tt} => {
        fn $name(&mut self) {
            let rhs = self.list.pop().unwrap().as_number();
            let lhs = self.list.pop().unwrap().as_number();
            self.list.push(Element::Integer((lhs $op rhs) as i32));
        }
    }
}

pub(crate) use impl_operation;
pub(crate) use impl_operation_integer;

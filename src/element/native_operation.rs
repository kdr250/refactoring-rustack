use crate::virtual_machine::Stack;

/// 組み込みの演算
#[derive(Debug, PartialEq, Clone)]
pub struct NativeOperation(pub fn(&mut Stack));

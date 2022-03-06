#![allow(non_camel_case_types)]

use crate::rtda::Frame;
use crate::rtda::cp_methodref::MethodRef;
use super::super::instruction::Instruction;
use super::super::bytecode_reader::BytecodeReader;
use super::super::invoke_method;

/// Invoke a class (static) method
#[derive(Default, Debug)]
pub struct INVOKE_STATIC {
    index: u64,
}

impl Instruction for INVOKE_STATIC {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.index = reader.read_u16() as u64;
    }

    fn execute(&mut self, frame: &mut Frame) {
        let current_class = frame.get_method().borrow().get_class();
        let r_cp = current_class.borrow().constant_pool();
        let resolved_method = r_cp.borrow_mut().get_constant_mut(self.index as usize)
            .as_any_mut().downcast_mut::<MethodRef>().unwrap().resolved_method(current_class);
        if !resolved_method.borrow().is_static() {
            panic!("java.lang.IncompatibleClassChangeError");
        }

        let class = resolved_method.borrow().get_class();

        invoke_method(frame, &resolved_method);
    }
}

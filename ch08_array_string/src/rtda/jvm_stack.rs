use super::frame::Frame;
use std::rc::Rc;
use std::cell::RefCell;

/// JVM Stack
pub struct Stack {
    max_size: usize,
    top: usize,
    frames: Vec<Option<Rc<RefCell<Frame>>>>,
}

impl Stack {
    pub fn new(max_size: usize) -> Self {
        Stack {
            max_size,
            top: 0,
            frames: vec![None; max_size],
        }
    }

    pub fn push(&mut self, frame: Frame) {
        if self.top >= self.max_size {
            panic!("java.lang.StackOverflowError");
        }

        self.frames[self.top] = Some(Rc::new(RefCell::new(frame)));
        self.top += 1;
    }

    pub fn pop(&mut self) -> Option<Rc<RefCell<Frame>>> {
        if self.top == 0 {
            panic!("jvm stack is empty!");
        }
        self.top -= 1;
        self.frames[self.top].clone()
    }

    pub fn top(&self) -> Rc<RefCell<Frame>> {
        if self.top == 0 {
            panic!("jvm stack is empty!");
        }
        self.frames[self.top - 1].clone().unwrap()
    }

    pub fn is_empty(&self) -> bool {
        self.top == 0
    }
}
use super::local_vars::Slot;
use super::object::Object;
use std::rc::Rc;
use std::cell::RefCell;

pub struct OperandStack {
    size: usize,
    slots: Vec<Slot>,
}

impl OperandStack {
    pub fn new(max_stack: usize) -> Self {
        OperandStack {
            size: 0,
            slots: vec![Slot::default(); max_stack],
        }
    }

    pub fn push_int(&mut self, val: i32) {
        self.slots[self.size].num = val;
        self.size += 1;
    }

    pub fn pop_int(&mut self) -> i32 {
        self.size -= 1;
        self.slots[self.size].num
    }

    pub fn push_float(&mut self, val: f32) {
        let bytes = f32::to_be_bytes(val);
        self.slots[self.size].num = i32::from_be_bytes(bytes);
        self.size += 1;
    }

    pub fn pop_float(&mut self) -> f32 {
        self.size -= 1;
        let bytes = i32::to_be_bytes(self.slots[self.size].num);
        f32::from_be_bytes(bytes)
    }

    pub fn push_long(&mut self, val: i64) {
        // Long consumes two slots
        self.slots[self.size].num = val as i32;
        self.slots[self.size + 1].num = (val >> 32) as i32;
        self.size += 2;
    }

    pub fn pop_long(&mut self) -> i64 {
        self.size -= 2;
        let low = self.slots[self.size].num as u32;
        let high = self.slots[self.size + 1].num as u32;
        (high as i64) << 32 | low as i64
    }

    pub fn push_double(&mut self, val: f64) {
        // Double consumes two slots
        let bytes = f64::to_be_bytes(val);
        self.push_long(i64::from_be_bytes(bytes));
    }

    pub fn pop_double(&mut self) -> f64 {
        let bytes = i64::to_be_bytes(self.pop_long());
        f64::from_be_bytes(bytes)
    }

    pub fn push_ref(&mut self, val: Option<Rc<RefCell<Object>>>) {
        self.slots[self.size]._ref = val;
        self.size += 1;
    }

    pub fn pop_ref(&mut self) -> Option<Rc<RefCell<Object>>> {
        self.size -= 1;
        self.slots[self.size]._ref.clone()
    }

    pub fn push_slot(&mut self, slot: Slot) {
        self.slots[self.size] = slot;
        self.size += 1;
    }

    pub fn pop_slot(&mut self) -> Slot {
        self.size -= 1;
        self.slots[self.size].clone()
    }

    pub fn get_ref_from_top(&self, n: usize) -> Option<Rc<RefCell<Object>>> {
        self.slots[self.size - n - 1]._ref.clone()
    }

    pub fn pop_boolean(&mut self) -> bool {
        self.pop_int() == 1
    }

    pub fn push_boolean(&mut self, val: bool) {
        if val {
            self.push_int(1);
        } else {
            self.push_int(0);
        }
    }
}

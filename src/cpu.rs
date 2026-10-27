pub mod dex;
mod opcodes;

use std::{cell::RefCell, rc::Rc};

use crate::ram::MemCtrl;

enum Status {
    Carry = 0,
    Zero = 1,
    Interrupt = 2,
    Decimal = 3, // unused for nes
    Break = 4,
    Overflow = 6,
    Negative = 7,
}

pub struct CPU {
    pc: u16,
    acc: u8,
    x: u8,
    y: u8,
    sr: u8,
    sp: u8,
    ram: Rc<RefCell<MemCtrl>>,
}

impl CPU {
    pub fn new(ram: Rc<RefCell<MemCtrl>>) -> Self {
        Self {
            pc: 0,
            acc: 0,
            x: 0,
            y: 0,
            sr: 0,
            sp: 0xFF,
            ram,
        }
    }

    fn update_status(&mut self, status: Status, value: Option<bool>) {
        match value {
            Some(true) => self.sr |= 1 << status as u32,
            Some(false) => self.sr &= !(1 << status as u32),
            None => self.sr ^= 1 << status as u32,
        }
    }

    fn get_status(&self, status: Status) -> bool {
        self.sr & 1 << status as u32 != 0
    }

    fn push_u8(&mut self, v: u8) {
        self.ram.borrow_mut().write(0x100 + self.sp as u16, v);
        self.sp -= 1;
    }

    fn push_u16(&mut self, v: u16) {
        self.push_u8((v >> 8) as u8);
        self.push_u8(v as u8);
    }

    fn pop_u8(&mut self) -> u8 {
        let val = self.ram.borrow().read(0x100 + self.sp as u16);
        self.sp += 1;
        val
    }

    fn pop_u16(&mut self) -> u16 {
        (self.pop_u8() as u16) | (self.pop_u8() as u16) << 8
    }
}

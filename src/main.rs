#![allow(unused)]

use std::{cell::RefCell, rc::Rc};

use crate::{cpu::CPU, ram::MemCtrl};

mod cpu;
mod ram;

struct Bus {
    cpu: CPU,
    ram: Rc<RefCell<MemCtrl>>,
}

impl Bus {
    fn new() -> Self {
        let ram = Rc::new(RefCell::new(MemCtrl::new()));
        Self {
            cpu: CPU::new(ram.clone()),
            ram,
        }
    }
}

fn main() {
    println!("Hello, world!");
}

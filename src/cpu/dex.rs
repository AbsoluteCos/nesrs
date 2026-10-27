use super::*;

// https://www.pagetable.com/c64ref/6502/
impl CPU {
    // ADDRESSING MODES
    fn absolute(&self, ab: u16) -> u8 {
        self.ram.borrow().read(ab)
    }

    fn update_absolute(&mut self, ab: u16, val: u8) {
        self.ram.borrow_mut().write(ab, val);
    }

    fn x_indexed_absolute(&self, ab: u16) -> u8 {
        self.absolute(ab + self.x as u16)
    }

    fn y_indexed_absolute(&self, ab: u16) -> u8 {
        self.absolute(ab + self.y as u16)
    }

    fn update_x_indexed_absolute(&mut self, ab: u16, val: u8) {
        self.update_absolute(ab + self.x as u16, val);
    }

    fn update_y_indexed_absolute(&mut self, ab: u16, val: u8) {
        self.update_absolute(ab + self.y as u16, val);
    }

    fn zero_page(&self, a: u8) -> u8 {
        self.ram.borrow().read(a as u16)
    }

    fn update_zero_page(&mut self, a: u8, val: u8) {
        self.ram.borrow_mut().write(a as u16, val);
    }

    fn x_indexed_zero_page(&self, a: u8) -> u8 {
        let zero_page_address = self.x.wrapping_add(a) as u16;
        self.ram.borrow().read(zero_page_address)
    }

    fn update_x_indexed_zero_page(&mut self, a: u8, val: u8) {
        let zero_page_address = self.x.wrapping_add(a) as u16;
        self.ram.borrow_mut().write(zero_page_address, val);
    }

    fn y_indexed_zero_page(&self, a: u8) -> u8 {
        let zero_page_address = self.y.wrapping_add(a) as u16;
        self.ram.borrow().read(zero_page_address)
    }

    fn update_y_indexed_zero_page(&mut self, a: u8, val: u8) {
        let zero_page_address = self.y.wrapping_add(a) as u16;
        self.ram.borrow_mut().write(zero_page_address, val);
    }

    fn x_indexed_zero_page_indirect(&self, a: u8) -> u8 {
        let zero_page_address = self.x.wrapping_add(a) as u16;
        let addr = self.ram.borrow().read(zero_page_address) as u16
            | (self.ram.borrow().read(zero_page_address + 1) as u16) << 8;
        self.ram.borrow().read(addr)
    }

    fn y_indexed_zero_page_indirect(&self, a: u8) -> u8 {
        let zero_page_address = self.y.wrapping_add(a) as u16;
        let addr = self.ram.borrow().read(zero_page_address) as u16
            | (self.ram.borrow().read(zero_page_address + 1) as u16) << 8;
        self.ram.borrow().read(addr)
    }

    pub fn decode_and_execute(&mut self) {
        let instruction = self.ram.borrow().read(self.pc);
        let a = self.ram.borrow().read(self.pc + 1);
        let b = self.ram.borrow().read(self.pc + 2);
        let ab = ((b as u16) << 8) | (a as u16);
        match instruction {
            0x00 => {
                self.pc += 1;
                self.brk();
            }
            0x01 => {
                self.pc += 2;
                self.ora(self.x_indexed_zero_page_indirect(a));
            }
            0x05 => {
                self.pc += 2;
                self.ora(self.zero_page(a));
            }
            0x06 => {
                self.pc += 2;
                let mut val = self.zero_page(a);
                self.asl(&mut val);
                self.update_zero_page(a, val);
            }
            0x08 => {
                self.pc += 1;
                self.php();
            }
            0x09 => {
                self.pc += 2;
                self.ora(a);
            }
            0x0A => {
                self.pc += 1;
                let mut val = self.acc;
                self.asl(&mut val);
                self.acc = val;
            }
            0x0D => {
                self.pc += 3;
                self.ora(self.absolute(ab));
            }
            0x0E => {
                self.pc += 3;
                let mut val = self.absolute(ab);
                self.asl(&mut val);
                self.update_absolute(ab, val);
            }
            0x10 => {
                self.pc += 2;
                self.bpl(a as i8);
            }
            0x11 => {
                self.pc += 2;
                self.ora(self.y_indexed_zero_page_indirect(a));
            }
            0x15 => {
                self.pc += 2;
                self.ora(self.x_indexed_zero_page(a));
            }
            0x16 => {
                self.pc += 2;
                let mut val = self.x_indexed_zero_page(a);
                self.asl(&mut val);
                self.update_x_indexed_zero_page(a, val);
            }
            0x18 => {
                self.pc += 1;
                self.clc();
            }
            0x19 => {
                self.pc += 3;
                self.ora(self.y_indexed_absolute(ab));
            }
            0x1D => {
                self.pc += 3;
                self.ora(self.x_indexed_absolute(ab));
            }
            0x1E => {
                self.pc += 3;
                let mut val = self.x_indexed_absolute(ab);
                self.asl(&mut val);
                self.update_x_indexed_absolute(ab, val);
            }
            0x20 => {
                self.pc += 3;
                self.jsr(ab);
            }
            0x21 => {
                self.pc += 2;
                self.and(self.x_indexed_zero_page_indirect(a));
            }
            0x24 => {
                self.pc += 2;
                self.bit(self.zero_page(a));
            }
            0x25 => {
                self.pc += 2;
                self.and(self.zero_page(a));
            }
            0x26 => {
                self.pc += 2;
                let mut val = self.zero_page(a);
                self.rol(&mut val);
                self.update_zero_page(a, val);
            }
            0x28 => {
                self.pc += 1;
                self.plp();
            }
            0x29 => {
                self.pc += 2;
                self.and(a);
            }
            0x2A => {
                self.pc += 1;
                let mut val = self.acc;
                self.rol(&mut val);
                self.acc = val;
            }
            0x2C => {
                self.pc += 3;
                self.bit(self.absolute(ab));
            }
            0x2D => {
                self.pc += 3;
                self.and(self.absolute(ab));
            }
            0x2E => {
                self.pc += 3;
                let mut val = self.absolute(ab);
                self.rol(&mut val);
                self.update_absolute(ab, val);
            }
            0x30 => {
                self.pc += 2;
                self.bmi(a as i8);
            }
            0x31 => {
                self.pc += 2;
                self.and(self.y_indexed_zero_page_indirect(a));
            }
            0x35 => {
                self.pc += 2;
                self.and(self.x_indexed_zero_page(a));
            }
            0x36 => {
                self.pc += 2;
                let mut val = self.x_indexed_zero_page(a);
                self.rol(&mut val);
                self.update_x_indexed_zero_page(a, val);
            }
            0x38 => {
                self.pc += 1;
                self.sec();
            }
            0x39 => {
                self.pc += 3;
                self.and(self.y_indexed_absolute(ab));
            }
            0x3D => {
                self.pc += 3;
                self.and(self.x_indexed_absolute(ab));
            }
            0x3E => {
                self.pc += 3;
                let mut val = self.x_indexed_absolute(ab);
                self.rol(&mut val);
                self.update_x_indexed_absolute(ab, val);
            }

            op => panic!("Unsupported opcode {op}"),
        }
    }
}

use super::*;

impl CPU {
    // MEMORY
    pub fn lda(&mut self, addr: u16) {
        self.acc = self.ram.borrow().read(addr);
        self.update_status(Status::Negative, Some(self.acc & 0b10000000 != 0));
        self.update_status(Status::Zero, Some(self.acc == 0));
    }

    pub fn ldx(&mut self, addr: u16) {
        self.x = self.ram.borrow().read(addr);
        self.update_status(Status::Negative, Some(self.x & 0b10000000 != 0));
        self.update_status(Status::Zero, Some(self.x == 0));
    }

    pub fn ldy(&mut self, addr: u16) {
        self.y = self.ram.borrow().read(addr);
        self.update_status(Status::Negative, Some(self.y & 0b10000000 != 0));
        self.update_status(Status::Zero, Some(self.y == 0));
    }

    pub fn sta(&mut self, addr: u16) {
        self.ram.borrow_mut().write(addr, self.acc);
    }

    pub fn stx(&mut self, addr: u16) {
        self.ram.borrow_mut().write(addr, self.x);
    }

    pub fn sty(&mut self, addr: u16) {
        self.ram.borrow_mut().write(addr, self.y);
    }

    // TRANSFER
    pub fn tax(&mut self) {
        self.x = self.acc;
        self.update_status(Status::Negative, Some(self.x & 0b10000000 != 0));
        self.update_status(Status::Zero, Some(self.x == 0));
    }

    pub fn tay(&mut self) {
        self.y = self.acc;
        self.update_status(Status::Negative, Some(self.y & 0b10000000 != 0));
        self.update_status(Status::Zero, Some(self.y == 0));
    }

    pub fn tsx(&mut self) {
        self.x = self.sp;
        self.update_status(Status::Negative, Some(self.x & 0b10000000 != 0));
        self.update_status(Status::Zero, Some(self.x == 0));
    }

    pub fn txa(&mut self) {
        self.acc = self.x;
        self.update_status(Status::Negative, Some(self.acc & 0b10000000 != 0));
        self.update_status(Status::Zero, Some(self.acc == 0));
    }

    pub fn txs(&mut self) {
        self.sp = self.x;
    }

    pub fn tya(&mut self) {
        self.acc = self.y;
        self.update_status(Status::Negative, Some(self.acc & 0b10000000 != 0));
        self.update_status(Status::Zero, Some(self.acc == 0));
    }

    // STACK
    pub fn pha(&mut self) {
        self.ram
            .borrow_mut()
            .write(0x100 + self.sp as u16, self.acc);
        self.sp -= 1;
    }

    pub fn php(&mut self) {
        self.ram.borrow_mut().write(0x100 + self.sp as u16, self.sr);
        self.sp -= 1;
    }

    pub fn pla(&mut self) {
        self.acc = self.ram.borrow().read(0x100 + self.sp as u16);
        self.sp += 1;
        self.update_status(Status::Negative, Some(self.acc & 0b10000000 != 0));
        self.update_status(Status::Zero, Some(self.acc == 0));
    }

    pub fn plp(&mut self) {
        self.sr = self.ram.borrow().read(0x100 + self.sp as u16);
        self.sp += 1;
    }

    // SHIFT
    pub fn asl(&mut self, m: &mut u8) {
        self.update_status(Status::Carry, Some(*m & 0b10000000 != 0));
        *m <<= 1;
        self.update_status(Status::Negative, Some(*m & 0b10000000 != 0));
        self.update_status(Status::Zero, Some(*m == 0));
    }

    pub fn lsr(&mut self, m: &mut u8) {
        self.update_status(Status::Carry, Some(*m & 1 == 1));
        *m >>= 1;
        self.update_status(Status::Negative, Some(false));
        self.update_status(Status::Zero, Some(*m == 0));
    }

    pub fn rol(&mut self, m: &mut u8) {
        let c = if self.get_status(Status::Carry) { 1 } else { 0 };
        self.update_status(Status::Carry, Some(*m & 0b10000000 != 0));
        *m <<= 1;
        *m |= c;
        self.update_status(Status::Negative, Some(*m & 0b10000000 != 0));
        self.update_status(Status::Zero, Some(*m == 0));
    }

    pub fn ror(&mut self, m: &mut u8) {
        let c = if self.get_status(Status::Carry) {
            0b10000000
        } else {
            0
        };
        self.update_status(Status::Carry, Some(*m & 1 != 0));
        *m >>= 1;
        *m |= c;
        self.update_status(Status::Negative, Some(*m & 0b10000000 != 0));
        self.update_status(Status::Zero, Some(*m == 0));
    }

    // LOGIC
    pub fn and(&mut self, m: u8) {
        self.acc &= m;
        self.update_status(Status::Negative, Some(self.acc & 0b10000000 != 0));
        self.update_status(Status::Zero, Some(self.acc == 0));
    }

    pub fn bit(&mut self, m: u8) {
        let result = self.acc & m;
        self.update_status(Status::Negative, Some(result & 0b10000000 != 0));
        self.update_status(Status::Overflow, Some(result & 0b01000000 != 0));
        self.update_status(Status::Zero, Some(result == 0));
    }

    pub fn eor(&mut self, m: u8) {
        self.acc ^= m;
        self.update_status(Status::Negative, Some(self.acc & 0b10000000 != 0));
        self.update_status(Status::Zero, Some(self.acc == 0));
    }

    pub fn ora(&mut self, m: u8) {
        self.acc |= m;
        self.update_status(Status::Negative, Some(self.acc & 0b10000000 != 0));
        self.update_status(Status::Zero, Some(self.acc == 0));
    }

    // ARITHMETIC
    pub fn adc(&mut self, m: u8) {
        let value = self
            .acc
            .wrapping_add(m)
            .wrapping_add(self.get_status(Status::Carry) as u8);
        self.update_status(Status::Carry, Some(self.acc > value));
        self.update_status(
            Status::Overflow,
            Some(!(self.acc & m) & (m ^ value) & 0b10000000 != 0), // args same sign and sum
                                                                   // has different sign
        );
        self.update_status(Status::Negative, Some(value & 0b10000000 != 0));
        self.update_status(Status::Zero, Some(value == 0));
        self.acc = value;
    }

    pub fn cmp(&mut self, m: u8) {
        self.update_status(Status::Carry, Some(m <= self.acc));
        self.update_status(Status::Negative, Some(self.acc as i8 - (m as i8) < 0));
        self.update_status(Status::Zero, Some(m == self.acc));
    }

    pub fn cpx(&mut self, m: u8) {
        self.update_status(Status::Carry, Some(m <= self.x));
        self.update_status(Status::Negative, Some(self.x as i8 - (m as i8) < 0));
        self.update_status(Status::Zero, Some(m == self.x));
    }

    pub fn cpy(&mut self, m: u8) {
        self.update_status(Status::Carry, Some(m <= self.y));
        self.update_status(Status::Negative, Some(self.y as i8 - (m as i8) < 0));
        self.update_status(Status::Zero, Some(m == self.y));
    }

    pub fn sbc(&mut self, m: u8) {
        self.adc(!m);
    }

    // INCREMENT
    pub fn dec(&mut self, m: &mut u8) {
        *m = m.wrapping_sub(1);
        self.update_status(Status::Negative, Some(*m & 0b10000000 != 0));
        self.update_status(Status::Zero, Some(*m == 0));
    }

    pub fn dex(&mut self) {
        self.x.wrapping_sub(1);
        self.update_status(Status::Negative, Some(self.x & 0b10000000 != 0));
        self.update_status(Status::Zero, Some(self.x == 0));
    }

    pub fn dey(&mut self) {
        self.y.wrapping_sub(1);
        self.update_status(Status::Negative, Some(self.y & 0b10000000 != 0));
        self.update_status(Status::Zero, Some(self.y == 0));
    }

    pub fn inc(&mut self, m: &mut u8) {
        *m = m.wrapping_add(1);
        self.update_status(Status::Negative, Some(*m & 0b10000000 != 0));
        self.update_status(Status::Zero, Some(*m == 0));
    }

    pub fn inx(&mut self) {
        self.x.wrapping_add(1);
        self.update_status(Status::Negative, Some(self.x & 0b10000000 != 0));
        self.update_status(Status::Zero, Some(self.x == 0));
    }

    pub fn iny(&mut self) {
        self.y.wrapping_add(1);
        self.update_status(Status::Negative, Some(self.y & 0b10000000 != 0));
        self.update_status(Status::Zero, Some(self.y == 0));
    }

    // CONTROL

    pub fn brk(&mut self) {
        self.push_u16(self.pc);
        self.pc = 0xFFFE;
        self.update_status(Status::Interrupt, Some(true));
        self.push_u8(self.sr);
    }

    pub fn jmp(&mut self, addr: u16) {
        self.pc = addr;
    }

    pub fn jsr(&mut self, addr: u16) {
        self.push_u16(self.pc);
        self.pc = addr;
    }

    pub fn rti(&mut self) {
        self.sr = self.pop_u8();
        self.pc = self.pop_u16();
    }

    pub fn rts(&mut self) {
        self.pc = self.pop_u16();
    }

    // BRANCH
    pub fn bcc(&mut self, offset: i8) {
        if !self.get_status(Status::Carry) {
            self.pc = self.pc.saturating_add_signed(offset as i16);
        }
    }

    pub fn bcs(&mut self, offset: i8) {
        if self.get_status(Status::Carry) {
            self.pc = self.pc.saturating_add_signed(offset as i16);
        }
    }

    pub fn beq(&mut self, offset: i8) {
        if self.get_status(Status::Zero) {
            self.pc = self.pc.saturating_add_signed(offset as i16);
        }
    }

    pub fn bmi(&mut self, offset: i8) {
        if self.get_status(Status::Negative) {
            self.pc = self.pc.saturating_add_signed(offset as i16);
        }
    }

    pub fn bne(&mut self, offset: i8) {
        if !self.get_status(Status::Zero) {
            self.pc = self.pc.saturating_add_signed(offset as i16);
        }
    }

    pub fn bpl(&mut self, offset: i8) {
        if !self.get_status(Status::Negative) {
            self.pc = self.pc.saturating_add_signed(offset as i16);
        }
    }

    pub fn bvc(&mut self, offset: i8) {
        if !self.get_status(Status::Overflow) {
            self.pc = self.pc.saturating_add_signed(offset as i16);
        }
    }

    pub fn bvs(&mut self, offset: i8) {
        if self.get_status(Status::Overflow) {
            self.pc = self.pc.saturating_add_signed(offset as i16);
        }
    }

    // FLAGS
    pub fn clc(&mut self) {
        self.update_status(Status::Carry, Some(false));
    }

    pub fn cld(&mut self) {
        self.update_status(Status::Decimal, Some(false));
    }

    pub fn cli(&mut self) {
        self.update_status(Status::Interrupt, Some(false));
    }

    pub fn clv(&mut self) {
        self.update_status(Status::Overflow, Some(false));
    }

    pub fn sec(&mut self) {
        self.update_status(Status::Carry, Some(true));
    }

    pub fn sed(&mut self) {
        self.update_status(Status::Decimal, Some(true));
    }

    pub fn sei(&mut self) {
        self.update_status(Status::Interrupt, Some(true));
    }

    // NO-OP
    pub fn nop() {}
}

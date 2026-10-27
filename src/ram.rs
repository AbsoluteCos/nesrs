pub struct MemCtrl {
    ram: [u8; RAM_SIZE],
    vram: [u8; RAM_SIZE],
}

const RAM_SIZE: usize = 2048;

impl MemCtrl {
    pub fn new() -> Self {
        Self {
            // TODO: Random Initialization
            ram: [0; RAM_SIZE],
            vram: [0; RAM_SIZE],
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            a @ 0x0000..=0x1FFF => self.ram[a as usize % RAM_SIZE],
            a @ 0x2000..=0x3FFF => todo!(), // nes ppu registers, mod 8 bytes
            a @ 0x4000..=0x401F => todo!(),
            a @ 0x4020..=0xFFFF => todo!(),
        }
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        match addr {
            a @ 0x0000..=0x1FFF => self.ram[a as usize % RAM_SIZE] = val,
            a @ 0x2000..=0x3FFF => todo!(), // nes ppu registers, mod 8 bytes
            a @ 0x4000..=0x401F => todo!(),
            a @ 0x4020..=0xFFFF => todo!(),
        }
    }
}

// $0000-$07FF 2KB Work RAM
// $0800-$0FFF Mirror of RAM
// $1000-$17FF Mirror of RAM
// $1800-$1FFF Mirror of RAM
// $2000-$2007 NES PPU Registers
// $2008-$3FFF Mirrors of PPU Registers (repeasts every 8 bytes)
// $4000-$401A memory mapped registers
// $401C-$401F unused
// $4020-$FFFF available for Game PAK

// $0000-$00FF Zero Page
// $0100-$01FF Stack (typically 01FF start 0100 end)

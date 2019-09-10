pub const BIOS_SIZE: usize = 16 * 1024;
pub const BIOS_START: usize = 0x0000_0000;
pub const BIOS_END: usize = BIOS_START + BIOS_SIZE;

pub const EXT_SIZE: usize = 256 * 1024;
pub const EXT_START: usize = 0x0200_0000;
pub const EXT_END: usize = EXT_START + EXT_SIZE;

pub const RAM_SIZE: usize = 32 * 1024;
pub const RAM_START: usize = 0x0300_0000;
pub const RAM_END: usize = RAM_START + RAM_SIZE;

pub const IO_SIZE: usize = 1024;
pub const IO_START: usize = 0x0400_0000;
pub const IO_END: usize = IO_START + IO_SIZE;

pub const PALETTE_SIZE: usize = 1024;
pub const PALETTE_START: usize = 0x0500_0000;
pub const PALETTE_END: usize = PALETTE_START + PALETTE_SIZE;

pub const VRAM_SIZE: usize = 96 * 1024;
pub const VRAM_START: usize = 0x0600_0000;
pub const VRAM_END: usize = VRAM_START + VRAM_SIZE;

pub const OBJECT_ATTRIBUTE_SIZE: usize = 1024;
pub const OBJECT_ATTRIBUTE_START: usize = 0x0700_0000;
pub const OBJECT_ATTRIBUTE_END: usize = OBJECT_ATTRIBUTE_START + OBJECT_ATTRIBUTE_SIZE;

pub const ROM_SIZE: usize = 32 * 1024 * 1024;
pub const ROM_START: usize = 0x0800_0000;
pub const ROM_END: usize = ROM_START + ROM_SIZE;
pub const ROM_WAIT1_START: usize = 0x0a00_0000;
pub const ROM_WAIT1_END: usize = ROM_WAIT1_START + ROM_SIZE;
pub const ROM_WAIT2_START: usize = 0x0c00_0000;
pub const ROM_WAIT2_END: usize = ROM_WAIT2_START + ROM_SIZE;

pub const SAVE_SIZE: usize = 64 * 1024;
pub const SAVE_START: usize = 0x0e00_0000;
pub const SAVE_END: usize = SAVE_START + SAVE_SIZE;

pub struct Memory {
    /// Stores the BIOS of the Game Boy Advance, which is home to the software
    /// interupt table and some useful methods that there are not instructions for.
    pub bios: Vec<u8>,
    /// Links to the RAM made available by the external cartiridge.
    pub ext: Vec<u8>,
    /// Links to the RAM that is embedded into the CPU.
    pub ram: Vec<u8>,
    /// Address space for the memory mapped IO registers.
    pub io: Vec<u8>,
    /// Contains color palette information for the display modes that use palettes.
    pub palette: Vec<u8>,
    /// The video memory which contains background layer information or bitmaps
    /// depending on the display mode.
    pub vram: Vec<u8>,
    /// Object attribute memory, or a more familiar description might be
    /// sprite properties. Contains information relating to the object/sprite
    /// layer for layered display modes.
    pub object: Vec<u8>,
    /// The ROM memory of the currently inserted cartridge.
    pub rom: Vec<u8>,
    /// The writtable save memory inside of the cartridge is mappen here. The
    /// contents of this memory are copied out exactly as is when creating a
    /// save state.
    pub save: Vec<u8>,
}

impl Memory {
    pub fn init() -> Self {
        let mut memory = Self {
            bios: vec![0; BIOS_SIZE],
            ext: vec![0; EXT_SIZE],
            ram: vec![0; RAM_SIZE],
            io: vec![0; IO_SIZE],
            palette: vec![0; PALETTE_SIZE],
            vram: vec![0; VRAM_SIZE],
            object: vec![0; OBJECT_ATTRIBUTE_SIZE],
            rom: vec![0; 1],
            save: vec![0; SAVE_SIZE],
        };

        // Copy the BIOS into memory
        for each in 0..BIOS.len() {
            memory.bios[each] = BIOS[each];
        }

        memory
    }

    pub fn init_small_no_bios() -> Self {
        Self {
            bios: vec![0; 32],
            ext: vec![0; 32],
            ram: vec![0; 32],
            io: vec![0; 32],
            palette: vec![0; 32],
            vram: vec![0; 32],
            object: vec![0; 32],
            rom: vec![0; 1],
            save: vec![0; 32],
        }
    }

    pub fn read_word(&self, address: u32) -> u32 {
        assert_eq!(address % 4, 0);

        let mut accumulator = u32::from(self.read_byte(address));
        for each in 1..4 {
            accumulator += u32::from(self.read_byte(address + each)) << each * 8;
        }

        accumulator
    }

    pub fn write_word(&mut self, address: u32, value: u32) {
        assert_eq!(address % 4, 0);

        for each in 0..4 {
            self.write_byte(address + each, ((value >> each * 8) & 0xff) as u8);
        }
    }

    pub fn read_half_word(&self, address: u32) -> u16 {
        assert_eq!(address % 2, 0);

        u16::from(self.read_byte(address)) + (u16::from(self.read_byte(address + 1)) << 8)
    }

    pub fn write_half_word(&mut self, address: u32, value: u16) {
        assert_eq!(address % 2, 0);

        self.write_byte(address, (value & 0xff) as u8);
        self.write_byte(address + 1, (value >> 8 & 0xff) as u8);
    }

    pub fn read_byte(&self, address: u32) -> u8 {
        let i = address as usize;

        match i {
            BIOS_START..=BIOS_END => self.bios[i],
            EXT_START..=EXT_END => self.ext[i - EXT_START],
            RAM_START..=RAM_END => self.ram[i - RAM_START],
            IO_START..=IO_END => self.io[i - IO_START],
            PALETTE_START..=PALETTE_END => self.palette[i - PALETTE_START],
            VRAM_START..=VRAM_END => self.vram[i - VRAM_START],
            OBJECT_ATTRIBUTE_START..=OBJECT_ATTRIBUTE_END => {
                self.object[i - OBJECT_ATTRIBUTE_START]
            }
            ROM_START..=ROM_END => self.rom[i - ROM_START],
            ROM_WAIT1_START..=ROM_WAIT1_END => self.rom[i - ROM_WAIT1_START],
            ROM_WAIT2_START..=ROM_WAIT2_END => self.rom[i - ROM_WAIT2_START],
            SAVE_START..=SAVE_END => self.save[i - SAVE_START],
            _ => 0,
        }
    }

    pub fn write_byte(&mut self, address: u32, value: u8) {
        let i = address as usize;

        match i {
            // Note that BIOS is intentionally missing.
            EXT_START..=EXT_END => self.ext[i - EXT_START] = value,
            RAM_START..=RAM_END => self.ram[i - RAM_START] = value,
            IO_START..=IO_END => self.io[i - IO_START] = value,
            PALETTE_START..=PALETTE_END => {
                self.palette[i - PALETTE_START] = value
            }
            VRAM_START..=VRAM_END => self.vram[i - VRAM_START] = value,
            OBJECT_ATTRIBUTE_START..=OBJECT_ATTRIBUTE_END => {
                self.object[i - OBJECT_ATTRIBUTE_START] = value
            }
            ROM_START..=ROM_END => self.rom[i - ROM_START] = value,
            ROM_WAIT1_START..=ROM_WAIT1_END => self.rom[i - ROM_WAIT1_START] = value,
            ROM_WAIT2_START..=ROM_WAIT2_END => self.rom[i - ROM_WAIT2_START] = value,
            SAVE_START..=SAVE_END => self.save[i - SAVE_START] = value,
            _ => (),
        };
    }
}

pub const BIOS: [u8; 548] = [
    0x06, 0x00, 0x00, 0xea, 0xfe, 0xff, 0xff, 0xea, 0x0b, 0x00, 0x00, 0xea, 0xfe, 0xff, 0xff, 0xea,
    0xfe, 0xff, 0xff, 0xea, 0x00, 0x00, 0xa0, 0xe1, 0x2c, 0x00, 0x00, 0xea, 0xfe, 0xff, 0xff, 0xea,
    0x02, 0x03, 0xa0, 0xe3, 0x03, 0x10, 0xd0, 0xe5, 0xea, 0x00, 0x51, 0xe3, 0xec, 0x01, 0x9f, 0x15,
    0x10, 0xff, 0x2f, 0xe1, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf0, 0x29, 0xe1, 0x00, 0x00, 0x5d, 0xe3,
    0x01, 0xd3, 0xa0, 0x03, 0x20, 0xd0, 0x4d, 0x02, 0x00, 0x58, 0x2d, 0xe9, 0x02, 0xb0, 0x5e, 0xe5,
    0x9c, 0xc0, 0xa0, 0xe3, 0x0b, 0xb1, 0x9c, 0xe7, 0x00, 0x00, 0x5b, 0xe3, 0x00, 0xc0, 0x4f, 0xe1,
    0x00, 0x10, 0x2d, 0xe9, 0x80, 0xc0, 0x0c, 0xe2, 0x1f, 0xc0, 0x8c, 0xe3, 0x0c, 0xf0, 0x29, 0xe1,
    0x00, 0x40, 0x2d, 0xe9, 0x0f, 0xe0, 0xa0, 0xe1, 0x1b, 0xff, 0x2f, 0x11, 0x00, 0x40, 0xbd, 0xe8,
    0x93, 0xf0, 0x29, 0xe3, 0x00, 0x10, 0xbd, 0xe8, 0x0c, 0xf0, 0x69, 0xe1, 0x00, 0x58, 0xbd, 0xe8,
    0x0e, 0xf0, 0xb0, 0xe1, 0x00, 0x00, 0x00, 0x00, 0x04, 0x20, 0xa0, 0xe3, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf8, 0x00, 0x00, 0x00,
    0xf0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x01, 0x00, 0x00, 0xc8, 0x01, 0x00, 0x00,
    0x0f, 0x50, 0x2d, 0xe9, 0x01, 0x03, 0xa0, 0xe3, 0x00, 0xe0, 0x8f, 0xe2, 0x04, 0xf0, 0x10, 0xe5,
    0x0f, 0x50, 0xbd, 0xe8, 0x04, 0xf0, 0x5e, 0xe2, 0x00, 0x00, 0x00, 0x00, 0x02, 0xc0, 0x5e, 0xe5,
    0x01, 0x00, 0xa0, 0xe3, 0x01, 0x10, 0xa0, 0xe3, 0x0c, 0x40, 0x2d, 0xe9, 0x01, 0xc3, 0xa0, 0xe3,
    0x00, 0x00, 0x50, 0xe3, 0x00, 0x00, 0xa0, 0xe3, 0x01, 0x20, 0xa0, 0xe3, 0x03, 0x00, 0x00, 0x0a,
    0xb8, 0x30, 0x5c, 0xe1, 0x01, 0x30, 0xc3, 0xe1, 0xb8, 0x30, 0x4c, 0xe1, 0x01, 0x03, 0xcc, 0xe5,
    0x08, 0x02, 0xcc, 0xe5, 0xb8, 0x30, 0x5c, 0xe1, 0x01, 0x30, 0x13, 0xe0, 0x01, 0x30, 0x23, 0x10,
    0xb8, 0x30, 0x4c, 0x11, 0x08, 0x22, 0xcc, 0xe5, 0xf7, 0xff, 0xff, 0x0a, 0x0c, 0x80, 0xbd, 0xe8,
    0x00, 0x40, 0x2d, 0xe9, 0x02, 0x36, 0xa0, 0xe1, 0x01, 0x04, 0x12, 0xe3, 0x0f, 0x00, 0x00, 0x0a,
    0x01, 0x03, 0x12, 0xe3, 0x05, 0x00, 0x00, 0x0a, 0x23, 0x35, 0x81, 0xe0, 0x04, 0x00, 0xb0, 0xe8,
    0x03, 0x00, 0x51, 0xe1, 0x04, 0x00, 0xa1, 0xb8, 0xfc, 0xff, 0xff, 0xba, 0x14, 0x00, 0x00, 0xea,
    0x01, 0x00, 0xc0, 0xe3, 0x01, 0x10, 0xc1, 0xe3, 0xa3, 0x35, 0x81, 0xe0, 0xb0, 0x20, 0xd0, 0xe1,
    0x03, 0x00, 0x51, 0xe1, 0xb2, 0x20, 0xc1, 0xb0, 0xfc, 0xff, 0xff, 0xba, 0x0c, 0x00, 0x00, 0xea,
    0x01, 0x03, 0x12, 0xe3, 0x05, 0x00, 0x00, 0x0a, 0x23, 0x35, 0x81, 0xe0, 0x03, 0x00, 0x51, 0xe1,
    0x04, 0x00, 0xb0, 0xb8, 0x04, 0x00, 0xa1, 0xb8, 0xfb, 0xff, 0xff, 0xba, 0x04, 0x00, 0x00, 0xea,
    0xa3, 0x35, 0x81, 0xe0, 0x03, 0x00, 0x51, 0xe1, 0xb2, 0x20, 0xd0, 0xb0, 0xb2, 0x20, 0xc1, 0xb0,
    0xfb, 0xff, 0xff, 0xba, 0x00, 0x80, 0xbd, 0xe8, 0xf0, 0x47, 0x2d, 0xe9, 0x01, 0x04, 0x12, 0xe3,
    0x02, 0x36, 0xa0, 0xe1, 0x23, 0x25, 0x81, 0xe0, 0x0b, 0x00, 0x00, 0x0a, 0x00, 0x30, 0x90, 0xe5,
    0x03, 0x40, 0xa0, 0xe1, 0x03, 0x50, 0xa0, 0xe1, 0x03, 0x60, 0xa0, 0xe1, 0x03, 0x70, 0xa0, 0xe1,
    0x03, 0x80, 0xa0, 0xe1, 0x03, 0x90, 0xa0, 0xe1, 0x03, 0xa0, 0xa0, 0xe1, 0x02, 0x00, 0x51, 0xe1,
    0xf8, 0x07, 0xa1, 0xb8, 0xfc, 0xff, 0xff, 0xba, 0x03, 0x00, 0x00, 0xea, 0x02, 0x00, 0x51, 0xe1,
    0xf8, 0x07, 0xb0, 0xb8, 0xf8, 0x07, 0xa1, 0xb8, 0xfb, 0xff, 0xff, 0xba, 0xf0, 0x87, 0xbd, 0xe8,
    0xc0, 0x00, 0x00, 0x02,
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cant_write_to_bios() {
        // We have to declare it as mutable so that write_word can borrow it
        // as mutable, even though we shouldn't actually mutate it.
        let mut memory = Memory::init();

        assert_eq!(memory.read_word(0), 0xea000006);
        memory.write_word(0, 0xdeadbeef);
        assert_eq!(memory.read_word(0), 0xea000006);
    }

    #[test]
    fn write_to_ram() {
        let mut memory = Memory::init();

        // Write into interal ram
        let offset = RAM_START as u32;
        memory.write_word(offset, 0x01020304);

        // Test that it is stored correctly
        assert_eq!(memory.ram[0], 4);
        assert_eq!(memory.ram[1], 3);
        assert_eq!(memory.ram[2], 2);
        assert_eq!(memory.ram[3], 1);
    }

    #[test]
    fn read_from_ram() {
        let mut memory = Memory::init();

        // Write into interal ram
        let offset = RAM_START as u32;
        memory.write_word(offset, 0x01020304);

        // Test that we read it out properly
        assert_eq!(0x01020304, memory.read_word(offset));
        assert_eq!(0x0304, memory.read_half_word(offset));
        assert_eq!(0x0102, memory.read_half_word(offset + 2));
        assert_eq!(0x04, memory.read_byte(offset));
        assert_eq!(0x03, memory.read_byte(offset + 1));
    }

    #[test]
    fn write_half_word_to_vram() {
        let mut memory = Memory::init();

        // Initialize memory and set a red pixel
        let offset = VRAM_START as u32;
        memory.write_half_word(offset, 0x001f);

        // Make sure that the red pixel has the correct value
        assert_eq!(memory.read_half_word(offset), 0x001f);
    }
}

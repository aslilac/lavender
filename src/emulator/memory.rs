#[macro_use]
use crate::log;

pub struct Memory {
    // ALE [32:31]
    // Address [31:0]
    // ABE [0:]
    bios: Vec<u8>,
    ext: Vec<u8>,
    ram: Vec<u8>,
    pub io: Vec<u8>,
    palette: Vec<u8>,
    pub vram: Vec<u8>,
    object: Vec<u8>,
    pub rom: Vec<u8>,
    save: Vec<u8>,
}

impl Memory {
    // These are the actual sizes, but WebAssembly doesn't allocate enough
    // memory by default, so we need to figure out how to allocate more within
    // Webpack/Parcel/just say fuck it and not use either of them.
    const BIOS_START: usize = 0x0000_0000;
    const BIOS_SIZE: usize = 16 * 1024;
    const BIOS_END: usize = Self::BIOS_START + Self::BIOS_SIZE;

    const EXT_START: usize = 0x0200_0000;
    const EXT_SIZE: usize = 256 * 1024;
    const EXT_END: usize = Self::EXT_START + Self::EXT_SIZE;

    const RAM_START: usize = 0x0300_0000;
    const RAM_SIZE: usize = 32 * 1024;
    const RAM_END: usize = Self::RAM_START + Self::RAM_SIZE;

    pub const IO_START: usize = 0x0400_0000;
    pub const IO_SIZE: usize = 1024;
    pub const IO_END: usize = Self::IO_START + Self::IO_SIZE;

    const PALETTE_START: usize = 0x0500_0000;
    const PALETTE_SIZE: usize = 1024;
    const PALETTE_END: usize = Self::PALETTE_START + Self::PALETTE_SIZE;

    pub const VRAM_START: usize = 0x0600_0000;
    pub const VRAM_SIZE: usize = 96 * 1024;
    pub const VRAM_END: usize = Self::VRAM_START + Self::VRAM_SIZE;

    const OBJECT_START: usize = 0x0700_0000;
    const OBJECT_SIZE: usize = 1024;
    const OBJECT_END: usize = Self::OBJECT_START + Self::OBJECT_SIZE;

    const ROM_START: usize = 0x0800_0000;
    const ROM_SIZE: usize = 32 * 1024 * 1024;
    const ROM_END: usize = Self::ROM_START + Self::ROM_SIZE;

    const SAVE_START: usize = 0x0e00_0000;
    const SAVE_SIZE: usize = 64 * 1024;
    const SAVE_END: usize = Self::SAVE_START + Self::SAVE_SIZE;

    pub fn init() -> Memory {
        // Pages are 64kb.
        // Memory (excluding the rom) is 8 pages.
        Memory {
            bios: vec![0; Self::BIOS_SIZE],
            ext: vec![0; Self::EXT_SIZE],
            ram: vec![0; Self::RAM_SIZE],
            io: vec![0; Self::IO_SIZE],
            palette: vec![0; Self::PALETTE_SIZE],
            vram: vec![0; Self::VRAM_SIZE],
            object: vec![0; Self::OBJECT_SIZE],
            rom: vec![0; 10],
            save: vec![0; Self::SAVE_SIZE],
        }
    }

    pub fn read_word(&self, address: u32) -> u32 {
        assert_eq!(address % 4, 0);

        let mut accumulator = self.read_byte(address) as u32;
        for each in 1..4 {
            accumulator += (self.read_byte(address + each) as u32) << each * 8;
        }

        accumulator
    }

    pub fn write_word(&mut self, address: u32, value: u32) {
        assert_eq!(address % 4, 0);

        for each in 0..4 {
            self.write_byte(address + each, ((value >> each * 8) & 0xff) as u8);
        }
    }

    pub fn read_word_be(&self, address: u32) -> u32 {
        assert_eq!(address % 4, 0);
        log!("Not implemented!");
        0
    }

    pub fn write_word_be(&mut self, address: u32, value: u32) {
        assert_eq!(address % 4, 0);
        log!("Not implemented!");
    }

    pub fn read_half_word(&self, address: u32) -> u16 {
        assert_eq!(address % 2, 0);

        self.read_byte(address) as u16 + ((self.read_byte(address + 1) as u16) << 8)
    }

    pub fn write_half_word(&mut self, address: u32, value: u16) {
        assert_eq!(address % 2, 0);

        self.write_byte(address, (value & 0xff) as u8);
        self.write_byte(address + 1, (value >> 8 & 0xff) as u8);
    }

    pub fn read_half_word_be(&self, address: u32) -> u16 {
        assert_eq!(address % 2, 0);
        log!("Not implemented!");
        0
    }

    pub fn write_half_word_be(&mut self, address: u32, value: u16) {
        assert_eq!(address % 2, 0);
        log!("Not implemented!");
    }

    pub fn read_byte(&self, address: u32) -> u8 {
        let i = address as usize;

        // log!("Reading from address {:x}", address);

        match i {
            Self::BIOS_START..=Self::BIOS_END => self.bios[i],
            Self::EXT_START..=Self::EXT_END => self.ext[i - Self::EXT_START],
            Self::RAM_START..=Self::RAM_END => self.ram[i - Self::RAM_START],
            Self::IO_START..=Self::IO_END => self.io[i - Self::IO_START],
            Self::PALETTE_START..=Self::PALETTE_END => self.palette[i - Self::PALETTE_START],
            Self::VRAM_START..=Self::VRAM_END => self.vram[i - Self::VRAM_START],
            Self::OBJECT_START..=Self::OBJECT_END => self.object[i - Self::OBJECT_START],
            Self::ROM_START..=Self::ROM_END => self.rom[i - Self::ROM_START],
            Self::SAVE_START..=Self::SAVE_END => self.save[i - Self::SAVE_START],
            _ => 0
        }
    }

    pub fn write_byte(&mut self, address: u32, value: u8) {
        let i = address as usize;

        // log!("Writing to address {:x} value {:x}", address, value);

        match i {
            Self::BIOS_START..=Self::BIOS_END => self.bios[i] = value,
            Self::EXT_START..=Self::EXT_END => self.ext[i - Self::EXT_START] = value,
            Self::RAM_START..=Self::RAM_END => self.ram[i - Self::RAM_START] = value,
            Self::IO_START..=Self::IO_END => self.io[i - Self::IO_START] = value,
            Self::PALETTE_START..=Self::PALETTE_END => self.palette[i - Self::PALETTE_START] = value,
            Self::VRAM_START..=Self::VRAM_END => self.vram[i - Self::VRAM_START] = value,
            Self::OBJECT_START..=Self::OBJECT_END => self.object[i - Self::OBJECT_START] = value,
            Self::ROM_START..=Self::ROM_END => self.rom[i - Self::ROM_START] = value,
            Self::SAVE_START..=Self::SAVE_END => self.save[i - Self::SAVE_START] = value,
            _ => ()
        };
    }
}

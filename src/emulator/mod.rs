mod armv4t;
pub mod cpu;
mod io;
pub mod memory;

#[macro_use]
use crate::log;

use armv4t::{arm, thumb};
use cpu::*;
use memory::*;
// use std::{
//     time::Duration,
//     thread::sleep
// };

pub struct Emulator {
    pub cpu: Arm7Tdmi,
    pub memory: Memory,
}

impl Emulator {
    pub fn new() -> Self {
        Self {
            cpu: Arm7Tdmi::init(),
            memory: Memory::init(),
        }
    }

    pub fn start(rom: &[u8]) -> Self {
        let mut emulator = Self::new();
        emulator.load_rom(&rom);
        emulator.cpu.set_frequency(16_780_000); // 16.78MHz
        emulator
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        self.memory.rom = rom.to_vec();
    }

    pub fn step(&mut self) {
        if self.cpu.get_thumb_bit() {
            thumb::process_instruction(self, self.memory.read_half_word(self.cpu.registers.r15));

            // Increment the program counter by a half word
            self.cpu.registers.r15 += 2;
        } else {
            arm::process_instruction(self, self.memory.read_word(self.cpu.registers.r15));

            // Increment the program counter by a word
            self.cpu.registers.r15 += 4;
        }

        // WebAssembly *hates* this, so idk how we'll handle timing.
        // sleep(Duration::from_nanos(59)
    }

    pub fn test(&mut self) {
        use Arm7RegisterNames::*;

        // Addition test
        {
            // Initialize r3 as the accumulator and r4 as the amount to add
            self.cpu.set_register_value(r3, 0);
            self.cpu.set_register_value(r4, 1);

            // Run a basic (arm) adding instruction in a loop
            for _ in 0..10 {
                // ADD r3, r3, r4 (or something like that I think)
                arm::process_instruction(self, 0b1110_00_1_0100_1_0011_0011_0100_00000000);
            }

            // Assert that the adding completed correctly
            assert_eq!(self.cpu.get_register_value(r3), 10);
        }

        // Test branch instruction decoding
        // Link with distance of 0
        arm::process_instruction(self, 0b1110_101_1_0000_0000_0000_0000_0000_0000);
        arm::process_instruction(self, 0b1110_101_1_0111_1111_1111_1111_1111_1111);

        log!("Before subtracting 4 from pc: {}", self.cpu.registers.r15);
        arm::process_instruction(self, 0b1110_101_1_1111_1111_1111_1111_1111_1111);
        log!("After subtracting 4 from pc: {}", self.cpu.registers.r15);
        // arm::process_instruction(self, 0b1110_101_1_1000_0000_0000_0000_0000_0000);

        let i: i32 = 1;
        log!("i has a value {}", i << 31);

        // Test thumb (??????) instruction decoding
        thumb::process_instruction(self, 0b0000_0000_0000_0000);

        // Make sure the rom is correctly loaded into memory
        assert_eq!(self.memory.read_byte(0x0800_0000), self.memory.rom[0]);

        // Start stepping through instructions. This probably isn't how we
        // actually want to do it though.
        self.step();

        // Directly manipulate the screen start and vram so that our renderer
        // will attempt to draw an image.
        {
            // Set display mode to bitmap
            self.memory.write_half_word(0x0400_0000, 0x0403);

            // Write a few test pixels into vram
            self.memory
                .write_half_word(0x0600_0000 + (120 + 80 * 240) * 2, 0x001F);
            self.memory
                .write_half_word(0x0600_0000 + (136 + 80 * 240) * 2, 0x03E0);
            self.memory
                .write_half_word(0x0600_0000 + (120 + 96 * 240) * 2, 0x7C00);

            // Make sure that the red pixel has the correct value (the rest probably do to)
            assert_eq!(
                self.memory
                    .read_half_word(0x0600_0000 + (120 + 80 * 240) * 2),
                31
            );
        }

        log!("Emulator started successfully!");
    }
}

pub mod armv4t;
pub mod cpu;
mod io;
pub mod memory;

#[macro_use]
use crate::log;

use armv4t::{arm, thumb};
use cpu::*;
use memory::*;

pub struct Emulator {
    pub cpu: Arm7Tdmi,
    pub memory: Memory,

    /// Used to keep track of how much more the emulator should do before
    /// updating the screen. When this reaches zero, the emulator will pause
    /// execution until the next `requestAnimationFrame` delegation.
    pub remaining_cycles: u32,
}

impl Emulator {
    pub fn new() -> Self {
        Self {
            cpu: Arm7Tdmi::init(),
            memory: Memory::init(),
            remaining_cycles: 0,
        }
    }

    pub fn dummy() -> Self {
        Self {
            cpu: Arm7Tdmi::init(),
            memory: Memory::init_small_no_bios(),
            remaining_cycles: 0,
        }
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        self.memory.rom = rom.to_vec();
    }

    pub fn step_frame(&mut self) {
        // 16.78MHz CPU clock speed / 60Hz display refresh rate = 279,666 CPU cycles
        self.remaining_cycles += 279_666;

        while self.remaining_cycles > 0 {
            let cycles_used =
                arm::process_instruction(self, 0b1110_00_1_0100_1_0011_0011_0000_00000001);
            self.remaining_cycles = self.remaining_cycles.saturating_sub(cycles_used);
            // log!("{} cycles remaining", self.remaining_cycles);
        }

        // log!("Did a frame! r3 has value of {}", self.cpu.get_register_value(r3));
    }

    pub fn test(&mut self) {
        // Set display mode to bitmap
        self.memory.write_half_word(0x0400_0000, 0x0403);
        let point = |x, y: u32| 0x0600_0000 + (x + y * 240) * 2;

        // Write a few test pixels into vram
        self.memory.write_half_word(point(120, 80), 0x03ff);
        self.memory.write_half_word(point(136, 80), 0x7c16);
        self.memory.write_half_word(point(120, 96), 0x4fe3);

        log!("Emulator started successfully!");
    }
}

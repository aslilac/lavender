mod armv4t;
mod bios;
pub mod cpu;
pub mod memory;

#[macro_use]
use crate::log;

use armv4t::{arm, thumb};
use cpu::*;
use memory::*;

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
        emulator.cpu.set_frequency( 16780000 ); // 16.78MHz
        emulator
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        self.memory.rom = rom.to_vec();
    }

    pub fn step(&mut self) {
        let instruction = self.memory.read_word(self.cpu.registers.r15);
        log!("Instruction looks like {:b}", instruction);
        self.cpu.registers.r15 += 4;
    }

    pub fn test(&mut self) {
        use Arm7OperationModes::*;
        use Arm7RegisterNames::*;

        // Switch into UND mode
        self.cpu.set_operation_mode(UND);
        assert_eq!(self.cpu.get_operation_mode(), Some(UND));

        // Test some registers
        self.cpu.set_register_value(r13, 0xdeadbeef);
        assert_eq!(self.cpu.get_register_value(r0), 0);
        assert_eq!(self.cpu.get_register_value(r5), 5);
        assert_eq!(self.cpu.get_register_value(r13), 0xdeadbeef);

        // Test the thumb bit
        assert_eq!(self.cpu.get_thumb_bit(), false);
        self.cpu.set_thumb_bit(true);
        assert_eq!(self.cpu.get_thumb_bit(), true);
        self.cpu.set_thumb_bit(false);
        assert_eq!(self.cpu.get_thumb_bit(), false);

        // Test the interupt disable bits
        assert!(self.cpu.is_fiq_disabled());
        assert!(self.cpu.is_irq_disabled());

        // Change into USR mode
        self.cpu.set_operation_mode(USR);
        assert_eq!(self.cpu.get_operation_mode(), Some(USR));

        // Now that we are no longer is a priveledged mode we should get 0 from this
        assert_eq!(
            self.cpu.get_register_value(r13),
            0
        );

        // Write into interal ram
        let offset = 0x0300_0000;
        self.memory.write_word(offset, 0x01020304);

        // Test that we read it out properly
        assert_eq!(0x01020304, self.memory.read_word(offset));
        assert_eq!(0x0304, self.memory.read_half_word(offset));
        assert_eq!(0x0102, self.memory.read_half_word(offset + 2));
        assert_eq!(0x04, self.memory.read_byte(offset));
        assert_eq!(0x03, self.memory.read_byte(offset + 1));

        // Turn on the z bit
        self.cpu.registers.cpsr = self.cpu.registers.cpsr | 0x40000000;

        // Check that condition codes are (hopefully) all working
        // TODO: Probably add more tests.
        assert!(self.cpu.check_condition(Arm7ConditionCodes::EQ));
        assert!(!self.cpu.check_condition(Arm7ConditionCodes::NE));

        // Run a basic adding instruction in a loop
        self.cpu.set_register_value(r3, 0);
        self.cpu.set_register_value(r4, 1);

        for _ in 0..10 {
            arm::process_data_processing_instruction(
                &mut self.cpu,
                // ADD r3, r3, r4
                0b1110_00_1_0100_1_0011_0011_0100_00000000
            );
        };

        assert_eq!(self.cpu.get_register_value(r3), 10);

        // Make sure the rom is correctly loaded into memory
        assert_eq!(self.memory.read_byte(0x0800_0000), self.memory.rom[0]);

        self.step();

        // Set display mode to bitmap
        self.memory.write_half_word(0x0400_0000, 0x0403);

        // Write a few test pixels into vram
        self.memory.write_half_word(0x0600_0000 + (120 + 80 * 240) * 2, 0x001F);
        self.memory.write_half_word(0x0600_0000 + (136 + 80 * 240) * 2, 0x03E0);
        self.memory.write_half_word(0x0600_0000 + (120 + 96 * 240) * 2, 0x7C00);

        assert_eq!(self.memory.read_half_word(0x0600_0000 + (120 + 80 * 240) * 2), 31);

        log!("Emulator started successfully!");
    }
}

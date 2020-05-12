use crate::emulator::{cpu::*, Emulator};
use std::convert::TryFrom;

pub fn process_shifter_operand(emulator: &mut Emulator, instruction: u32) -> u32 {
    let is_immediate_value = instruction >> 25 & 1 > 0;

    if is_immediate_value {
        // Get the shift amount and the value from the instruction
        let rotate = (instruction >> 8 & 0xf) * 2;
        (instruction & 0xff).rotate_right(rotate)
    } else {
        // Determine what shifting mode will be used
        // 00: LSL Logical shift left
        // 01: LSR Logical shift right
        // 10: ASR Arithmetic shift right (sign extending)
        // 11: ROR Rotate right
        // 11, but with 0 for shift value: RRX Shift right 1 and extend.
        let shift_mode = instruction >> 5 & 3;
        // Determine if we need to fetch the shift amount from the register
        let is_register_shift = instruction >> 4 & 1 > 0;
        // Get the value from the register
        let value = emulator
            .cpu
            .get_register_value(RegisterNames::try_from(instruction & 15).unwrap());

        let shift = if is_register_shift {
            // Check to make sure that extension space instructions don't end
            // up here somehow. That is unpredictable behavior.
            let extension_space_identifier = instruction >> 7 & 1;
            assert_eq!(
                extension_space_identifier,
                0,
                "'Multiplies' extension space instructions should not enter process_shifter_operand"
            );

            // Anything above the bottom 8 bits should be ignored (because they
            // wouldn't matter anyway)
            0xff & emulator
                .cpu
                .get_register_value(RegisterNames::try_from(instruction >> 8 & 15).unwrap())
        } else {
            instruction >> 7 & 0x1f
        };

        match (shift_mode, shift) {
            (0, 0) => value,
            (0, _) => value << shift,
            (1, _) => value >> shift,
            (2, _) => value,
            (3, 0) => (if emulator.cpu.get_c() { 1 << 31 } else { 0 }) | (value >> 1),
            (3, _) => value.rotate_right(shift),
            (_, _) => panic!("Shift mode not matched for shifter_operand."),
        }
    }
}

pub fn process_addressing_mode(emulator: &mut Emulator, instruction: u32) -> (u32, AddressingType) {
    let is_immediate_value = instruction >> 25 & 1 == 0;

    let post_index_addressing = instruction >> 24 & 1 == 0;

    // In post indexed addressing mode this would indicate that we're dealing with a T version of
    // the instruction (with Translation). These are usually used in privileged mode to emulate
    // user mode memory accesses. If that's the case then we can just ignore it.
    // In pre indexed mode this indicates whether we'll write the calculated memory address back to
    // the base register.
    let write_address_to_base_register = instruction >> 21 & 1 == 1;

    let base_register = RegisterNames::try_from(instruction >> 16 & 0xf).unwrap();
    let base_register_value = emulator.cpu.get_register_value(base_register);

    let add_offset = instruction >> 23 & 1 == 1;

    let offset = if is_immediate_value {
        instruction & 0xfff
    } else {
        let shift_mode = instruction >> 5 & 0x3;
        let register_value = emulator
            .cpu
            .get_register_value(RegisterNames::try_from(instruction & 0xf).unwrap());

        let shift_imm = instruction >> 7 & 0x1f;

        match (shift_mode, shift_imm) {
            (0, 0) => register_value,
            (0, _) => register_value << shift_imm,
            (1, 0) => 0,
            (1, _) => register_value >> shift_imm,
            (2, 0) => {
                (if register_value & 0x8000_0000 == 0x8000_0000 {
                    0xFFFF_FFFF
                } else {
                    0x0
                })
            }
            // Shift operations on signed integers always perform an arithmetic shift in Rust
            (2, _) => ((register_value as i32) >> shift_imm) as u32,
            (3, 0) => (if emulator.cpu.get_c() { 1 << 31 } else { 0 }) | (register_value >> 1),
            (3, _) => register_value.rotate_right(shift_imm),
            (_, _) => panic!("Shift mode not matched for shifter_operand."),
        }
    };

    let (address, _) = if add_offset {
        base_register_value.overflowing_add(offset)
    } else {
        base_register_value.overflowing_sub(offset)
    };

    if post_index_addressing {
        let temporary = emulator.cpu.get_register_value(base_register);

        // Write the calculated address back into the base register
        emulator.cpu.set_register_value(base_register, address);

        (temporary, AddressingType::PostIndexed)
    } else {
        let addressing_type = if write_address_to_base_register {
            // This should only run if the instruction condition passes, but we should never be here if
            // it doesn't so.. don't do anything special
            emulator.cpu.set_register_value(base_register, address);

            AddressingType::PreIndexed
        } else {
            AddressingType::Offset
        };

        (address, addressing_type)
    }
}

pub fn process_misc_addressing_mode(
    emulator: &mut Emulator,
    instruction: u32,
) -> (u32, AddressingType) {
    let immediate_offset = instruction >> 22 & 0x1 == 0x1;
    let add_offset = instruction >> 23 & 0x1 == 0x1;
    let post_index_addressing = instruction >> 24 & 0x1 == 0x0;

    let offset = if immediate_offset {
        let immediate_high = instruction >> 8 & 0xf;
        let immediate_low = instruction & 0xf;

        (immediate_high << 4) | immediate_low
    } else {
        emulator
            .cpu
            .get_register_value(RegisterNames::try_from(instruction & 0xf).unwrap())
    };

    let base_register = RegisterNames::try_from(instruction >> 16 & 0xf).unwrap();
    let base_register_value = emulator.cpu.get_register_value(base_register);

    let (address, _) = if add_offset {
        base_register_value.overflowing_add(offset)
    } else {
        base_register_value.overflowing_sub(offset)
    };

    if post_index_addressing {
        emulator.cpu.set_register_value(base_register, address);

        (base_register_value, AddressingType::PostIndexed)
    } else {
        let write_address_to_base_register = instruction >> 21 & 0x1 == 0x1;

        let addressing_type = if write_address_to_base_register {
            emulator.cpu.set_register_value(base_register, address);
            AddressingType::PreIndexed
        } else {
            AddressingType::Offset
        };

        (address, addressing_type)
    }
}

pub fn get_data_processing_operands(
    emulator: &mut Emulator,
    instruction: u32,
) -> (RegisterNames, u32, u32) {
    let destination_register = RegisterNames::try_from(instruction >> 12 & 0xf).unwrap();
    let operand_register_value = emulator
        .cpu
        .get_register_value(RegisterNames::try_from(instruction >> 16 & 0xf).unwrap());
    let shifter_operand_value = process_shifter_operand(emulator, instruction);

    (
        destination_register,
        operand_register_value,
        shifter_operand_value,
    )
}

/// Common addressing types
#[derive(Debug, PartialEq)]
pub enum AddressingType {
    Offset,
    PreIndexed,
    PostIndexed,
}

#[macro_use]
use crate::{
    emulator::{
        cpu::{Arm7OperationModes::*, Arm7RegisterNames::*, *},
        Emulator,
    },
    log,
};
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
        let value = emulator.cpu.get_register_value(
            Arm7RegisterNames::try_from(instruction & 15).unwrap()
        );

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
            0xff & emulator.cpu.get_register_value(
                Arm7RegisterNames::try_from(instruction >> 8 & 15).unwrap()
            )
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

pub fn get_data_processing_operands(
    emulator: &mut Emulator,
    instruction: u32
) -> (Arm7RegisterNames, u32, u32) {
    let destination_register = Arm7RegisterNames::try_from(instruction >> 12 & 0xf).unwrap();
    let operand_register_value = emulator.cpu.get_register_value(
        Arm7RegisterNames::try_from(instruction >> 16 & 0xf).unwrap()
    );
    let shifter_operand_value = process_shifter_operand(emulator, instruction);

    (destination_register, operand_register_value, shifter_operand_value)
}

use crate::emulator::Emulator;
use instructions::*;

/// Decodes and runs the instruction in the context of the given emulator.
pub fn process_instruction(emulator: &mut Emulator, instruction: u32) {
    decode_instruction(instruction)(emulator, instruction);
}

/// Decodes the instruction and returns the appropriate implementation.
pub fn decode_instruction(instruction: u32) -> fn(&mut Emulator, u32) {
    // [27:20] and [7:4] are the CPU's decode bits
    // The first onces we want to look at are the three bits [27:25]
    let category = instruction >> 25 & 7;

    match category {
        // Data processing immediate shift if opcode != 0b10xx && s == 1
        // Miscellaneous instructions (Figure A3-4)
        // Data processing register shift if opcode != 0b10xx && s == 1
        // Miscellaneous instructions (Figure A3-4)
        // Multiplies (Figure A3-3) and Extra load/stores (Figure A3-5)
        0b000 | 0b001 => {
            let opcode = instruction >> 21 & 0xf;
            let set_flags = instruction >> 20 & 1;
            let lower_decode_bits = instruction >> 4 & 0xf;

            match (opcode, set_flags, lower_decode_bits) {
                (_, 1, 0b1011) if category == 0 => ldrh,
                (_, 1, 0b1101) if category == 0 => ldrsb,
                (_, 1, 0b1111) if category == 0 => ldrsh,
                (_, 0, 0b1011) if category == 0 => strh,
                (0b0000, _, 0b1001) if category == 0 => mul,
                (0b0000, _, _) => and,
                (0b0001, _, 0b1001) if category == 0 => mla,
                (0b0001, _, _) => eor,
                (0b0010, _, _) => sub,
                (0b0011, _, _) => rsb,
                (0b0100, _, 0b1001) if category == 0 => umull,
                (0b0100, _, _) => add,
                (0b0101, _, 0b1001) if category == 0 => umlal,
                (0b0101, _, _) => adc,
                (0b0110, _, 0b1001) => smull,
                (0b0110, _, _) => sbc,
                (0b0111, _, 0b1001) if category == 0 => smlal,
                (0b0111, _, _) => rsc,
                (0b1000, 0, 0b1001) if category == 0 => swp,
                (0b1010, 0, 0b1001) if category == 0 => swpb,
                (0b1000, 0, _) | (0b1010, 0, _) if category == 0 => mrs,
                (0b1000, _, _) => tst,
                (0b1001, 0, 0b0001) if category == 0 => bx,
                (0b1001, 0, _) | (0b1011, 0, _) => msr,
                (0b1001, _, _) => teq,
                (0b1010, _, _) => cmp,
                (0b1011, _, _) => cmn,
                (0b1100, _, _) => or,
                (0b1101, _, _) => mov,
                (0b1110, _, _) => bic,
                (0b1111, _, _) => mvn,
                (_, _, _) => unreachable!(),
            }
        }
        // Load/store
        // This is stupid and backward from how the dp instructions differentiate
        // between immediates and register values.
        0b010 | 0b011 => {
            let n = instruction >> 22 & 1;
            let load = instruction >> 20 & 1;
            let t = instruction >> 24 & 1 == 0 && instruction >> 21 & 1 == 1;

            match (n, load, t) {
                (0, 0, true) => strt,
                (0, 0, false) => str,
                (1, 0, true) => strbt,
                (1, 0, false) => strb,
                (0, 1, true) => ldrt,
                (0, 1, false) => ldr,
                (1, 1, true) => ldrbt,
                (1, 1, false) => ldrb,
                (_, _, _) => unreachable!(),
            }
        }
        // Media instructions + architecturally undefined (Figure A3-2)
        // Architecturally undefined
        // Load/store multiple
        0b100 => {
            let n = instruction >> 22 & 1;
            let load = instruction >> 20 & 1;
            match (n, load) {
                (0, 0) => stm, // mode 1?
                (1, 0) => stm, // mode 2? plus 21 = 0
                (0, 1) => ldm, // mode 1?
                (1, 1) => ldm, // mode 2? plus 21 = 0 and 15 = 0
                // (1, 1) => ldm, // mode 3? plus 15 = 1
                (_, _) => unreachable!(),
            }
        }
        // Branch instructions
        0b101 => {
            let link = instruction >> 24 & 1 > 0;
            match link {
                false => b,
                true => bl,
            }
        }
        // Coprocessor load/store and double register transfers
        0b110 => {
            let load = instruction >> 20 & 1 > 0;
            match load {
                false => stc,
                true => ldc,
            }
        }
        // Coprocessor data processing
        // Coprocessor register transfers
        // Software interupt
        0b111 => {
            let coprocessor_or_swi = instruction >> 24 & 1;
            let direction = instruction >> 20 & 1;
            let coprocessor_mov = instruction >> 4 & 1;
            match (coprocessor_or_swi, direction, coprocessor_mov) {
                (0, _, 0) => cdp,
                (0, 0, 1) => mcr,
                (0, 1, 1) => mrc,
                (1, _, _) => swi,
                (_, _, _) => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
}

/// A module containing functions which implement all of the 32-bit ARM v4T
/// instructions.
pub mod instructions {
    use crate::emulator::{
        armv4t::utils::*,
        cpu::{Arm7RegisterNames::*, *},
        Emulator,
    };
    use std::convert::TryFrom;

    /// Addition that includes carry from the carry bit in the CPSR register.
    pub fn adc(emulator: &mut Emulator, instruction: u32) {
        let carry_amount = if emulator.cpu.get_c() { 1 } else { 0 };
        let should_update_flags = instruction >> 20 & 1 > 0;

        // Get the instruction operands
        let destination_register = Arm7RegisterNames::try_from(instruction >> 12 & 0xf).unwrap();
        let operand_register = Arm7RegisterNames::try_from(instruction >> 16 & 0xf).unwrap();
        let shifter_operand = process_shifter_operand(emulator, instruction);

        let (result, overflow) = emulator
            .cpu
            .get_register_value(operand_register)
            .overflowing_add(shifter_operand + carry_amount);

        // Update flags if necessary
        if should_update_flags {
            emulator.cpu.set_nzcv(
                result >> 31 & 1 > 0,
                if result == 0 { true } else { false },
                // xxx: one of these two is incorrect
                overflow, // c: an unsigned overflow occured
                overflow, // v: a signed overflow occured
            );
        }

        emulator
            .cpu
            .set_register_value(destination_register, result);
    }

    /// Addition
    pub fn add(emulator: &mut Emulator, instruction: u32) {
        let should_update_flags = instruction >> 20 & 1 > 0;

        // Get the instruction operands
        let (destination_register, operand_register_value, shifter_operand_value) =
            get_data_processing_operands(emulator, instruction);

        let (result, overflow) = operand_register_value.overflowing_add(shifter_operand_value);

        // Update flags if necessary
        if should_update_flags {
            if destination_register == r15 {
                emulator
                    .cpu
                    .set_register_value(cpsr, emulator.cpu.get_register_value(spsr));
            } else {
                emulator.cpu.set_nzcv(
                    result >> 31 & 1 > 0,
                    if result == 0 { true } else { false },
                    // xxx: one of these two is incorrect
                    overflow, // c: an unsigned overflow occured
                    overflow, // v: a signed overflow occured
                );
            }
        }

        emulator
            .cpu
            .set_register_value(destination_register, result);
    }

    /// Logical AND
    pub fn and(emulator: &mut Emulator, instruction: u32) {
        let should_update_flags = instruction >> 20 & 1 > 0;

        // Get the instruction operands
        let (destination_register, operand_register_value, shifter_operand_value) =
            get_data_processing_operands(emulator, instruction);

        let result = operand_register_value & shifter_operand_value;

        if should_update_flags {
            if destination_register == r15 {
                emulator
                    .cpu
                    .set_register_value(cpsr, emulator.cpu.get_register_value(spsr));
            } else {
                emulator.cpu.set_nzcv(
                    result >> 31 & 1 > 0,
                    if result == 0 { true } else { false },
                    false, // xxx: c: shifter_carry_out
                    false, // xxx: this actually shouldn't be mutated at all
                );
            }
        }

        emulator
            .cpu
            .set_register_value(destination_register, result);
    }

    /// Relative code branching by up 32MB in either direction.
    pub fn b(emulator: &mut Emulator, instruction: u32) {
        let pc_value = emulator.cpu.get_register_value(r15);
        let negative = instruction >> 23 & 1 > 0;

        // The shift amount is a 26 bit two's complement integer stored in 24 bits.
        // This is all just a very complicated way to convert it to the proper 32 bit
        // two's complement integer format. We still store it as an unsigned
        // integer because otherwise Rust won't let us add them together.
        let shift = if negative {
            instruction & 0x7fffff | 0x3f80_0000
        } else {
            instruction & 0x7fffff
        } << 2;

        emulator
            .cpu
            .set_register_value(r15, pc_value.wrapping_add(shift));
    }

    /// Equivalent to `a AND (NOT b)`
    pub fn bic(emulator: &mut Emulator, instruction: u32) {
        let should_update_flags = instruction >> 20 & 1 > 0;

        // Get the instruction operands
        let destination_register = Arm7RegisterNames::try_from(instruction >> 12 & 0xf).unwrap();
        let operand_register = Arm7RegisterNames::try_from(instruction >> 16 & 0xf).unwrap();
        let shifter_operand = process_shifter_operand(emulator, instruction);

        let result = emulator.cpu.get_register_value(operand_register) & !shifter_operand;

        if should_update_flags {
            if destination_register == r15 {
                emulator
                    .cpu
                    .set_register_value(cpsr, emulator.cpu.get_register_value(spsr));
            } else {
                emulator.cpu.set_nzcv(
                    result >> 31 & 1 > 0,
                    if result == 0 { true } else { false },
                    false, // xxx: c: shifter_carry_out
                    false, // xxx: this actually shouldn't be mutated at all
                );
            }
        }

        emulator
            .cpu
            .set_register_value(destination_register, result);
    }

    /// Linked relative code branching by up 32MB in either direction. Sets r14
    /// with an address to return to after execution.
    pub fn bl(emulator: &mut Emulator, instruction: u32) {
        let pc_value = emulator.cpu.get_register_value(r15);
        let negative = instruction >> 23 & 1 > 0;

        // The shift amount is a 26 bit two's complement integer stored in 24 bits.
        // This is all just a very complicated way to convert it to the proper 32 bit
        // two's complement integer format. We still store it as an unsigned
        // integer because otherwise Rust won't let us add them together.
        let shift = if negative {
            instruction & 0x7fffff | 0x3f80_0000
        } else {
            instruction & 0x7fffff
        } << 2;

        emulator.cpu.set_register_value(r14, pc_value);
        emulator
            .cpu
            .set_register_value(r15, pc_value.wrapping_add(shift));
    }

    /// Branches execution relative to the current program counter by up 32MB in
    /// either direction. Exchanges instruction set to Thumb at the new location.
    pub fn bx(_emulator: &mut Emulator, _instruction: u32) {}
    /// Coprocessor data processing
    pub fn cdp(_emulator: &mut Emulator, _instruction: u32) {}
    pub fn cmn(_emulator: &mut Emulator, _instruction: u32) {}
    pub fn cmp(_emulator: &mut Emulator, _instruction: u32) {}
    /// Logical XOR
    pub fn eor(_emulator: &mut Emulator, _instruction: u32) {}
    pub fn ldc(_emulator: &mut Emulator, _instruction: u32) {}
    pub fn ldm(_emulator: &mut Emulator, _instruction: u32) {}
    pub fn ldr(_emulator: &mut Emulator, _instruction: u32) {}
    pub fn ldrb(_emulator: &mut Emulator, _instruction: u32) {}
    pub fn ldrbt(_emulator: &mut Emulator, _instruction: u32) {}
    pub fn ldrh(_emulator: &mut Emulator, _instruction: u32) {}
    pub fn ldrsb(_emulator: &mut Emulator, _instruction: u32) {}
    pub fn ldrsh(_emulator: &mut Emulator, _instruction: u32) {}
    pub fn ldrt(_emulator: &mut Emulator, _instruction: u32) {}
    pub fn mcr(_emulator: &mut Emulator, _instruction: u32) {}
    pub fn mla(_emulator: &mut Emulator, _instruction: u32) {}
    pub fn mov(_emulator: &mut Emulator, _instruction: u32) {}
    pub fn mrc(_emulator: &mut Emulator, _instruction: u32) {}
    pub fn mrs(_emulator: &mut Emulator, _instruction: u32) {}
    pub fn msr(_emulator: &mut Emulator, _instruction: u32) {}
    pub fn mul(_emulator: &mut Emulator, _instruction: u32) {}
    pub fn mvn(_emulator: &mut Emulator, _instruction: u32) {}
    /// Logical OR (also referred to as the orr instruction)
    pub fn or(_emulator: &mut Emulator, _instruction: u32) {}
    pub fn rsb(_emulator: &mut Emulator, _instruction: u32) {}
    pub fn rsc(_emulator: &mut Emulator, _instruction: u32) {}
    pub fn sbc(_emulator: &mut Emulator, _instruction: u32) {}
    pub fn smlal(_emulator: &mut Emulator, _instruction: u32) {}
    pub fn smull(_emulator: &mut Emulator, _instruction: u32) {}
    pub fn stc(_emulator: &mut Emulator, _instruction: u32) {}
    pub fn stm(_emulator: &mut Emulator, _instruction: u32) {}
    pub fn str(_emulator: &mut Emulator, _instruction: u32) {}
    pub fn strb(_emulator: &mut Emulator, _instruction: u32) {}
    pub fn strbt(_emulator: &mut Emulator, _instruction: u32) {}
    pub fn strh(_emulator: &mut Emulator, _instruction: u32) {}
    pub fn strt(_emulator: &mut Emulator, _instruction: u32) {}
    pub fn sub(_emulator: &mut Emulator, _instruction: u32) {}
    /// Triggers an interupt vector from software. Usually used to make system
    /// calls into the BIOS.
    pub fn swi(_emulator: &mut Emulator, _instruction: u32) {}
    pub fn swp(_emulator: &mut Emulator, _instruction: u32) {}
    pub fn swpb(_emulator: &mut Emulator, _instruction: u32) {}
    pub fn teq(_emulator: &mut Emulator, _instruction: u32) {}
    pub fn tst(_emulator: &mut Emulator, _instruction: u32) {}
    pub fn umlal(_emulator: &mut Emulator, _instruction: u32) {}
    pub fn umull(_emulator: &mut Emulator, _instruction: u32) {}
}

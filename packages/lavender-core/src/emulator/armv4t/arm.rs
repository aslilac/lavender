use crate::emulator::{cpu::*, Emulator};
use instructions::*;
use std::convert::TryFrom;

/// Decodes and runs the instruction using the given emulator, and returns the
/// number of cycles used.
pub fn process_instruction(emulator: &mut Emulator, instruction: u32) -> u32 {
    // Check if the condition is met before executing the instruction.
    let condition = ConditionCodes::try_from(instruction >> 28 & 15).unwrap();
    if !emulator.cpu.check_condition(condition) {
        return 1;
    }

    decode_instruction(instruction)(emulator, instruction)
}

/// Decodes the instruction and returns the appropriate implementation.
pub fn decode_instruction(instruction: u32) -> fn(&mut Emulator, u32) -> u32 {
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
            let n = instruction >> 22 & 1 > 0;
            let load = instruction >> 20 & 1 > 0;
            let t = instruction >> 24 & 1 == 0 && instruction >> 21 & 1 > 0;

            match (n, load, t) {
                (false, false, true) => strt,
                (false, false, false) => str,
                (true, false, true) => strbt,
                (true, false, false) => strb,
                (false, true, true) => ldrt,
                (false, true, false) => ldr,
                (true, true, true) => ldrbt,
                (true, true, false) => ldrb,
            }
        }
        // Media instructions + architecturally undefined (Figure A3-2)
        // Architecturally undefined
        // Load/store multiple
        0b100 => {
            let load = instruction >> 20 & 1 > 0;
            match load {
                false => stm,
                true => ldm,
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
            let coprocessor_or_swi = instruction >> 24 & 1 > 0;
            let direction = instruction >> 20 & 1 > 0;
            let coprocessor_mov = instruction >> 4 & 1 > 0;
            match (coprocessor_or_swi, direction, coprocessor_mov) {
                (false, _, false) => cdp,
                (false, false, true) => mcr,
                (false, true, true) => mrc,
                (true, _, _) => swi,
            }
        }
        _ => unreachable!(),
    }
}

mod internal {
    use crate::emulator::{armv4t::utils::*, cpu::RegisterNames::*, cpu::*, Emulator};
    use std::convert::TryFrom;

    // Internal functions for reading and writing from/to memory "securely". There is nothing
    // secure about these because there is no permission checking for memory accesses on the GBA.
    // It's useful to have separate functions so that it's easier to add debug_assert's or extend
    // it them for a system with a memory protection unit.
    pub fn read_byte(emulator: &Emulator, address: u32) -> u8 {
        emulator.memory.read_byte(address)
    }

    pub fn read_half_word(emulator: &Emulator, address: u32) -> u16 {
        emulator.memory.read_half_word(address)
    }

    pub fn read_word(emulator: &Emulator, address: u32) -> u32 {
        emulator.memory.read_word(address)
    }

    pub fn write_byte(emulator: &mut Emulator, address: u32, value: u8) {
        emulator.memory.write_byte(address, value);
    }

    pub fn write_half_word(emulator: &mut Emulator, address: u32, value: u16) {
        emulator.memory.write_half_word(address, value);
    }

    pub fn write_word(emulator: &mut Emulator, address: u32, value: u32) {
        emulator.memory.write_word(address, value);
    }

    // Main bits of the load/store instructions, these are used in both the normal instructions and
    // in the with translation instructions.
    pub fn store_register(emulator: &mut Emulator, source_register: RegisterNames, address: u32) {
        let source_register_value = emulator.cpu.get_register_value(source_register);

        write_word(emulator, address & 0xFFFF_FFFC, source_register_value);
    }

    pub fn store_register_byte(
        emulator: &mut Emulator,
        source_register: RegisterNames,
        address: u32,
    ) {
        let source_register_value = emulator.cpu.get_register_value(source_register);

        write_byte(emulator, address, (source_register_value & 0xff) as u8);
    }

    pub fn load_register(
        emulator: &mut Emulator,
        destination_register: RegisterNames,
        address: u32,
    ) {
        let mut value = read_word(emulator, address & 0xFFFF_FFFC);

        // For ARMv5 and below, if the address is not word aligned, then the loaded value needs to
        // be rotated right by 8 times the value of the 2 LSB's of the address.
        let rotate = address & 0x3;
        if rotate > 0 {
            value = value.rotate_right(rotate * 8);
        }

        if destination_register == r15 {
            emulator.cpu.set_register_value(r15, value & 0xFFFF_FFFC);
        } else {
            emulator.cpu.set_register_value(destination_register, value);
        }
    }

    pub fn load_store_instruction_wrapper(
        emulator: &mut Emulator,
        instruction: u32,
        operation: fn(&mut Emulator, RegisterNames, u32),
    ) {
        let source_or_destination_register =
            RegisterNames::try_from(instruction >> 12 & 0xf).unwrap();

        debug_assert!(
            !(instruction >> 22 & 0x1 == 0x1
                && source_or_destination_register == r15),
            "Loads/stores: result unpredictable if source/destination register == r15 for byte operations");

        let (address, addressing_type) = process_addressing_mode(emulator, instruction);

        debug_assert!(
            !(addressing_type == AddressingType::PreIndexed
                && source_or_destination_register
                    == RegisterNames::try_from(instruction >> 16 & 0xf).unwrap()),
            "Loads/stores: result unpredictable if Rn == Rd and addressing type is PreIndexed"
        );

        operation(emulator, source_or_destination_register, address);
    }
}

/// A module containing functions which implement all of the 32-bit ARM v4T
/// instructions.
pub mod instructions {
    use crate::emulator::{
        armv4t::utils::*,
        cpu::{RegisterNames::*, *},
        Emulator,
    };
    use std::convert::TryFrom;

    /// Addition that includes carry from the carry bit in the CPSR register.
    pub fn adc(emulator: &mut Emulator, instruction: u32) -> u32 {
        let carry_amount = if emulator.cpu.get_c() { 1 } else { 0 };
        let should_update_flags = instruction >> 20 & 1 > 0;

        // Get the instruction operands
        let destination_register = RegisterNames::try_from(instruction >> 12 & 0xf).unwrap();
        let operand_register = RegisterNames::try_from(instruction >> 16 & 0xf).unwrap();
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

        // xxx: Return the actual number of cycles that the instruction should take
        5
    }

    /// Addition
    pub fn add(emulator: &mut Emulator, instruction: u32) -> u32 {
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

        // xxx: Return the actual number of cycles that the instruction should take
        5
    }

    /// Logical AND
    pub fn and(emulator: &mut Emulator, instruction: u32) -> u32 {
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

        // xxx: Return the actual number of cycles that the instruction should take
        5
    }

    /// Relative code branching by up 32MB in either direction.
    pub fn b(emulator: &mut Emulator, instruction: u32) -> u32 {
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

        // xxx: Return the actual number of cycles that the instruction should take
        5
    }

    /// Bit clear - Equivalent to `a AND (NOT b)`
    pub fn bic(emulator: &mut Emulator, instruction: u32) -> u32 {
        let should_update_flags = instruction >> 20 & 1 > 0;

        // Get the instruction operands
        let destination_register = RegisterNames::try_from(instruction >> 12 & 0xf).unwrap();
        let operand_register = RegisterNames::try_from(instruction >> 16 & 0xf).unwrap();
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

        // xxx: Return the actual number of cycles that the instruction should take
        5
    }

    /// Linked relative code branching by up 32MB in either direction. Sets r14
    /// with an address to return to after execution.
    pub fn bl(emulator: &mut Emulator, instruction: u32) -> u32 {
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

        // xxx: Return the actual number of cycles that the instruction should take
        5
    }

    /// Branches execution relative to the current program counter by up 32MB in
    /// either direction. Exchanges instruction set to Thumb at the new location.
    pub fn bx(_emulator: &mut Emulator, _instruction: u32) -> u32 {
        1
    }
    /// Coprocessor data processing
    pub fn cdp(_emulator: &mut Emulator, _instruction: u32) -> u32 {
        1
    }

    /// Compare negative
    pub fn cmn(_emulator: &mut Emulator, _instruction: u32) -> u32 {
        1
    }

    /// Compare
    pub fn cmp(_emulator: &mut Emulator, _instruction: u32) -> u32 {
        // alu_out = Rn - shifter_operand
        // N Flag = alu_out[31]
        // Z Flag = if alu_out == 0 then 1 else 0
        // C Flag = NOT BorrowFrom(Rn - shifter_operand) V Flag = OverflowFrom(Rn - shifter_operand)

        1
    }

    /// Logical XOR
    pub fn eor(emulator: &mut Emulator, instruction: u32) -> u32 {
        // Rd = Rn EOR shifter_operand
        // if S == 1 and Rd == R15 then
        //     if CurrentModeHasSPSR() then CPSR = SPSR
        //     else UNPREDICTABLE
        // else if S == 1 then
        //     N Flag = Rd[31]
        //     Z Flag = if Rd == 0 then 1 else 0
        //     C Flag = shifter_carry_out
        //     V Flag = unaffected

        let should_update_flags = instruction >> 20 & 1 > 0;

        // Get the instruction operands
        let (destination_register, operand_register_value, shifter_operand_value) =
            get_data_processing_operands(emulator, instruction);

        let result = operand_register_value ^ shifter_operand_value;

        if should_update_flags {
            // idk man
        }

        emulator
            .cpu
            .set_register_value(destination_register, result);

        5
    }

    /// Load coprocessor - Loads memory into a coprocessor
    pub fn ldc(_emulator: &mut Emulator, _instruction: u32) -> u32 {
        1
    }

    /// Load multiple
    pub fn ldm(_emulator: &mut Emulator, _instruction: u32) -> u32 {
        1
    }

    /// Load register
    pub fn ldr(emulator: &mut Emulator, instruction: u32) -> u32 {
        /*
        MemoryAccess(B-bit, E-bit)
        if ConditionPassed(cond) then
            if (CP15_reg1_Ubit == 0) then
                data = Memory[address,4] Rotate_Right (8 * address[1:0])
            else /* CP15_reg_Ubit == 1 */
                data = Memory[address,4]
            if (Rd is R15) then
                if (ARMv5 or above) then
                    PC = data AND 0xFFFFFFFE
                    T Bit = data[0]
                else
                    PC = data AND 0xFFFFFFFC
            else
                Rd = data
        */

        load_store_instruction_wrapper(emulator, instruction, load_register);

        5
    }

    pub fn ldrb(_emulator: &mut Emulator, _instruction: u32) -> u32 {
        1
    }
    pub fn ldrbt(_emulator: &mut Emulator, _instruction: u32) -> u32 {
        1
    }
    pub fn ldrh(_emulator: &mut Emulator, _instruction: u32) -> u32 {
        1
    }
    pub fn ldrsb(_emulator: &mut Emulator, _instruction: u32) -> u32 {
        1
    }
    pub fn ldrsh(_emulator: &mut Emulator, _instruction: u32) -> u32 {
        1
    }
    pub fn ldrt(_emulator: &mut Emulator, _instruction: u32) -> u32 {
        1
    }
    pub fn mcr(_emulator: &mut Emulator, _instruction: u32) -> u32 {
        1
    }
    pub fn mla(_emulator: &mut Emulator, _instruction: u32) -> u32 {
        1
    }
    pub fn mov(_emulator: &mut Emulator, _instruction: u32) -> u32 {
        1
    }
    pub fn mrc(_emulator: &mut Emulator, _instruction: u32) -> u32 {
        1
    }
    pub fn mrs(_emulator: &mut Emulator, _instruction: u32) -> u32 {
        1
    }
    pub fn msr(_emulator: &mut Emulator, _instruction: u32) -> u32 {
        1
    }
    pub fn mul(_emulator: &mut Emulator, _instruction: u32) -> u32 {
        1
    }
    pub fn mvn(_emulator: &mut Emulator, _instruction: u32) -> u32 {
        1
    }
    /// Logical OR (also referred to as the orr instruction)
    pub fn or(_emulator: &mut Emulator, _instruction: u32) -> u32 {
        1
    }
    pub fn rsb(_emulator: &mut Emulator, _instruction: u32) -> u32 {
        1
    }
    pub fn rsc(_emulator: &mut Emulator, _instruction: u32) -> u32 {
        1
    }
    pub fn sbc(_emulator: &mut Emulator, _instruction: u32) -> u32 {
        1
    }
    pub fn smlal(_emulator: &mut Emulator, _instruction: u32) -> u32 {
        1
    }
    pub fn smull(_emulator: &mut Emulator, _instruction: u32) -> u32 {
        1
    }
    pub fn stc(_emulator: &mut Emulator, _instruction: u32) -> u32 {
        1
    }
    pub fn stm(_emulator: &mut Emulator, _instruction: u32) -> u32 {
        1
    }

    /// Store register
    pub fn str(emulator: &mut Emulator, instruction: u32) -> u32 {
        /*
        MemoryAccess(B-bit, E-bit)
        processor_id = ExecutingProcessor()
        if ConditionPassed(cond) then
            Memory[address,4] = Rd
            if Shared(address) then /* from ARMv6 */
                physical_address = TLB(address)
                ClearExclusiveByAddress(physical_address,processor_id,4)
                /* See Summary of operation on page A2-49 */
        */

        // Probably don't need to worry about the processor ID etc. because there is only one
        // processor which interacts with the MMU (hopefully?). The ClearExclusiveByAddress is used
        // to clear any requests for exclusive access, from an address range, for all processors,
        // other than the one specified by processor ID. This is only valid for ARM architectures
        // from ARMv6 onwards.

        // MemoryAccess(B-bit, E-bit) defines the endian model (see Glossary-9).

        load_store_instruction_wrapper(emulator, instruction, store_register);

        1
    }

    /// Store register byte
    pub fn strb(emulator: &mut Emulator, instruction: u32) -> u32 {
        /*
        processor_id = ExecutingProcessor()
        if ConditionPassed(cond) then
            Memory[address,1] = Rd[7:0]
        */

        load_store_instruction_wrapper(emulator, instruction, store_register_byte);

        1
    }
    pub fn strbt(_emulator: &mut Emulator, _instruction: u32) -> u32 {
        1
    }
    pub fn strh(_emulator: &mut Emulator, _instruction: u32) -> u32 {
        1
    }
    pub fn strt(_emulator: &mut Emulator, _instruction: u32) -> u32 {
        1
    }
    pub fn sub(_emulator: &mut Emulator, _instruction: u32) -> u32 {
        1
    }
    /// Triggers an interupt vector from software. Usually used to make system
    /// calls into the BIOS.
    pub fn swi(_emulator: &mut Emulator, _instruction: u32) -> u32 {
        1
    }
    pub fn swp(_emulator: &mut Emulator, _instruction: u32) -> u32 {
        1
    }
    pub fn swpb(_emulator: &mut Emulator, _instruction: u32) -> u32 {
        1
    }
    pub fn teq(_emulator: &mut Emulator, _instruction: u32) -> u32 {
        1
    }
    pub fn tst(_emulator: &mut Emulator, _instruction: u32) -> u32 {
        1
    }
    pub fn umlal(_emulator: &mut Emulator, _instruction: u32) -> u32 {
        1
    }
    pub fn umull(_emulator: &mut Emulator, _instruction: u32) -> u32 {
        1
    }
}

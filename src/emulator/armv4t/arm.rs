#[macro_use]
use crate::log;

use crate::emulator::{
    cpu::{Arm7OperationModes::*, Arm7RegisterNames::*, *},
    Emulator,
};
use std::convert::TryFrom;

pub fn process_instruction(emulator: &mut Emulator, instruction: u32) {
    // [27:20] and [7:4] are the CPU's decode bits
    // The first onces we want to look at are the three bits [27:25]
    let category = instruction >> 25 & 7;

    let operation = match category {
        // Data processing immediate shift if opcode != 0b10xx && s == 1
        // Miscellaneous instructions (Figure A3-4)
        // Data processing register shift if opcode != 0b10xx && s == 1
        // Miscellaneous instructions (Figure A3-4)
        // Multiplies (Figure A3-3) and Extra load/stores (Figure A3-5)
        0b000 | 0b001 => {
            let opcode = instruction >> 21 & 7;
            let s = instruction >> 20 & 1;
            match (opcode, s) {
                (0b0000, _) => and,
                (0b0001, _) => eor,
                (0b0010, _) => sub,
                (0b0011, _) => rsb,
                (0b0100, _) => add,
                (0b0101, _) => adc,
                (0b0110, _) => sbc,
                (0b0111, _) => rsc,
                (0b1000..=0b1011, 0) => placeholder,
                (0b1000, _) => tst,
                (0b1001, _) => teq,
                (0b1010, _) => cmp,
                (0b1011, _) => cmn,
                (0b1100, _) => or,
                (0b1101, _) => mov,
                (0b1110, _) => bic,
                (0b1111, _) => mvn,
                (_, _) => placeholder,
            }
        },
        // Data processing immediate if opcode != 0b10xx && s == 1
        // Undefined instruction
        // Move immediate to status register
        // (0b001, 0b0101) => adc_immediate(emulator, instruction),
        // Load/store immediate offset
        0b010 => placeholder,
        // Load/store register offset
        // Media instructions + architecturally undefined (Figure A3-2)
        // Architecturally undefined
        0b011 => placeholder,
        // Load/store multiple
        0b100 => placeholder,
        0b101 => branch,
        // Coprocessor load/store and double register transfers
        0b110 => placeholder,
        // Coprocessor data processing
        // Coprocessor register transfers
        // Software interupt
        0b111 => placeholder,
        // Theoretically, this is impossible, but we don't have a way to tell the
        // compiler that, so we have to have the case here anyway.
        _ => placeholder,
    };

    operation(emulator, instruction);
}

/// Handles the b and bl instructions
pub fn branch(emulator: &mut Emulator, instruction: u32) {
    let pc = emulator.cpu.get_register_value(r15);
    // If this bit is set, then we need to set r14 = r15 before
    // actually branching.
    let link = instruction >> 24 & 1 > 0;
    // See if the number is negative or not.
    let negative = instruction >> 23 & 1 > 0;

    // The shift amount is a 24 bit two's complement integer. This is
    // all just a very complicated way to convert it to the proper 32 bit
    // two's complement integer format. We still store it as an unsigned
    // integer because otherwise Rust won't let us add them together.
    let shift = if negative {
        instruction & 0x7fffff | 0x3f80_0000
    } else {
        instruction & 0x7fffff
    } << 2;

    if link {
        // r15 is safe to access directly because it isn't branched,
        // but r14 is branched so we much use set_register_value
        emulator.cpu.set_register_value(r14, pc);
    }

    emulator.cpu.set_register_value(r15, pc.wrapping_add(shift));
}

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
                "Extension space instructions should not enter process_shifter_operand"
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

pub fn placeholder(_emulator: &mut Emulator, instruction: u32) {
    panic!("Received unsupported instruction {:x}", instruction);
}

pub fn adc(emulator: &mut Emulator, instruction: u32) {
    let carry_amount = if emulator.cpu.get_c() { 1 } else { 0 };
    let should_update_flags = instruction >> 20 & 1 > 0;

    // Get the instruction operands
    let destination_register = Arm7RegisterNames::try_from(instruction >> 12 & 0xf).unwrap();
    let operand_register = Arm7RegisterNames::try_from(instruction >> 16 & 0xf).unwrap();
    let shifter_operand = process_shifter_operand(emulator, instruction);

    let (result, overflow) =
        emulator.cpu.get_register_value(operand_register)
            .overflowing_add(shifter_operand + carry_amount);
    
    // Update flags if necessary
    if should_update_flags {
        emulator.cpu.set_nzcv(
            result >> 31 & 1 > 0,
            if result == 0 { true } else { false },
            // xxx: one of these two is incorrect
            overflow, // c: an unsigned overflow occured
            overflow // v: a signed overflow occured
        );
    }
    
    emulator.cpu.set_register_value(destination_register, result);
}

pub fn add(emulator: &mut Emulator, instruction: u32) {
    let should_update_flags = instruction >> 20 & 1 > 0;

    // Get the instruction operands
    let destination_register = Arm7RegisterNames::try_from(instruction >> 12 & 0xf).unwrap();
    let operand_register = Arm7RegisterNames::try_from(instruction >> 16 & 0xf).unwrap();
    let shifter_operand = process_shifter_operand(emulator, instruction);

    let (result, overflow) =
        emulator.cpu.get_register_value(operand_register)
            .overflowing_add(shifter_operand);
    
    // Update flags if necessary
    if should_update_flags {
        emulator.cpu.set_nzcv(
            result >> 31 & 1 > 0,
            if result == 0 { true } else { false },
            // xxx: one of these two is incorrect
            overflow, // c: an unsigned overflow occured
            overflow // v: a signed overflow occured
        );
    }
    
    emulator.cpu.set_register_value(destination_register, result);
}

pub fn and(emulator: &mut Emulator, instruction: u32) {}
pub fn b(emulator: &mut Emulator, instruction: u32) {}
pub fn bic(emulator: &mut Emulator, instruction: u32) {}
pub fn bl(emulator: &mut Emulator, instruction: u32) {}
pub fn bx(emulator: &mut Emulator, instruction: u32) {}
pub fn cdp(emulator: &mut Emulator, instruction: u32) {}
pub fn cmn(emulator: &mut Emulator, instruction: u32) {}
pub fn cmp(emulator: &mut Emulator, instruction: u32) {}
pub fn eor(emulator: &mut Emulator, instruction: u32) {}
pub fn ldc(emulator: &mut Emulator, instruction: u32) {}
pub fn ldm(emulator: &mut Emulator, instruction: u32) {}
pub fn ldr(emulator: &mut Emulator, instruction: u32) {}
pub fn ldrb(emulator: &mut Emulator, instruction: u32) {}
pub fn ldrbt(emulator: &mut Emulator, instruction: u32) {}
pub fn ldrh(emulator: &mut Emulator, instruction: u32) {}
pub fn ldrsb(emulator: &mut Emulator, instruction: u32) {}
pub fn ldrsh(emulator: &mut Emulator, instruction: u32) {}
pub fn ldrt(emulator: &mut Emulator, instruction: u32) {}
pub fn mcr(emulator: &mut Emulator, instruction: u32) {}
pub fn mla(emulator: &mut Emulator, instruction: u32) {}
pub fn mov(emulator: &mut Emulator, instruction: u32) {}
pub fn mrc(emulator: &mut Emulator, instruction: u32) {}
pub fn mrs(emulator: &mut Emulator, instruction: u32) {}
pub fn msr(emulator: &mut Emulator, instruction: u32) {}
pub fn mul(emulator: &mut Emulator, instruction: u32) {}
pub fn mvn(emulator: &mut Emulator, instruction: u32) {}
pub fn or(emulator: &mut Emulator, instruction: u32) {}
pub fn rsb(emulator: &mut Emulator, instruction: u32) {}
pub fn rsc(emulator: &mut Emulator, instruction: u32) {}
pub fn sbc(emulator: &mut Emulator, instruction: u32) {}
pub fn smlal(emulator: &mut Emulator, instruction: u32) {}
pub fn smull(emulator: &mut Emulator, instruction: u32) {}
pub fn stc(emulator: &mut Emulator, instruction: u32) {}
pub fn stm(emulator: &mut Emulator, instruction: u32) {}
pub fn str(emulator: &mut Emulator, instruction: u32) {}
pub fn strb(emulator: &mut Emulator, instruction: u32) {}
pub fn strbt(emulator: &mut Emulator, instruction: u32) {}
pub fn strh(emulator: &mut Emulator, instruction: u32) {}
pub fn strt(emulator: &mut Emulator, instruction: u32) {}
pub fn sub(emulator: &mut Emulator, instruction: u32) {}
pub fn swi(emulator: &mut Emulator, instruction: u32) {}
pub fn swp(emulator: &mut Emulator, instruction: u32) {}
pub fn swpb(emulator: &mut Emulator, instruction: u32) {}
pub fn teq(emulator: &mut Emulator, instruction: u32) {}
pub fn tst(emulator: &mut Emulator, instruction: u32) {}
pub fn umlal(emulator: &mut Emulator, instruction: u32) {}
pub fn umull(emulator: &mut Emulator, instruction: u32) {}












// Old notes, get rid of this when comfortable

// Move SPSR to register
// MRS{cond} Rd, SPSR
// Move CPSR to register
// MRS{cond} Rd, CPSR
// pub fn mrs() {}

// Move register to SPSR
// MSR{cond} SPSR{field}, Rm
// Move register to CPSR
// MSR{cond} CPSR{field}, Rm
// Move immediate to SPSR flags
// MSR{cond} SPSR_f, #32bit_Imm
// Move immediate to CPSR flags
// MSR{cond} CPSR_f, #32bit_Imm
// pub fn msr() {}

// Multiply
// pub fn mul() {}
// Multiply accumulate
// pub fn mla() {}

// Multiply unsigned long
// pub fn umull() {}

// Multiply unsigned accumulate long
// fn umlal() {}

// Multiply signed long
// fn smull() {}

// Multiply signed accumulate long
// fn smlal() {}

// Compare
// fn cmp() {}

// Compare negative
// fn cmn() {}

// Test
// fn tst() {}

// Test equivalent
// fn teq() {}

// AND

// Exclusive OR
// fn eor() {}

// ORR
// fn orr() {}

// Bit clear
// fn bic() {}

// Branch
// fn b() {}

// Branch link
// fn bl() {}

// Branch, link, and exchange instruction set
// fn blx() {}

// Branch and exchange instruction set
// fn bx() {}

// Word
// LDR{cond} Rd, <a_mode2>
// Word with user-mode privilege
// LDR{cond}T Rd, <a_mode2P>
// Byte
// LDR{cond}B Rd, <a_mode2>
// Byte with user-mode privilege
// LDR{cond}BT Rd, <a_mode2P>
// Byte signed
// LDR{cond}SB Rd, <a_mode3>
// Halfword
// LDR{cond}H Rd, <a_mode3>
// Halfword signed
// LDR{cond}SH Rd, <a_mode3>
// fn ldr() {}

// Increment before
// LDM{cond}IB Rd{!}, <reglist>{^}
// Increment after
// LDM{cond}IA Rd{!}, <reglist>{^}
// Decrement before
// LDM{cond}DB Rd{!}, <reglist>{^}
// Decrement after
// LDM{cond}DA Rd{!}, <reglist>{^}
// Stack operation
// LDM{cond}<a_mode4L> Rd{!}, <reglist>
// Stack operation, and restore CPSR
// LDM{cond}<a_mode4L> Rd{!}, <reglist+pc>^
// Stack operation with user registers
// LDM{cond}<a_mode4L> Rd{!}, <reglist>^
// fn ldm() {}

// Store, all the same shit as ldr, probably
// https://developer.arm.com/docs/ddi0210/latest/introduction/instruction-set-summary/arm-instruction-summary
// fn str() {}
// fn stm() {}

// Word
// SWP{cond} Rd, Rm, [Rn]
// Byte
// SWP{cond}B Rd, Rm, [Rn]
// fn swp() {}

// Coprocessors
// Data operation
// CDP{cond} p<cpnum>, <op1>, CRd, CRn, CRm, <op2>
// Move to ARM register from coprocessor
// MRC{cond} p<cpnum>, <op1>, Rd, CRn, CRm, <op2>
// Move to coprocessor from ARM register
// MCR{cond} p<cpnum>, <op1>, Rd, CRn, CRm, <op2>
// Load
// LDC{cond} p<cpnum>, CRd, <a_mode5>
// Store
// STC{cond} p<cpnum>, CRd, <a_mode5>
// fn cdp() {}
// fn mrc() {}
// fn mcr() {}
// fn ldc() {}
// fn stc() {}

// Software interrupt
// SWI 24bit_Imm
// fn swi() {}

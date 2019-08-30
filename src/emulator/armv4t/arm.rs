#[macro_use]
use crate::log;

use crate::emulator::{
    cpu::{Arm7OperationModes::*, Arm7RegisterNames::*, *},
    Emulator,
};
use std::convert::TryFrom;

pub fn process_instruction(emulator: &mut Emulator, instruction: u32) {
    // [27:20] and [7:4] are decode bits
    let category = instruction >> 25 & 7;

    log!("Category: {}", category);

    match category {
        // Data processing immediate shift if opcode != 0b10xx && s == 1
        // Miscellaneous instructions (Figure A3-4)
        // Data processing register shift if opcode != 0b10xx && s == 1
        // Miscellaneous instructions (Figure A3-4)
        // Multiplies (Figure A3-3) and Extra load/stores (Figure A3-5)
        0b000 => (),
        // Data processing immediate if opcode != 0b10xx && s == 1
        // Undefined instruction
        // Move immediate to status register
        0b001 => data_processing_immediate(emulator, instruction),
        // Load/store immediate offset
        0b010 => ls_immediate_offset(emulator, instruction),
        // Load/store register offset
        // Media instructions + architecturally undefined (Figure A3-2)
        // Architecturally undefined
        0b011 => (),
        // Load/store multiple
        0b100 => (),
        // Branch
        // 0b1010 - no link
        // 0b1011 - link
        0b101 => {
            let link = instruction >> 24 & 1;
            let shift = instruction & 0xffffff;
            log!("Hit a branch instruction with link {}", link);
            log!("Shifting by {}", shift);
            if link > 0 {
                log!("Setting r14 to return back");
                // r15 is safe to access directly because it isn't branched,
                // but r14 is branched so we much use set_register_value
                emulator
                    .cpu
                    .set_register_value(r14, emulator.cpu.registers.r15);
            }
            emulator.cpu.registers.r15 += shift;
        }
        // Coprocessor load/store and double register transfers
        0b110 => (),
        // Coprocessor data processing
        // Coprocessor register transfers
        // Software interupt
        0b111 => (),
        // Theoretically, this is impossible, but we don't have a way to tell the
        // compiler that, so we have to have the case here anyway.
        _ => (),
    }
}

pub fn data_processing_immediate_shift(emulator: &mut Emulator, instruction: u32) {

}

pub fn data_processing_register_shift(emulator: &mut Emulator, instruction: u32) {

}

pub fn data_processing_immediate(emulator: &mut Emulator, instruction: u32) {
    // let immediate = instruction >> 25 & 1;
    let opcode = instruction >> 21 & 7;
    let set_condition_flags = instruction >> 20 & 1;
    let r_operand = Arm7RegisterNames::try_from(instruction >> 16 & 15).unwrap();
    let r_destination = Arm7RegisterNames::try_from(instruction >> 12 & 15).unwrap();
    let rs = Arm7RegisterNames::try_from(instruction >> 8 & 15).unwrap();

    let a = emulator.cpu.get_register_value(r_operand);
    let b = emulator.cpu.get_register_value(rs);

    let Emulator { cpu, memory } = emulator;

    if set_condition_flags > 0 {
        log!("Setting the condition flags");
    }

    match opcode {
        0b0000 => cpu.set_register_value(r_destination, a & b),
        0b0001 => {
            emulator.cpu.set_register_value(r_destination, a ^ b);
        }
        0b0010 => {
            emulator.cpu.set_register_value(r_destination, a - b);
        }
        0b0011 => {
            emulator.cpu.set_register_value(r_destination, b - a);
        }
        0b0100 => {
            emulator.cpu.set_register_value(r_destination, a + b);
        }
        0b0101 => {
            log!("Add with carry!");
        }
        0b0110 => {
            log("Subtract with carry!");
        }
        0b1101 => {
            emulator.cpu.set_register_value(r_destination, b);
        }

        _ => log!("unknown instruction {:x}", instruction),
    };
}

pub fn ls_immediate_offset(emulator: &mut Emulator, instruction: u32) {
  let Emulator{ cpu, memory } = emulator;
}

// Move
// pub fn mov() {}

// Move NOT
// pub fn mvn() {}

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

// Add
// pub fn add() {}

// Add carry
// pub fn adc() {}

// Subtract
// pub fn sub() {}

// Subtract with carry
// pub fn sbc() {}

// Reverse subtract
// pub fn rsb() {}

// Reverse subtract with carry
// pub fn rsc() {}

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
// fn and() {}

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

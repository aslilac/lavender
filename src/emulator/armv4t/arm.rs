#[macro_use]
use crate::log;

use std::convert::TryFrom;
use crate::emulator::cpu::*;

const DP_I: u32 = 1 << 25;
const DP_OPCODE: u32 = 7 << 21;
const DP_S: u32 = 1 << 20;
const DP_Rn: u32 = 15 << 16;
const DP_Rd: u32 = 15 << 12;
const DP_Rs: u32 = 15 << 8;

pub fn process_instruction(instruction: u32) {
}

pub fn process_data_processing_instruction(cpu: &mut Arm7Tdmi, instruction: u32) {
  // let i = (instruction & Self::DP_I) >> 25;
  let opcode = (instruction & DP_OPCODE) >> 21;
  let s = (instruction & DP_S) >> 20;
  let rn = Arm7RegisterNames::try_from((instruction & DP_Rn) >> 16).unwrap();
  let rd = Arm7RegisterNames::try_from((instruction & DP_Rd) >> 12).unwrap();
  let rs = Arm7RegisterNames::try_from((instruction & DP_Rs) >> 8).unwrap();

  let a = cpu.get_register_value(rn);
  let b = cpu.get_register_value(rs);

  match opcode {
    0b0000 => {
      cpu.set_register_value(rd, a & b);
    }
    0b0001 => {
      cpu.set_register_value(rd, a ^ b);
    }
    0b0010 => {
      cpu.set_register_value(rd, a - b);
    }
    0b0011 => {
      cpu.set_register_value(rd, b - a);
    }
    0b0100 => {
      cpu.set_register_value(rd, a + b);
    }
    0b0101 => {
      log!("Add with carry!");
    }
    0b0110 => {
      log("Subtract with carry!");
    }
    0b1101 => {
      cpu.set_register_value(rd, b);
    }

    _ => log!("opcode: {:b}; s: {}; rn: {:?}; rd: {:?};", opcode, s, rn, rd)
  };
}

// Move
pub fn mov(instruction: u32) {}

// Move NOT
pub fn mvn() {}

// Move SPSR to register
// MRS{cond} Rd, SPSR
// Move CPSR to register
// MRS{cond} Rd, CPSR
pub fn mrs() {}

// Move register to SPSR
// MSR{cond} SPSR{field}, Rm
// Move register to CPSR
// MSR{cond} CPSR{field}, Rm
// Move immediate to SPSR flags
// MSR{cond} SPSR_f, #32bit_Imm
// Move immediate to CPSR flags
// MSR{cond} CPSR_f, #32bit_Imm
pub fn msr() {}

// Add
pub fn add() {}

// Add carry
pub fn adc() {}

// Subtract
pub fn sub() {}

// Subtract with carry
pub fn sbc() {}

// Reverse subtract
pub fn rsb() {}

// Reverse subtract with carry
pub fn rsc() {}

// Multiply
pub fn mul() {}

// Multiply accumulate
pub fn mla() {}

// Multiply unsigned long
pub fn umull() {}

// Multiply unsigned accumulate long
fn umlal() {}

// Multiply signed long
fn smull() {}

// Multiply signed accumulate long
fn smlal() {}

// Compare
fn cmp() {}

// Compare negative
fn cmn() {}

// Test
fn tst() {}

// Test equivalent
fn teq() {}

// AND
fn and() {}

// Exclusive OR
fn eor() {}

// ORR
fn orr() {}

// Bit clear
fn bic() {}

// Branch
fn b() {}

// Branch link
fn bl() {}

// Branch, link, and exchange instruction set
fn blx() {}

// Branch and exchange instruction set
fn bx() {}

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
fn ldr() {}

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
fn ldm() {}

// Store, all the same shit as ldr, probably
// https://developer.arm.com/docs/ddi0210/latest/introduction/instruction-set-summary/arm-instruction-summary
fn str() {}
fn stm() {}

// Word
// SWP{cond} Rd, Rm, [Rn]
// Byte
// SWP{cond}B Rd, Rm, [Rn]
fn swp() {}

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
fn cdp() {}
fn mrc() {}
fn mcr() {}
fn ldc() {}
fn stc() {}

// Software interrupt
// SWI 24bit_Imm
fn swi() {}

use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::convert::TryFrom;

use super::{ConditionCode, OperationMode};

/// An enum of the register names available to the processor. These names can be
/// used in the `get_register_value` and `set_register_value` functions to ensure
/// that the register being accessed is the correct one for the current execution
/// mode. (r14 maps to r14_irq when in irq execution, etc.)
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u32)]
pub enum Reg {
	r0,
	r1,
	r2,
	r3,
	r4,
	r5,
	r6,
	r7,
	r8,
	r9,
	r10,
	r11,
	r12,
	r13,
	r14,
	r15,
	cpsr,
	spsr,
}

impl Reg {
	fn is_gp(&self) -> bool {
		use Reg::*;
		self != &cpsr && self != &spsr
	}
}

/// The 31 registers contained within the ARM7TDMI processor.
#[derive(Default)]
pub struct RegisterSet {
	// General purpose registers
	pub r0: u32,
	pub r1: u32,
	pub r2: u32,
	pub r3: u32,
	pub r4: u32,
	pub r5: u32,
	pub r6: u32,
	pub r7: u32,
	pub r8: u32,
	pub r8_fiq: u32,
	pub r9: u32,
	pub r9_fiq: u32,
	pub r10: u32,
	pub r10_fiq: u32,
	pub r11: u32,
	pub r11_fiq: u32,
	pub r12: u32,
	pub r12_fiq: u32,

	// Stack pointer, but only by convention, not actually enforced.
	// Known as sp in thumb
	pub r13: u32,
	pub r13_fiq: u32,
	pub r13_svc: u32,
	pub r13_abt: u32,
	pub r13_irq: u32,
	pub r13_und: u32,

	// Link register
	// Basically, the address to jump back to once a subroutine
	// or interupt has completed execution.
	// Known as lr in thumb
	pub r14: u32,
	pub r14_fiq: u32,
	pub r14_svc: u32,
	pub r14_abt: u32,
	pub r14_irq: u32,
	pub r14_und: u32,

	// Program counter
	// Known as pc in thumb
	// In ARM, the 2 least significant bits should always be zero
	// In Thumb, the least significant bit should always be zero
	// If in a branch instruction bit 0 is set to 1, then the thumb bit should flip.
	pub r15: u32,

	// Status registers
	// Current program state register
	// https://developer.arm.com/docs/ddi0210/latest/programmers-model/the-program-status-registers
	// # Condition codes
	// Set by arithmatic instructions, as well as MSR and LDM
	// N - Negative or less than [31]
	// Z - Zero [30]
	// C - Carry, borrow, extend [29]
	// V - Overflow [28]
	// # Reserved [27:8]
	// IRQ Disable [7]
	// FIQ Disable [6]
	// Thumb bit [5]
	// Mode [4:0]
	// An illegal mode value should trigger a reset
	pub cpsr: u32,
	// Stored program state register
	// Stores information on the user program while in a priveledged mode.
	// Should probably be restored to cpsr when leaving priviledge.
	// Stored from cpsr before mode switching.
	pub spsr_fiq: u32,
	pub spsr_svc: u32,
	pub spsr_abt: u32,
	pub spsr_irq: u32,
	pub spsr_und: u32,
}

impl RegisterSet {
	const IRQ_DISABLE: u32 = 1 << 7;
	const FIQ_DISABLE: u32 = 1 << 6;
	const THUMB_BIT: u32 = 1 << 5;

	pub fn new() -> Self {
		Default::default()
	}

	pub fn get_value(&self, name: Reg) -> u32 {
		use OperationMode::*;
		use Reg::*;

		let mode = self.get_operation_mode().unwrap();

		match (name, mode) {
			// General purpose registers
			(r0, _) => self.r0,
			(r1, _) => self.r1,
			(r2, _) => self.r2,
			(r3, _) => self.r3,
			(r4, _) => self.r4,
			(r5, _) => self.r5,
			(r6, _) => self.r6,
			(r7, _) => self.r7,
			(r8, FIQ) => self.r8_fiq,
			(r8, _) => self.r8,
			(r9, FIQ) => self.r9_fiq,
			(r9, _) => self.r9,
			(r10, FIQ) => self.r10_fiq,
			(r10, _) => self.r10,
			(r11, FIQ) => self.r11_fiq,
			(r11, _) => self.r11,
			(r12, FIQ) => self.r12_fiq,
			(r12, _) => self.r12,
			(r13, FIQ) => self.r13_fiq,
			(r13, SVC) => self.r13_svc,
			(r13, ABT) => self.r13_abt,
			(r13, IRQ) => self.r13_irq,
			(r13, UND) => self.r13_und,
			(r13, _) => self.r13,
			(r14, FIQ) => self.r14_fiq,
			(r14, SVC) => self.r14_svc,
			(r14, ABT) => self.r14_abt,
			(r14, IRQ) => self.r14_irq,
			(r14, UND) => self.r14_und,
			(r14, _) => self.r14,
			(r15, _) => self.r15,

			// Control registers
			(cpsr, _) => self.cpsr,
			(spsr, FIQ) => self.spsr_fiq,
			(spsr, SVC) => self.spsr_svc,
			(spsr, ABT) => self.spsr_abt,
			(spsr, IRQ) => self.spsr_irq,
			(spsr, UND) => self.spsr_und,
			// This register is the only one that is unaccessable in certain
			// execution modes.
			(spsr, _) => panic!("Attempting to use spsr register while not in a priveledged mode"),
		}
	}

	pub fn set_value(&mut self, name: Reg, value: u32) {
		use OperationMode::*;
		use Reg::*;

		let mode = self.get_operation_mode().unwrap();

		match (name, mode) {
			(r0, _) => self.r0 = value,
			(r1, _) => self.r1 = value,
			(r2, _) => self.r2 = value,
			(r3, _) => self.r3 = value,
			(r4, _) => self.r4 = value,
			(r5, _) => self.r5 = value,
			(r6, _) => self.r6 = value,
			(r7, _) => self.r7 = value,
			(r8, FIQ) => self.r8_fiq = value,
			(r8, _) => self.r8 = value,
			(r9, FIQ) => self.r9_fiq = value,
			(r9, _) => self.r9 = value,
			(r10, FIQ) => self.r10_fiq = value,
			(r10, _) => self.r10 = value,
			(r11, FIQ) => self.r11_fiq = value,
			(r11, _) => self.r11 = value,
			(r12, FIQ) => self.r12_fiq = value,
			(r12, _) => self.r12 = value,
			(r13, FIQ) => self.r13_fiq = value,
			(r13, SVC) => self.r13_svc = value,
			(r13, ABT) => self.r13_abt = value,
			(r13, IRQ) => self.r13_irq = value,
			(r13, UND) => self.r13_und = value,
			(r13, _) => self.r13 = value,
			(r14, FIQ) => self.r14_fiq = value,
			(r14, SVC) => self.r14_svc = value,
			(r14, ABT) => self.r14_abt = value,
			(r14, IRQ) => self.r14_irq = value,
			(r14, UND) => self.r14_und = value,
			(r14, _) => self.r14 = value,
			(r15, _) => self.r15 = value,

			// We might want to protect these from writes, but we also
			// might not need to.
			(cpsr, _) => self.cpsr = value,
			(spsr, FIQ) => self.spsr_fiq = value,
			(spsr, SVC) => self.spsr_svc = value,
			(spsr, ABT) => self.spsr_abt = value,
			(spsr, IRQ) => self.spsr_irq = value,
			(spsr, UND) => self.spsr_und = value,
			// This register cannot be accessed from other modes.
			(spsr, _) => panic!("Attempting to use spsr register while not in a priveledged mode"),
		};
	}

	pub fn map_value<F>(&mut self, name: Reg, map: F)
	where
		F: FnOnce(u32) -> u32,
	{
		let prev = self.get_value(name);
		self.set_value(name, map(prev));
	}

	pub fn get_n(&self) -> bool {
		self.cpsr >> 31 & 1 > 0
	}

	pub fn get_z(&self) -> bool {
		self.cpsr >> 30 & 1 > 0
	}

	pub fn get_c(&self) -> bool {
		self.cpsr >> 29 & 1 > 0
	}

	pub fn get_v(&self) -> bool {
		self.cpsr >> 28 & 1 > 0
	}

	pub fn set_nzcv(&mut self, n: bool, z: bool, c: bool, v: bool) {
		let mut flags = 0;

		if n {
			flags |= 0x8
		}
		if z {
			flags |= 0x4
		}
		if c {
			flags |= 0x2
		}
		if v {
			flags |= 0x1
		}

		self.cpsr = (self.cpsr & 0x0fffffff) | (flags << 28);
	}

	pub fn set_fiq_disable(&mut self, disabled: bool) {
		if disabled {
			self.cpsr |= RegisterSet::FIQ_DISABLE;
		} else {
			self.cpsr &= !RegisterSet::FIQ_DISABLE;
		}
	}

	pub fn is_fiq_disabled(&self) -> bool {
		(self.cpsr & RegisterSet::FIQ_DISABLE) > 0
	}

	pub fn set_irq_disable(&mut self, disabled: bool) {
		if disabled {
			self.cpsr |= RegisterSet::IRQ_DISABLE;
		} else {
			self.cpsr &= !RegisterSet::IRQ_DISABLE;
		}
	}

	pub fn is_irq_disabled(&self) -> bool {
		(self.cpsr & RegisterSet::IRQ_DISABLE) > 0
	}

	pub fn set_thumb_bit(&mut self, thumb: bool) {
		if thumb {
			self.cpsr |= RegisterSet::THUMB_BIT;
		} else {
			self.cpsr &= !RegisterSet::THUMB_BIT;
		}
	}

	pub fn get_thumb_bit(&self) -> bool {
		(self.cpsr & RegisterSet::THUMB_BIT) > 0
	}

	pub fn set_operation_mode(&mut self, mode: OperationMode) {
		// When switching to priviledge, should store cpsr in spsr, as
		// well as the current PC (r15) in LR (r14), and then change modes
		// When switching back, should load cpsr from spsr
		let mode_flags: u32 = mode.into();
		self.cpsr = (self.cpsr & 0xffffffe0) | mode_flags;
	}

	pub fn get_operation_mode(&self) -> Option<OperationMode> {
		match OperationMode::try_from(self.cpsr & 0b11111) {
			Ok(mode) => Some(mode),
			Err(_) => None,
		}
	}

	pub fn check_condition(&self, cond: ConditionCode) -> bool {
		use ConditionCode::*;

		match cond {
			EQ => self.get_z(),
			NE => !self.get_z(),
			CS => self.get_c(),
			CC => !self.get_c(),
			MI => self.get_n(),
			PL => !self.get_n(),
			VS => self.get_v(),
			VC => !self.get_v(),

			HI => self.get_c() && !self.get_z(),
			LS => !self.get_c() || self.get_z(),
			GE => self.get_n() == self.get_v(),
			LT => self.get_n() != self.get_v(),
			GT => !self.get_z() && (self.get_n() == self.get_v()),
			LE => self.get_z() && (self.get_n() != self.get_v()),

			AL => true,
			NO => true, // "Unpredictable behavior"
		}
	}
}

#[cfg(test)]
mod tests {
	use super::{ConditionCode::*, OperationMode::*, Reg::*, *};

	#[test]
	fn set_operation_mode() {
		let mut rs = RegisterSet::new();

		// SYS by default
		assert_eq!(rs.get_operation_mode(), Some(SYS));

		// Change into UND mode
		rs.set_operation_mode(UND);
		assert_eq!(rs.get_operation_mode(), Some(UND));

		// Change into USR mode
		rs.set_operation_mode(USR);
		assert_eq!(rs.get_operation_mode(), Some(USR));
	}

	#[test]
	fn thumb_bit() {
		let mut rs = RegisterSet::new();

		// Off by default
		assert_eq!(rs.get_thumb_bit(), false);
		rs.set_thumb_bit(true);
		assert_eq!(rs.get_thumb_bit(), true);
		rs.set_thumb_bit(false);
		assert_eq!(rs.get_thumb_bit(), false);
	}

	#[test]
	fn register_mapping() {
		let mut rs = RegisterSet::new();

		rs.set_operation_mode(SVC);
		rs.set_value(r13, 0xdeadbeef);
		assert_eq!(rs.get_value(r13), 0xdeadbeef);

		rs.set_operation_mode(USR);
		assert_eq!(rs.get_value(r13), 0);
	}

	#[test]
	fn interupts() {
		let mut rs = RegisterSet::new();

		// Disabled by default
		assert!(rs.is_fiq_disabled());
		assert!(rs.is_irq_disabled());

		// Enable them
		rs.set_fiq_disable(false);
		rs.set_irq_disable(false);
		assert!(!rs.is_fiq_disabled());
		assert!(!rs.is_irq_disabled());
	}

	#[test]
	fn condition_bits() {
		let mut rs = RegisterSet::new();

		// Should all be off by default
		assert!(!rs.get_n());
		assert!(!rs.get_z());
		assert!(!rs.get_c());
		assert!(!rs.get_v());

		// Make sure they stay off
		rs.set_nzcv(false, false, false, false);

		// Should still all be off
		assert!(!rs.get_n());
		assert!(!rs.get_z());
		assert!(!rs.get_c());
		assert!(!rs.get_v());

		// Set all of the flag bits
		rs.set_nzcv(true, true, true, true);

		// Should now all be on
		assert!(rs.get_n());
		assert!(rs.get_z());
		assert!(rs.get_c());
		assert!(rs.get_v());

		// Set all of the flag bits
		rs.set_nzcv(true, true, true, true);

		// Should still all be on
		assert!(rs.get_n());
		assert!(rs.get_z());
		assert!(rs.get_c());
		assert!(rs.get_v());
	}

	#[test]
	fn conditions() {
		let mut rs = RegisterSet::new();

		// All bits should be zero by default, so these conditions should pass.
		assert!(rs.check_condition(PL));
		assert!(rs.check_condition(NE));
		assert!(rs.check_condition(CC));
		assert!(rs.check_condition(VC));
		assert!(rs.check_condition(GE));
		assert!(rs.check_condition(AL));

		// Turn on the negative bit. MI should pass, PL should not.
		rs.set_nzcv(true, false, false, false);
		assert!(rs.check_condition(MI));
		assert!(!rs.check_condition(PL));

		// N bit is set and V is not.
		assert!(rs.check_condition(LT));

		// Turn on the zero bit. EQ should pass, NE should not.
		rs.set_nzcv(false, true, false, false);
		assert!(rs.check_condition(EQ));
		assert!(!rs.check_condition(NE));

		// Z bit is set and C is not.
		assert!(rs.check_condition(LS));

		// Turn on the carry bit. CS should pass, CC should not.
		rs.set_nzcv(false, false, true, false);
		assert!(rs.check_condition(CS));
		assert!(!rs.check_condition(CC));

		// C bit is set and Z is not.
		assert!(rs.check_condition(HI));

		// Turn on the overflow bit. VS should pass, VC should not.
		rs.set_nzcv(false, false, false, true);
		assert!(rs.check_condition(VS));
		assert!(!rs.check_condition(VC));
	}
}

use num_enum::IntoPrimitive;
use num_enum::TryFromPrimitive;

use crate::registers::RegisterSet;

/// All of the operation modes that are available to the processor.
#[derive(Copy, Clone, Debug, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u32)]
pub enum OperationMode {
	USR = 0b10000, // Normal execution
	FIQ = 0b10001, // Fast interupt
	IRQ = 0b10010, // Interupt
	SVC = 0b10011, // Service/supervisor
	SYS = 0b11111, // System operation, can only be entered from another priviledged mode
	ABT = 0b10111, // Abort
	UND = 0b11011, // Undefined, entered from invalid opcodes
}

impl From<&RegisterSet> for OperationMode {
	fn from(rs: &RegisterSet) -> Self {
		OperationMode::try_from(rs.cpsr & 0b11111)
			.ok()
			.unwrap_or(OperationMode::SYS)
	}
}

use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::convert::TryFrom;

pub use super::arm::{OperationMode, Reg, RegisterSet};

/// The primary processor of the Game Boy Advance. This is the CPU used to run
/// Game Boy Advance Games.
pub struct Arm7Tdmi {
	pub registers: RegisterSet,
	pub halt: bool,
}

/// The secondary processor of the Game Boy Advance. It is the same one used in
/// the Game Boy and Game Boy Color, and is included for backwards compatibility.
/// We do not yet do anything with it, but might in the future.
pub struct LR35902;

impl Arm7Tdmi {
	pub fn init() -> Self {
		let mut cpu = Self {
			halt: true,
			registers: RegisterSet::new(),
		};

		cpu.reset();
		cpu
	}

	pub fn reset(&mut self) {
		use OperationMode::SYS;
		use Reg::r15;
		// Entering service mode should probably do this automatically
		// once we have a bit more infrastructure set up.
		// These are technically undefined behavior.
		self.registers.r14_svc = self.registers.r14;
		self.registers.spsr_svc = self.registers.cpsr;

		// Program counter set to 0, and in sys mode
		self.registers.set_operation_mode(SYS);

		// Disable interupts and thumb instruction mode
		self.registers.set_fiq_disable(true);
		self.registers.set_irq_disable(true);
		self.registers.set_thumb_bit(false);

		// Load the program counter from the trap vector
		self.registers.set_value(
			r15, 0, // memory.read_word(0)
		);
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn empty() {}
}

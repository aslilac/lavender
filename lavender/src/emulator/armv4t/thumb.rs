use crate::emulator::Emulator;
use instructions::*;

/// Decodes and runs the instruction using the given emulator, and returns the
/// number of cycles used.
pub fn process_instruction(emulator: &mut Emulator, instruction: u16) -> u32 {
	decode_instruction(instruction)(emulator, instruction)
}

pub fn decode_instruction(instruction: u16) -> fn(&mut Emulator, u16) -> u32 {
	let category = instruction >> 13 & 7;

	match category {
		0b000 => {
			// Shift by rotate
			placeholder
		}
		0b001 => {
			// Add/subtract/compare/move immediate
			placeholder
		}
		0b010 => {
			let subcategory = instruction >> 10 & 0x7;
			match subcategory {
				0b000 => {
					// Data processing register
					let opcode = instruction >> 6 & 0xf;

					match opcode {
						_ => placeholder,
					}
				}
				0b001 => placeholder,         // Special data processing and branch/exchange
				0b010 | 0b011 => placeholder, // Load from literal pool
				0b100..=0b111 => placeholder, // Load/store register offset
				_ => placeholder,
			}
		}
		0b011 => placeholder, // Load/store word/byte immediate offset
		0b100 => {
			let stack = instruction >> 12 & 1 > 0;
			match stack {
				true => placeholder,  // Load/store to/from stack
				false => placeholder, // Load/store halfword immediate offset
			}
		}
		0b101 => {
			let misc = instruction >> 12 & 1 > 0;
			match misc {
				true => placeholder,  // miscellaneous instructions
				false => placeholder, // Add to SP or PC immediate
			}
		}
		0b110 => {
			let branch = instruction >> 12 & 1 > 0;
			let condition = instruction >> 8 & 0xf;
			match (branch, condition) {
				(true, 0b1110) => placeholder, // undefined
				(true, 0b1111) => swi,         // swi
				(true, _) => placeholder,      // conditional branch things
				(false, _) => placeholder,     // load/store multiple
			}
		}
		0b111 => placeholder, // unconditional branches
		_ => unreachable!(),
	}
}

pub fn placeholder(_emulator: &mut Emulator, _instruction: u16) -> u32 {
	1
}

pub mod instructions {
	use super::super::arm::instructions::*;
	use crate::emulator::Emulator;

	pub fn adc(_emulator: &mut Emulator, _instruction: u16) -> u32 {
		1
	}
	pub fn add(_emulator: &mut Emulator, _instruction: u16) -> u32 {
		1
	}
	pub fn and(_emulator: &mut Emulator, _instruction: u16) -> u32 {
		1
	}
	pub fn asr(_emulator: &mut Emulator, _instruction: u16) -> u32 {
		1
	}
	pub fn b(_emulator: &mut Emulator, _instruction: u16) -> u32 {
		1
	}
	pub fn bic(_emulator: &mut Emulator, _instruction: u16) -> u32 {
		1
	}
	pub fn bl(_emulator: &mut Emulator, _instruction: u16) -> u32 {
		1
	}
	pub fn bx(_emulator: &mut Emulator, _instruction: u16) -> u32 {
		1
	}
	pub fn cmn(_emulator: &mut Emulator, _instruction: u16) -> u32 {
		1
	}
	pub fn cmp(_emulator: &mut Emulator, _instruction: u16) -> u32 {
		1
	}
	/// Logical XOR
	pub fn eor(_emulator: &mut Emulator, _instruction: u16) -> u32 {
		1
	}
	pub fn ldmia(_emulator: &mut Emulator, _instruction: u16) -> u32 {
		1
	}
	pub fn ldr(_emulator: &mut Emulator, _instruction: u16) -> u32 {
		1
	}
	pub fn ldrb(_emulator: &mut Emulator, _instruction: u16) -> u32 {
		1
	}
	pub fn ldrh(_emulator: &mut Emulator, _instruction: u16) -> u32 {
		1
	}
	pub fn ldrsb(_emulator: &mut Emulator, _instruction: u16) -> u32 {
		1
	}
	pub fn ldrsh(_emulator: &mut Emulator, _instruction: u16) -> u32 {
		1
	}
	pub fn lsl(_emulator: &mut Emulator, _instruction: u16) -> u32 {
		1
	}
	pub fn lsr(_emulator: &mut Emulator, _instruction: u16) -> u32 {
		1
	}
	pub fn mov(_emulator: &mut Emulator, _instruction: u16) -> u32 {
		1
	}
	pub fn mul(_emulator: &mut Emulator, _instruction: u16) -> u32 {
		1
	}
	pub fn mvn(_emulator: &mut Emulator, _instruction: u16) -> u32 {
		1
	}
	pub fn neg(_emulator: &mut Emulator, _instruction: u16) -> u32 {
		1
	}
	/// Logical OR (also referred to as the orr instruction)
	pub fn or(_emulator: &mut Emulator, _instruction: u16) -> u32 {
		1
	}
	pub fn pop(_emulator: &mut Emulator, _instruction: u16) -> u32 {
		1
	}
	pub fn push(_emulator: &mut Emulator, _instruction: u16) -> u32 {
		1
	}
	pub fn ror(_emulator: &mut Emulator, _instruction: u16) -> u32 {
		1
	}
	pub fn sbc(_emulator: &mut Emulator, _instruction: u16) -> u32 {
		1
	}
	pub fn stmia(_emulator: &mut Emulator, _instruction: u16) -> u32 {
		1
	}
	pub fn str(_emulator: &mut Emulator, _instruction: u16) -> u32 {
		1
	}
	pub fn strb(_emulator: &mut Emulator, _instruction: u16) -> u32 {
		1
	}
	pub fn strh(_emulator: &mut Emulator, _instruction: u16) -> u32 {
		1
	}
	pub fn sub(_emulator: &mut Emulator, _instruction: u16) -> u32 {
		1
	}
	pub fn swi(_emulator: &mut Emulator, _instruction: u16) -> u32 {
		1
	}
	pub fn tst(_emulator: &mut Emulator, _instruction: u16) -> u32 {
		1
	}
}

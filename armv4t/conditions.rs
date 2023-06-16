use crate::registers::RegisterSet;
use num_enum::IntoPrimitive;
use num_enum::TryFromPrimitive;

/// All of the possible condition codes for 32-bit ARM instructions. All ARMv4T
/// instructions begin with a 4 bit condition code that can control whether or
/// not the instruction is executed.
#[derive(Copy, Clone, Debug, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u32)]
pub enum ConditionCode {
	/// Equals - Executes if Z is set
	EQ = 0b0000,
	/// Not equal - Executes if Z is clear
	NE = 0b0001,
	/// Carry set/unsigned higher or same - Executes if C is set
	CS = 0b0010,
	/// Carry clear/unsigned lower - Executes if C is clear
	CC = 0b0011,
	/// Minus/negative - Executes if N is set
	MI = 0b0100,
	/// Plus/positive and zero - Executes if N is clear
	PL = 0b0101,
	/// Overflow set - Executes if V is set
	VS = 0b0110,
	/// Overflow clear - Executes if V is clear
	VC = 0b0111,

	/// Unsigned higher - C set and Z clear
	HI = 0b1000,
	/// Unsigned lower or same - C clear or Z set
	LS = 0b1001,
	/// Signed greater than or equal - N set and V set, or N clear and V clear (N == V)
	GE = 0b1010,
	/// Signed less than - N set and V clear, or N clear and V set (N != V)
	LT = 0b1011,
	/// Signed greater than - Z clear, and either N set and V set, or N clear and V clear (Z == 0,N == V)
	GT = 0b1100,
	/// Signed less than or equal - Z set, or N set and V clear, or N clear and V set (Z == 1 or N != V)
	LE = 0b1101,

	/// Always, no conditions
	AL = 0b1110,
	/// Undefined, unpredictable
	NO = 0b1111,
}

impl ConditionCode {
	fn check_condition(&self, rs: &RegisterSet) -> bool {
		use ConditionCode::*;

		match self {
			EQ => rs.get_z(),
			NE => !rs.get_z(),
			CS => rs.get_c(),
			CC => !rs.get_c(),
			MI => rs.get_n(),
			PL => !rs.get_n(),
			VS => rs.get_v(),
			VC => !rs.get_v(),

			HI => rs.get_c() && !rs.get_z(),
			LS => !rs.get_c() || rs.get_z(),
			GE => rs.get_n() == rs.get_v(),
			LT => rs.get_n() != rs.get_v(),
			GT => !rs.get_z() && (rs.get_n() == rs.get_v()),
			LE => rs.get_z() && (rs.get_n() != rs.get_v()),

			AL => true,
			NO => true, // "Unpredictable behavior"
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use ConditionCode::*;

	#[test]
	fn conditions() {
		let mut rs = RegisterSet::default();

		// All bits should be zero by default, so these conditions should pass.
		assert!(PL.check_condition(&rs));
		assert!(NE.check_condition(&rs));
		assert!(CC.check_condition(&rs));
		assert!(VC.check_condition(&rs));
		assert!(GE.check_condition(&rs));
		assert!(AL.check_condition(&rs));

		// Turn on the negative bit. MI should pass, PL should not.
		rs.set_nzcv(true, false, false, false);
		assert!(MI.check_condition(&rs));
		assert!(!PL.check_condition(&rs));

		// N bit is set and V is not.
		assert!(LT.check_condition(&rs));

		// Turn on the zero bit. EQ should pass, NE should not.
		rs.set_nzcv(false, true, false, false);
		assert!(EQ.check_condition(&rs));
		assert!(!NE.check_condition(&rs));

		// Z bit is set and C is not.
		assert!(LS.check_condition(&rs));

		// Turn on the carry bit. CS should pass, CC should not.
		rs.set_nzcv(false, false, true, false);
		assert!(CS.check_condition(&rs));
		assert!(!CC.check_condition(&rs));

		// C bit is set and Z is not.
		assert!(HI.check_condition(&rs));

		// Turn on the overflow bit. VS should pass, VC should not.
		rs.set_nzcv(false, false, false, true);
		assert!(VS.check_condition(&rs));
		assert!(!VC.check_condition(&rs));
	}
}

use std::ops::Range;

/// Get some bits from a word, following the convention of the ARMv4T manual. Eg.
/// `3..0` represents the 4 least significant bits, like the `[3:0]` notation ARM uses.
fn get_bits(inst: u32, range: Range<u32>) -> u32 {
	debug_assert!(range.start < 32);
	debug_assert!(range.end < range.start);
	(inst >> range.end) & (0xffffffff >> (31 - range.end + range.start))
}

const fn get_bit(inst: u32, bit: u32) -> bool {
	debug_assert!(bit < 32);
	(inst >> bit) & 0x1 > 0
}

pub fn decode_instruction(inst: u32) {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn get_bits_test() {
		assert_eq!(get_bits(0b0110_1001_1010_0101, 3..0), 0b0101);
		assert_eq!(get_bits(0b0110_1001_1010_0101, 7..4), 0b1010);
	}
}

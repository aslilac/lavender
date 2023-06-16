use crate::registers::Reg;

pub struct Adc {
	i: bool,
	s: bool,
	rn: Reg,
	rd: Reg,
	shifter_12: u32,
}

fn adc(inst: Adc) {
	let Adc {
		i,
		s,
		rn,
		rd,
		shifter_12,
	} = inst;
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_adc() {}
}

pub mod arm7tdmi;
pub mod conditions;
pub mod ende;
pub mod modes;
pub mod registers;

pub mod instructions {
	pub enum Instruction {
		Adc(adc::Adc),
	}

	pub mod adc;
}

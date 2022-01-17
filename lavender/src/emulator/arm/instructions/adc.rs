use super::super::{Instruction, RegisterSet};

fn adc(instruction: Instruction) {
    if let Instruction::adc{ i, s, rn, rd, shifter_12 } = instruction {
        ()
    } else {
        unreachable!();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_adc() {
        
    }
}

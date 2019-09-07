use super::arm::*;

#[test]
fn decode_add() {
  assert_eq!(
    decode_instruction(0b1110_00_1_0100_1_0011_0011_0100_00000000),
    add
  );
}

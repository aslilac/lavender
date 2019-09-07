use super::arm::*;

#[test]
fn decode_adc() {
  assert_eq!(
    decode_instruction(0x0_0a_000_0_0) as usize,
    adc as usize
  );
}

#[test]
fn decode_add() {
  assert_eq!(
    decode_instruction(0x0_08_000_0_0) as usize,
    add as usize
  );
}

#[test]
fn decode_and() {
  assert_eq!(
    decode_instruction(0x0_00_000_0_0) as usize,
    and as usize
  );
}

#[test]
fn decode_bic() {
  assert_eq!(
    decode_instruction(0x0_1c_000_0_0) as usize,
    bic as usize
  );
}

#[test]
fn decode_cdp() {
  assert_eq!(
    decode_instruction(0x0_e0_000_0_0) as usize,
    cdp as usize
  );
}

#[test]
fn decode_cmn() {
  assert_eq!(
    decode_instruction(0x0_17_000_0_0) as usize,
    cmn as usize
  );
}

#[test]
fn decode_cmp() {
  assert_eq!(
    decode_instruction(0x0_15_000_0_0) as usize,
    cmp as usize
  );
}

#[test]
fn decode_eor() {
  assert_eq!(
    decode_instruction(0x0_02_000_0_0) as usize,
    eor as usize
  );
}

#[test]
fn decode_ldc() {
  assert_eq!(
    decode_instruction(0x0_c1_000_0_0) as usize,
    ldc as usize
  );
}

#[test]
fn decode_ldm() {
  // We only test against the one mode, because the others all share these two bits
  assert_eq!(
    decode_instruction(0x0_81_000_0_0) as usize,
    ldm as usize
  );
}

#[test]
fn decode_ldr() {
  assert_eq!(
    decode_instruction(0x0_41_000_0_0) as usize,
    ldr as usize
  );
}

#[test]
fn decode_ldrb() {
  assert_eq!(
    decode_instruction(0x0_45_000_0_0) as usize,
    ldrb as usize
  );
}

// #[test]
// fn decode_ldrbt() {
//   assert_eq!(
//     decode_instruction(0x0_47_000_0_0) as usize,
//     ldrbt as usize
//   );
// }

#[test]
fn decode_ldrh() {
  assert_eq!(
    decode_instruction(0x0_05_000_b_0) as usize,
    ldrh as usize
  );
}

#[test]
fn decode_ldrsb() {
  assert_eq!(
    decode_instruction(0x0_05_000_d_0) as usize,
    ldrsb as usize
  );
}

#[test]
fn decode_ldrsh() {
  assert_eq!(
    decode_instruction(0x0_05_000_f_0) as usize,
    ldrsh as usize
  );
}

// #[test]
// fn decode_ldrt() {
//   assert_eq!(
//     decode_instruction(0x0_43_000_0_0) as usize,
//     ldrt as usize
//   );
// }

#[test]
fn decode_mcr() {
  assert_eq!(
    decode_instruction(0x0_e0_000_1_0) as usize,
    mcr as usize
  );
}

#[test]
fn decode_mla() {
  assert_eq!(
    decode_instruction(0x0_02_000_9_0) as usize,
    mla as usize
  );
}

#[test]
fn decode_mov() {
  assert_eq!(
    decode_instruction(0x0_1a_000_0_0) as usize,
    mov as usize
  );
}

#[test]
fn decode_mrs() {
  assert_eq!(
    decode_instruction(0x0_10_000_0_0) as usize,
    mrs as usize
  );
}

#[test]
fn decode_msr() {
  assert_eq!(
    decode_instruction(0x0_12_000_0_0) as usize,
    msr as usize
  );
}

#[test]
fn decode_mul() {
  assert_eq!(
    decode_instruction(0x0_0_000_9_0) as usize,
    mul as usize
  );
}

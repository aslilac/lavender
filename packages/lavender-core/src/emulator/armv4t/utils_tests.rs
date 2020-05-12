use crate::emulator::{armv4t::utils::*, cpu::RegisterNames::*, Emulator};

#[test]
fn test_addressing_mode_2_immediate_offset() {
    let mut emulator = Emulator::dummy();

    emulator.cpu.set_register_value(r1, 0x4000_0000);

    // Add offset to base register value
    {
        //   cond    P UBWL Rn   Rd   offset12
        // 0b1110_0101_1001_0001_0011_1000_0000_0001 - ldr r3,[r1,0x801]
        let (address, addressing_type) = process_addressing_mode(&mut emulator, 0xE591_3801);
        assert_eq!(address, 0x4000_0801);
        assert_eq!(addressing_type, AddressingType::Offset);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x4000_0000);
    }

    // Substract offset from base register value
    {
        //   cond    P UBWL Rn   Rd   offset12
        // 0b1110_0101_0001_0001_0011_1000_0000_0001 - ldr r3,[r1,-0x801]
        let (address, addressing_type) = process_addressing_mode(&mut emulator, 0xE511_3801);
        assert_eq!(address, 0x3FFF_F7FF);
        assert_eq!(addressing_type, AddressingType::Offset);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x4000_0000);
    }
}

#[test]
fn test_addressing_mode_2_immediate_postindexed() {
    let mut emulator = Emulator::dummy();

    {
        emulator.cpu.set_register_value(r1, 0x4000_0000);

        //   cond    P UBWL Rn   Rd   offset12
        // 0b1110_0100_1001_0001_0011_1000_0000_0001 - ldr r3,[r1],0x801
        let (address, addressing_type) = process_addressing_mode(&mut emulator, 0xE491_3801);
        assert_eq!(address, 0x4000_0000);
        assert_eq!(addressing_type, AddressingType::PostIndexed);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x4000_0801);
    }

    // Substract offset
    {
        emulator.cpu.set_register_value(r1, 0x4000_0000);

        //   cond    P UBWL Rn   Rd   offset12
        // 0b1110_0100_0001_0001_0011_1000_0000_0001 - ldr r3,[r1],-0x801
        let (address, addressing_type) = process_addressing_mode(&mut emulator, 0xE411_3801);
        assert_eq!(address, 0x4000_0000);
        assert_eq!(addressing_type, AddressingType::PostIndexed);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x3FFF_F7FF);
    }
}

#[test]
fn test_addressing_mode_2_immediate_preindexed() {
    let mut emulator = Emulator::dummy();

    // Add offset
    {
        emulator.cpu.set_register_value(r1, 0x4000_0000);
        //   cond    P UBWL Rn   Rd   offset12
        // 0b1110_0101_1011_0001_0011_1000_0000_0001 - ldr r3,[r1,0x801]!
        let (address, addressing_type) = process_addressing_mode(&mut emulator, 0xE5B1_3801);
        assert_eq!(address, 0x4000_0801);
        assert_eq!(addressing_type, AddressingType::PreIndexed);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x4000_0801);
    }

    // Substract offset
    {
        emulator.cpu.set_register_value(r1, 0x4000_0000);
        //   cond    P UBWL Rn   Rd   offset12
        // 0b1110_0101_0011_0001_0011_1000_0000_0001 - ldr r3,[r1,-0x801]!
        let (address, addressing_type) = process_addressing_mode(&mut emulator, 0xE531_3801);
        assert_eq!(address, 0x3FFF_F7FF);
        assert_eq!(addressing_type, AddressingType::PreIndexed);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x3FFF_F7FF);
    }
}

#[test]
fn test_addressing_mode_2_register_offset() {
    let mut emulator = Emulator::dummy();

    emulator.cpu.set_register_value(r1, 0x4000_0000);
    emulator.cpu.set_register_value(r2, 0x1000_0001);

    // Add offset
    {
        //   cond    P UBWL Rn   Rd   SBZ       Rm
        // 0b1110_0111_1001_0001_0011_0000_0000_0001 - ldr r3,[r1,r2]
        let (address, addressing_type) = process_addressing_mode(&mut emulator, 0xE791_3002);
        assert_eq!(address, 0x5000_0001);
        assert_eq!(addressing_type, AddressingType::Offset);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x4000_0000);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x1000_0001);
    }

    // Substract offset
    {
        //   cond    P UBWL Rn   Rd   SBZ       Rm
        // 0b1110_0111_0001_0001_0011_0000_0000_0001 - ldr r3,[r1,-r2]
        let (address, addressing_type) = process_addressing_mode(&mut emulator, 0xE711_3002);
        assert_eq!(address, 0x2FFF_FFFF);
        assert_eq!(addressing_type, AddressingType::Offset);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x4000_0000);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x1000_0001);
    }
}

#[test]
fn test_addressing_mode_2_register_preindexed() {
    let mut emulator = Emulator::dummy();

    emulator.cpu.set_register_value(r2, 0x1000_0001);

    {
        emulator.cpu.set_register_value(r1, 0x4000_0000);
        //   cond    P UBWL Rn   Rd   SBZ       Rm
        // 0b1110_0111_1011_0001_0011_0000_0000_0010 - ldr r3,[r1,r2]!
        let (address, addressing_type) = process_addressing_mode(&mut emulator, 0xE7B1_3002);
        assert_eq!(address, 0x5000_0001);
        assert_eq!(addressing_type, AddressingType::PreIndexed);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x5000_0001);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x1000_0001);
    }

    // Substract offset
    {
        emulator.cpu.set_register_value(r1, 0x4000_0000);
        //   cond    P UBWL Rn   Rd   SBZ       Rm
        // 0b1110_0111_0011_0001_0011_0000_0000_0010 - ldr r3,[r1,-r2]!
        let (address, addressing_type) = process_addressing_mode(&mut emulator, 0xE731_3002);
        assert_eq!(address, 0x2FFF_FFFF);
        assert_eq!(addressing_type, AddressingType::PreIndexed);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x2FFF_FFFF);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x1000_0001);
    }
}

#[test]
fn test_addressing_mode_2_scaled_register_offset_asr() {
    let mut emulator = Emulator::dummy();

    emulator.cpu.set_register_value(r1, 0x4000_0000);
    emulator.cpu.set_register_value(r2, 0x8000_0000);

    {
        //   cond    P UBWL Rn   Rd   Imm   Sh   Rm
        // 0b1110_0111_1001_0001_0011_11110_10_0_0010 - ldr r3,[r1,r2,asr 0x1E]
        let (address, addressing_type) = process_addressing_mode(&mut emulator, 0xE791_3F42);
        assert_eq!(address, 0x3FFF_FFFE);
        assert_eq!(addressing_type, AddressingType::Offset);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x4000_0000);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x8000_0000);
    }

    // Special case when shift_imm == 0 and Rm contains a negative number
    {
        //   cond    P UBWL Rn   Rd   Imm   Sh   Rm
        // 0b1110_0111_1001_0001_0011_00000_10_0_0010 - ldr r3,[r1,r2,asr 0x20]
        let (address, addressing_type) = process_addressing_mode(&mut emulator, 0xE791_3042);
        assert_eq!(address, 0x3FFF_FFFF);
        assert_eq!(addressing_type, AddressingType::Offset);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x4000_0000);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x8000_0000);
    }

    // Special case when shift_imm == 0 and Rm contains a positive number
    {
        emulator.cpu.set_register_value(r2, 0x7FFF_FFFF);

        //   cond    P UBWL Rn   Rd   Imm   Sh   Rm
        // 0b1110_0111_1001_0001_0011_00000_10_0_0010 - ldr r3,[r1,r2,asr 0x20]
        let (address, addressing_type) = process_addressing_mode(&mut emulator, 0xE791_3042);
        assert_eq!(address, 0x4000_0000);
        assert_eq!(addressing_type, AddressingType::Offset);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x4000_0000);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x7FFF_FFFF);
    }
}

#[test]
fn test_addressing_mode_2_scaled_register_offset_lsl() {
    let mut emulator = Emulator::dummy();

    emulator.cpu.set_register_value(r1, 0x4000_0000);
    emulator.cpu.set_register_value(r2, 0x0000_0001);

    // Add offset
    {
        //   cond    P UBWL Rn   Rd   Imm   Sh   Rm
        // 0b1110_0111_1001_0001_0011_11100_00_0_0010 - ldr r3,[r1,r2,lsl 0x1C]
        let (address, addressing_type) = process_addressing_mode(&mut emulator, 0xE791_3E02);
        assert_eq!(address, 0x5000_0000);
        assert_eq!(addressing_type, AddressingType::Offset);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x4000_0000);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x0000_0001);
    }

    // Substract offset
    {
        //   cond    P UBWL Rn   Rd   Imm   Sh   Rm
        // 0b1110_0111_0001_0001_0011_11100_00_0_0010 - ldr r3,[r1,-r2,lsl 0x1C]
        let (address, addressing_type) = process_addressing_mode(&mut emulator, 0xE711_3E02);
        assert_eq!(address, 0x3000_0000);
        assert_eq!(addressing_type, AddressingType::Offset);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x4000_0000);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x0000_0001);
    }
}

#[test]
fn test_addressing_mode_2_scaled_register_offset_lsr() {
    let mut emulator = Emulator::dummy();

    emulator.cpu.set_register_value(r1, 0x4000_0000);
    emulator.cpu.set_register_value(r2, 0x8000_0000);

    // Add offset
    {
        //   cond    P UBWL Rn   Rd   Imm   Sh   Rm
        // 0b1110_0111_1001_0001_0011_00011_01_0_0010 - ldr r3,[r1,r2,lsr 0x3]
        let (address, addressing_type) = process_addressing_mode(&mut emulator, 0xE791_31A2);
        assert_eq!(address, 0x5000_0000);
        assert_eq!(addressing_type, AddressingType::Offset);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x4000_0000);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x8000_0000);
    }

    // Special case when shift_imm == 0 (i.e. "32 bit shift")
    {
        //   cond    P UBWL Rn   Rd   Imm   Sh   Rm
        // 0b1110_0111_1001_0001_0011_00000_01_0_0010 - ldr r3,[r1,r2,lsr 0x20]
        let (address, addressing_type) = process_addressing_mode(&mut emulator, 0xE791_3022);
        assert_eq!(address, 0x4000_0000);
        assert_eq!(addressing_type, AddressingType::Offset);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x4000_0000);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x8000_0000);
    }
}

#[test]
fn test_addressing_mode_2_scaled_register_offset_ror_rrx() {
    let mut emulator = Emulator::dummy();

    emulator.cpu.set_register_value(r1, 0x4000_0000);
    emulator.cpu.set_register_value(r2, 0x0001_1000);

    {
        //   cond    P UBWL Rn   Rd   Imm   Sh   Rm
        // 0b1110_0111_1001_0001_0011_10000_11_0_0010 - ldr r3,[r1,r2,ror 0x10]
        let (address, addressing_type) = process_addressing_mode(&mut emulator, 0xE791_3862);
        assert_eq!(address, 0x5000_0001);
        assert_eq!(addressing_type, AddressingType::Offset);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x4000_0000);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x0001_1000);
    }

    // Special case when shift_imm == 0 and C flag is set
    {
        emulator.cpu.set_nzcv(false, false, true, false);
        emulator.cpu.set_register_value(r2, 0x2);

        //   cond    P UBWL Rn   Rd   Imm   Sh   Rm
        // 0b1110_0111_1001_0001_0011_00000_11_0_0010 - ldr r3,[r1,r2,rrx 0x1]
        let (address, addressing_type) = process_addressing_mode(&mut emulator, 0xE791_3062);
        assert_eq!(address, 0xC000_0001);
        assert_eq!(addressing_type, AddressingType::Offset);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x4000_0000);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x2);
    }

    // Special case when shift_imm == 0 and C flag is not set
    {
        emulator.cpu.set_nzcv(false, false, false, false);
        emulator.cpu.set_register_value(r2, 0x2);

        //   cond    P UBWL Rn   Rd   Imm   Sh   Rm
        // 0b1110_0111_1001_0001_0011_00000_11_0_0010 - ldr r3,[r1,r2,rrx 0x1]
        let (address, addressing_type) = process_addressing_mode(&mut emulator, 0xE791_3062);
        assert_eq!(address, 0x4000_0001);
        assert_eq!(addressing_type, AddressingType::Offset);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x4000_0000);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x2);
    }
}

#[test]
fn test_addressing_mode_2_scaled_register_preindexed_lsl() {
    let mut emulator = Emulator::dummy();

    emulator.cpu.set_register_value(r1, 0x4000_0000);
    emulator.cpu.set_register_value(r2, 0x0000_0001);

    {
        //   cond    P UBWL Rn   Rd   Imm   Sh   Rm
        // 0b1110_0111_1011_0001_0011_11100_00_0_0010 - ldr r3,[r1,r2,lsl 0x1C]!
        let (address, addressing_type) = process_addressing_mode(&mut emulator, 0xE7B1_3E02);
        assert_eq!(address, 0x5000_0000);
        assert_eq!(addressing_type, AddressingType::PreIndexed);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x5000_0000);
        assert_eq!(emulator.cpu.get_register_value(r2), 0x0000_0001);
    }
}

#[test]
fn test_addressing_mode_3_immediate_offset() {
    let mut emulator = Emulator::dummy();

    {
        emulator.cpu.set_register_value(r1, 0x4000_0000);

        //   cond    P U WL Rn   Rd   immH  SH  immL
        // 0b1110_0001_1101_0001_0010_1000_1011_0001 - ldrh r2,[r1,0x81]
        let (address, addressing_type) = process_misc_addressing_mode(&mut emulator, 0xE1D1_28B1);
        assert_eq!(address, 0x4000_0081);
        assert_eq!(addressing_type, AddressingType::Offset);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x4000_0000);
    }

    // Substract offset
    {
        //   cond    P U WL Rn   Rd   immH  SH  immL
        // 0b1110_0001_0101_0001_0010_1000_1011_0001 - ldrh r2,[r1,-0x81]
        let (address, addressing_type) = process_misc_addressing_mode(&mut emulator, 0xE151_28B1);
        assert_eq!(address, 0x3FFF_FF7F);
        assert_eq!(addressing_type, AddressingType::Offset);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x4000_0000);
    }
}

#[test]
fn test_addressing_mode_3_register_offset() {
    let mut emulator = Emulator::dummy();

    {
        emulator.cpu.set_register_value(r1, 0x4000_0000);
        emulator.cpu.set_register_value(r3, 0x1000_0000);

        //   cond    P U WL Rn   Rd   SBZ   SH  Rm
        // 0b1110_0001_1001_0001_0010_0000_1011_0011 - ldrh r2,[r1,r3]
        let (address, addressing_type) = process_misc_addressing_mode(&mut emulator, 0xE191_20B3);
        assert_eq!(address, 0x5000_0000);
        assert_eq!(addressing_type, AddressingType::Offset);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x4000_0000);
    }
}

#[test]
fn test_addressing_mode_3_immediate_preindexed() {
    let mut emulator = Emulator::dummy();

    {
        emulator.cpu.set_register_value(r1, 0x4000_0000);

        //   cond    P U WL Rn   Rd   immH  SH  Rm
        // 0b1110_0001_1111_0001_0010_1000_1011_0001 - ldrh r2,[r1,0x81]!
        let (address, addressing_type) = process_misc_addressing_mode(&mut emulator, 0xE1F1_28B1);
        assert_eq!(address, 0x4000_0081);
        assert_eq!(addressing_type, AddressingType::PreIndexed);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x4000_0081);
    }
}

#[test]
fn test_addressing_mode_3_register_preindexed() {
    let mut emulator = Emulator::dummy();

    {
        emulator.cpu.set_register_value(r1, 0x4000_0000);
        emulator.cpu.set_register_value(r3, 0x1000_0000);

        //   cond    P U WL Rn   Rd   SBZ   SH  Rm
        // 0b1110_0001_1011_0001_0010_0000_1011_0011 - ldrh r2,[r1,r3]!
        let (address, addressing_type) = process_misc_addressing_mode(&mut emulator, 0xE1B1_20B3);
        assert_eq!(address, 0x5000_0000);
        assert_eq!(addressing_type, AddressingType::PreIndexed);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x5000_0000);
    }
}

#[test]
fn test_addressing_mode_3_immediate_postindexed() {
    let mut emulator = Emulator::dummy();

    {
        emulator.cpu.set_register_value(r1, 0x4000_0000);

        //   cond    P U WL Rn   Rd   immH  SH  Rm
        // 0b1110_0000_1101_0001_0010_1000_1011_0001 - ldrh r2,[r1],0x81
        let (address, addressing_type) = process_misc_addressing_mode(&mut emulator, 0xE0D1_28B1);
        assert_eq!(address, 0x4000_0000);
        assert_eq!(addressing_type, AddressingType::PostIndexed);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x4000_0081);
    }
}

#[test]
fn test_addressing_mode_3_register_postindexed() {
    let mut emulator = Emulator::dummy();

    {
        emulator.cpu.set_register_value(r1, 0x4000_0000);
        emulator.cpu.set_register_value(r3, 0x1000_0000);

        //   cond    P U WL Rn   Rd   SBZ   SH  Rm
        // 0b1110_0000_1001_0001_0010_0000_1011_0011 - ldrh r2,[r1],r3
        let (address, addressing_type) = process_misc_addressing_mode(&mut emulator, 0xE091_20B3);
        assert_eq!(address, 0x4000_0000);
        assert_eq!(addressing_type, AddressingType::PostIndexed);
        assert_eq!(emulator.cpu.get_register_value(r1), 0x5000_0000);
    }
}

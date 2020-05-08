use crate::emulator::{
    armv4t::arm::{decode_instruction, instructions::*, process_instruction},
    cpu::RegisterNames::*,
    Emulator,
};

#[test]
fn decode_adc() {
    assert_eq!(decode_instruction(0x0_0a_000_0_0) as usize, adc as usize);
}

#[test]
fn decode_add() {
    assert_eq!(decode_instruction(0x0_08_000_0_0) as usize, add as usize);
}

#[test]
fn behavior_add() {
    let mut emulator = Emulator::dummy();

    // Initialize r3 adn r4 as the accumulators, and r5 as the increment for r4
    emulator.cpu.set_register_value(r3, 0);
    emulator.cpu.set_register_value(r4, 0);
    emulator.cpu.set_register_value(r5, 3);

    for _ in 0..10 {
        // add r3, r3, #1
        process_instruction(&mut emulator, 0b1110_00_1_0100_1_0011_0011_0000_00000001);
        // add r4, r4, r5
        process_instruction(&mut emulator, 0b1110_00_0_0100_1_0100_0100_00000000_0101);
    }

    // Assert that the adding completed correctly
    assert_eq!(emulator.cpu.get_register_value(r3), 10);
    assert_eq!(emulator.cpu.get_register_value(r4), 30);
}

#[test]
fn decode_and() {
    assert_eq!(decode_instruction(0x0_00_000_0_0) as usize, and as usize);
}

#[test]
fn decode_b() {
    assert_eq!(decode_instruction(0x0_a0_000_0_0) as usize, b as usize);
}

#[test]
fn behavior_b() {
    let mut emulator = Emulator::dummy();

    // Set the pc to a known value
    let starting_position = 0x0100_0000;
    emulator.cpu.set_register_value(r15, starting_position);

    // Branch with distance of 0
    process_instruction(&mut emulator, 0b1110_101_0_0000_0000_0000_0000_0000_0000);
    assert_eq!(emulator.cpu.get_register_value(r15), starting_position);

    // Branch with largest positive number (0x7fffff<<2)
    process_instruction(&mut emulator, 0b1110_101_0_0111_1111_1111_1111_1111_1111);
    assert_eq!(
        emulator.cpu.get_register_value(r15),
        starting_position + (0x7fffff << 2)
    );

    // Branch with smallest negative number (-4)
    process_instruction(&mut emulator, 0b1110_101_0_1111_1111_1111_1111_1111_1111);
    assert_eq!(
        emulator.cpu.get_register_value(r15),
        starting_position + (0x7fffff << 2) - 4
    );

    // Branch with largest negative number (0x800000<<2)
    process_instruction(&mut emulator, 0b1110_101_0_1000_0000_0000_0000_0000_0000);
    assert_eq!(emulator.cpu.get_register_value(r15), starting_position - 8);
}

#[test]
fn decode_bic() {
    assert_eq!(decode_instruction(0x0_1c_000_0_0) as usize, bic as usize);
}

#[test]
fn decode_bl() {
    assert_eq!(decode_instruction(0x0_b0_000_0_0) as usize, bl as usize);
}

#[test]
fn decode_bx() {
    assert_eq!(decode_instruction(0x0_12_000_1_0) as usize, bx as usize);
}

#[test]
fn decode_cdp() {
    assert_eq!(decode_instruction(0x0_e0_000_0_0) as usize, cdp as usize);
}

#[test]
fn decode_cmn() {
    assert_eq!(decode_instruction(0x0_17_000_0_0) as usize, cmn as usize);
}

#[test]
fn decode_cmp() {
    assert_eq!(decode_instruction(0x0_15_000_0_0) as usize, cmp as usize);
}

#[test]
fn decode_eor() {
    assert_eq!(decode_instruction(0x0_02_000_0_0) as usize, eor as usize);
}

#[test]
fn behavior_eor() {
    let mut emulator = Emulator::dummy();

    emulator.cpu.set_register_value(r4, 0xaaaaaaaa);
    emulator.cpu.set_register_value(r5, 0xbebebebe);

    // eor r4, r4, r5
    process_instruction(&mut emulator, 0b1110_00_0_0001_1_0100_0100_00000000_0101);

    assert_eq!(emulator.cpu.get_register_value(r4), 0x14141414);
}

#[test]
fn decode_ldc() {
    assert_eq!(decode_instruction(0x0_c1_000_0_0) as usize, ldc as usize);
}

#[test]
fn decode_ldm() {
    // Even though this instruction has multiple modes, they should all overlap
    assert_eq!(decode_instruction(0x0_81_000_0_0) as usize, ldm as usize);
}

#[test]
fn decode_ldr() {
    assert_eq!(decode_instruction(0x0_41_000_0_0) as usize, ldr as usize);
}

#[test]
fn decode_ldrb() {
    assert_eq!(decode_instruction(0x0_45_000_0_0) as usize, ldrb as usize);
}

#[test]
fn decode_ldrbt() {
    assert_eq!(decode_instruction(0x0_47_000_0_0) as usize, ldrbt as usize);
}

#[test]
fn decode_ldrh() {
    assert_eq!(decode_instruction(0x0_05_000_b_0) as usize, ldrh as usize);
}

#[test]
fn decode_ldrsb() {
    assert_eq!(decode_instruction(0x0_05_000_d_0) as usize, ldrsb as usize);
}

#[test]
fn decode_ldrsh() {
    assert_eq!(decode_instruction(0x0_05_000_f_0) as usize, ldrsh as usize);
}

#[test]
fn decode_ldrt() {
    assert_eq!(decode_instruction(0x0_43_000_0_0) as usize, ldrt as usize);
}

#[test]
fn decode_mcr() {
    assert_eq!(decode_instruction(0x0_e0_000_1_0) as usize, mcr as usize);
}

#[test]
fn decode_mla() {
    assert_eq!(decode_instruction(0x0_02_000_9_0) as usize, mla as usize);
}

#[test]
fn decode_mov() {
    assert_eq!(decode_instruction(0x0_1a_000_0_0) as usize, mov as usize);
}

#[test]
fn decode_mrc() {
    assert_eq!(decode_instruction(0x0_e1_000_1_0) as usize, mrc as usize);
}

#[test]
fn decode_mrs() {
    assert_eq!(decode_instruction(0x0_10_000_0_0) as usize, mrs as usize);
}

#[test]
fn decode_msr() {
    assert_eq!(decode_instruction(0x0_12_000_0_0) as usize, msr as usize);
}

#[test]
fn decode_mul() {
    assert_eq!(decode_instruction(0x0_00_000_9_0) as usize, mul as usize);
}

#[test]
fn decode_mvn() {
    assert_eq!(decode_instruction(0x0_1e_000_0_0) as usize, mvn as usize);
}

#[test]
fn decode_or() {
    assert_eq!(decode_instruction(0x0_18_000_0_0) as usize, or as usize);
}

#[test]
fn decode_rsb() {
    assert_eq!(decode_instruction(0x0_06_000_0_0) as usize, rsb as usize);
}

#[test]
fn decode_rsc() {
    assert_eq!(decode_instruction(0x0_0e_000_0_0) as usize, rsc as usize);
}

#[test]
fn decode_sbc() {
    assert_eq!(decode_instruction(0x0_0c_000_0_0) as usize, sbc as usize);
}

#[test]
fn decode_smlal() {
    assert_eq!(decode_instruction(0x0_0e_000_9_0) as usize, smlal as usize);
}

#[test]
fn decode_smull() {
    assert_eq!(decode_instruction(0x0_0c_000_9_0) as usize, smull as usize);
}

#[test]
fn decode_stc() {
    assert_eq!(decode_instruction(0x0_c0_000_0_0) as usize, stc as usize);
}

#[test]
fn decode_stm() {
    // Even though this instruction has multiple modes, they should all overlap
    assert_eq!(decode_instruction(0x0_80_000_0_0) as usize, stm as usize);
}

#[test]
fn decode_str() {
    assert_eq!(decode_instruction(0x0_40_000_0_0) as usize, str as usize);
}

#[test]
fn decode_strb() {
    assert_eq!(decode_instruction(0x0_44_000_0_0) as usize, strb as usize);
}

#[test]
fn decode_strbt() {
    assert_eq!(decode_instruction(0x0_46_000_0_0) as usize, strbt as usize);
}

#[test]
fn decode_strh() {
    assert_eq!(decode_instruction(0x0_00_000_b_0) as usize, strh as usize);
}

#[test]
fn decode_strt() {
    assert_eq!(decode_instruction(0x0_42_000_0_0) as usize, strt as usize);
}

#[test]
fn behavior_sub() {
    let mut emulator = Emulator::dummy();

    // sub{s} r0, r1, r2
    let instruction = 0xE051_0002;

    // r1 = 0xFFFF_FFFE
    // r2 = 0x7FFF_FFFF
    // r0 = 0x7FFF_FFFF
    // nzcv = 0b0011
    {
        emulator.cpu.set_register_value(r1, 0xFFFF_FFFE);
        emulator.cpu.set_register_value(r2, 0x7FFF_FFFF);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x7FFF_FFFF);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), true);
        assert_eq!(emulator.cpu.get_v(), true);
    }

    // r1 = 0x1
    // r2 = 0x1
    // r0 = 0x0
    // nzcv = 0b0110
    {
        emulator.cpu.set_register_value(r1, 0x1);
        emulator.cpu.set_register_value(r2, 0x1);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x0);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), true);
        assert_eq!(emulator.cpu.get_c(), true);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    // r1 = 0x0
    // r2 = 0x1
    // r0 = 0xFFFF_FFFF
    // nzcv = 0b1000
    {
        emulator.cpu.set_register_value(r1, 0x0);
        emulator.cpu.set_register_value(r2, 0x1);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0xFFFF_FFFF);
        assert_eq!(emulator.cpu.get_n(), true);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    // r1 = 0xFFFF_FFFF
    // r2 = 0xFFFF_FFFE
    // r0 = 0x1
    // nzcv = 0b0010
    {
        emulator.cpu.set_register_value(r1, 0xFFFF_FFFF);
        emulator.cpu.set_register_value(r2, 0xFFFF_FFFE);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x1);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), true);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    // r1 = 0x0
    // r2 = 0xFFFF_FFFF
    // r0 = 0x1
    // nzcv = 0b0000
    {
        emulator.cpu.set_register_value(r1, 0x0);
        emulator.cpu.set_register_value(r2, 0xFFFF_FFFF);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x1);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    // r1 = 0x2
    // r2 = 0x1
    // r0 = 0x1
    // nzcv = 0b0010
    {
        emulator.cpu.set_register_value(r1, 0x2);
        emulator.cpu.set_register_value(r2, 0x1);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x1);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), true);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    // r1 = 0xFFFF_FFFF
    // r2 = 0xFFFF_FFFF
    // r0 = 0x0
    // nzcv = 0b0110
    {
        emulator.cpu.set_register_value(r1, 0xFFFF_FFFF);
        emulator.cpu.set_register_value(r2, 0xFFFF_FFFF);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x0);
        assert_eq!(emulator.cpu.get_n(), false);
        assert_eq!(emulator.cpu.get_z(), true);
        assert_eq!(emulator.cpu.get_c(), true);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    // r1 = 0xFFFF_FFFE
    // r2 = 0xFFFF_FFFF
    // r0 = 0xFFFF_FFFF
    // nzcv = 0b0110
    {
        emulator.cpu.set_register_value(r1, 0xFFFF_FFFE);
        emulator.cpu.set_register_value(r2, 0xFFFF_FFFF);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0xFFFF_FFFF);
        assert_eq!(emulator.cpu.get_n(), true);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), false);
    }

    // r1 = 0x7FFF_FFFF
    // r2 = 0xFFFF_FFFF
    // r0 = 0x8000_0000
    // nzcv = 0b1001
    {
        emulator.cpu.set_register_value(r1, 0x7FFF_FFFF);
        emulator.cpu.set_register_value(r2, 0xFFFF_FFFF);

        process_instruction(&mut emulator, instruction);

        assert_eq!(emulator.cpu.get_register_value(r0), 0x8000_0000);
        assert_eq!(emulator.cpu.get_n(), true);
        assert_eq!(emulator.cpu.get_z(), false);
        assert_eq!(emulator.cpu.get_c(), false);
        assert_eq!(emulator.cpu.get_v(), true);
    }
}

#[test]
fn decode_sub() {
    assert_eq!(decode_instruction(0x0_04_000_0_0) as usize, sub as usize);
}

#[test]
fn decode_swi() {
    assert_eq!(decode_instruction(0x0_f0_000_0_0) as usize, swi as usize);
}

#[test]
fn decode_swp() {
    assert_eq!(decode_instruction(0x0_10_000_9_0) as usize, swp as usize);
}

#[test]
fn decode_swpb() {
    assert_eq!(decode_instruction(0x0_14_000_9_0) as usize, swpb as usize);
}

#[test]
fn decode_teq() {
    assert_eq!(decode_instruction(0x0_13_000_0_0) as usize, teq as usize);
}

#[test]
fn decode_tst() {
    assert_eq!(decode_instruction(0x0_11_000_0_0) as usize, tst as usize);
}

#[test]
fn decode_umlal() {
    assert_eq!(decode_instruction(0x0_0a_000_9_0) as usize, umlal as usize);
}

#[test]
fn decode_umull() {
    assert_eq!(decode_instruction(0x0_08_000_9_0) as usize, umull as usize);
}

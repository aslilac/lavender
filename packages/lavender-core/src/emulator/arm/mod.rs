mod instructions;
pub mod register_set;

use num_enum::{IntoPrimitive, TryFromPrimitive};
pub use register_set::*;

/// An enum that represents all of the instructions we currently know how to decode!
#[allow(non_camel_case_types)]
#[repr(u32)]
enum Instruction {
    adc {
        i: bool,
        s: bool,
        rn: Reg,
        rd: Reg,
        shifter_12: u32,
    },
    nop {},
}

fn decode_instruction(instruction: u32) -> Instruction {
    use Instruction::*;
    let decode = instruction >> 20 & 0xff;

    adc {
        i: true,
        s: true,
        rn: Reg::r0,
        rd: Reg::r0,
        shifter_12: 0,
    }
}

/// All of the operation modes that are available to the processor. Using this
/// enum ensures that we are always in a valid operation mode.
#[derive(Copy, Clone, Debug, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u32)]
pub enum OperationMode {
    USR = 0b10000, // Normal execution
    FIQ = 0b10001, // Fast interupt
    IRQ = 0b10010, // Interupt
    SVC = 0b10011, // Service/supervisor
    SYS = 0b11111, // System operation, can only be entered from another priviledged mode
    ABT = 0b10111, // Abort
    UND = 0b11011, // Undefined, entered from invalid opcodes
}

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

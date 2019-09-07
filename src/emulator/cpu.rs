#[macro_use]
use crate::log;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::{
    convert::TryFrom,
    default::Default,
    num::Wrapping,
};

pub struct Arm7Tdmi {
    pub frequency: u32,
    pub halt: bool,
    pub registers: Arm7TdmiRegisters,
}

impl Arm7Tdmi {
    pub fn init() -> Self {
        let mut cpu = Self {
            frequency: 0,
            halt: true,
            registers: Arm7TdmiRegisters::new(),
        };

        cpu.reset();
        cpu
    }

    pub fn set_frequency(&mut self, frequency: u32) {
        self.frequency = frequency;
    }

    pub fn start(&mut self) {
        self.halt = false;
        if self.frequency <= 0 {
            log!("Starting processor with frequency of zero will do nothing.");
        }
    }

    pub fn halt(&mut self) {
        self.halt = true;
    }

    pub fn reset(&mut self) {
        use Arm7OperationModes::SYS;
        use Arm7RegisterNames::*;
        // Entering service mode should probably do this automatically
        // once we have a bit more infrastructure set up.
        // These are technically undefined behavior.
        self.registers.r14_svc = self.registers.r14;
        self.registers.spsr_svc = self.registers.cpsr;

        // Program counter set to 0, and in sys mode
        self.set_operation_mode(SYS);

        // Disable interupts and thumb instruction mode
        self.set_fiq_disable(true);
        self.set_irq_disable(true);
        self.set_thumb_bit(false);

        // Load the program counter from the trap vector
        self.set_register_value(
            r15, 0, // memory.read_word(0)
        );
    }

    pub fn get_register_value(&self, name: Arm7RegisterNames) -> u32 {
        use Arm7OperationModes::*;
        use Arm7RegisterNames::*;

        let mode = self.get_operation_mode().unwrap();

        match (name, mode) {
            // General purpose registers
            (r0, _) => self.registers.r0,
            (r1, _) => self.registers.r1,
            (r2, _) => self.registers.r2,
            (r3, _) => self.registers.r3,
            (r4, _) => self.registers.r4,
            (r5, _) => self.registers.r5,
            (r6, _) => self.registers.r6,
            (r7, _) => self.registers.r7,
            (r8, FIQ) => self.registers.r8_fiq,
            (r8, _) => self.registers.r8,
            (r9, FIQ) => self.registers.r9_fiq,
            (r9, _) => self.registers.r9,
            (r10, FIQ) => self.registers.r10_fiq,
            (r10, _) => self.registers.r10,
            (r11, FIQ) => self.registers.r11_fiq,
            (r11, _) => self.registers.r11,
            (r12, FIQ) => self.registers.r12_fiq,
            (r12, _) => self.registers.r12,
            (r13, FIQ) => self.registers.r13_fiq,
            (r13, SVC) => self.registers.r13_svc,
            (r13, ABT) => self.registers.r13_abt,
            (r13, IRQ) => self.registers.r13_irq,
            (r13, UND) => self.registers.r13_und,
            (r13, _) => self.registers.r13,
            (r14, FIQ) => self.registers.r14_fiq,
            (r14, SVC) => self.registers.r14_svc,
            (r14, ABT) => self.registers.r14_abt,
            (r14, IRQ) => self.registers.r14_irq,
            (r14, UND) => self.registers.r14_und,
            (r14, _) => self.registers.r14,
            (r15, _) => self.registers.r15,

            // Control registers
            (cpsr, _) => self.registers.cpsr,
            (spsr, FIQ) => self.registers.spsr_fiq,
            (spsr, SVC) => self.registers.spsr_svc,
            (spsr, ABT) => self.registers.spsr_abt,
            (spsr, IRQ) => self.registers.spsr_irq,
            (spsr, UND) => self.registers.spsr_und,
            // This register is the only one that is unaccessable in certain
            // execution modes.
            (spsr, _) => panic!("Attempting to use spsr register while not in a priveledged mode"),
        }
    }

    pub fn set_register_value(&mut self, name: Arm7RegisterNames, value: u32) {
        use Arm7OperationModes::*;
        use Arm7RegisterNames::*;

        let mode = self.get_operation_mode().unwrap();

        match (name, mode) {
            (r0, _) => self.registers.r0 = value,
            (r1, _) => self.registers.r1 = value,
            (r2, _) => self.registers.r2 = value,
            (r3, _) => self.registers.r3 = value,
            (r4, _) => self.registers.r4 = value,
            (r5, _) => self.registers.r5 = value,
            (r6, _) => self.registers.r6 = value,
            (r7, _) => self.registers.r7 = value,
            (r8, FIQ) => self.registers.r8_fiq = value,
            (r8, _) => self.registers.r8 = value,
            (r9, FIQ) => self.registers.r9_fiq = value,
            (r9, _) => self.registers.r9 = value,
            (r10, FIQ) => self.registers.r10_fiq = value,
            (r10, _) => self.registers.r10 = value,
            (r11, FIQ) => self.registers.r11_fiq = value,
            (r11, _) => self.registers.r11 = value,
            (r12, FIQ) => self.registers.r12_fiq = value,
            (r12, _) => self.registers.r12 = value,
            (r13, FIQ) => self.registers.r13_fiq = value,
            (r13, SVC) => self.registers.r13_svc = value,
            (r13, ABT) => self.registers.r13_abt = value,
            (r13, IRQ) => self.registers.r13_irq = value,
            (r13, UND) => self.registers.r13_und = value,
            (r13, _) => self.registers.r13 = value,
            (r14, FIQ) => self.registers.r14_fiq = value,
            (r14, SVC) => self.registers.r14_svc = value,
            (r14, ABT) => self.registers.r14_abt = value,
            (r14, IRQ) => self.registers.r14_irq = value,
            (r14, UND) => self.registers.r14_und = value,
            (r14, _) => self.registers.r14 = value,
            (r15, _) => self.registers.r15 = value,

            // We might want to protect these from writes, but we also
            // might not need to.
            (cpsr, _) => self.registers.cpsr = value,
            (spsr, FIQ) => self.registers.spsr_fiq = value,
            (spsr, SVC) => self.registers.spsr_svc = value,
            (spsr, ABT) => self.registers.spsr_abt = value,
            (spsr, IRQ) => self.registers.spsr_irq = value,
            (spsr, UND) => self.registers.spsr_und = value,
            // This register cannot be accessed from other modes.
            (spsr, _) => panic!("Attempting to use spsr register while not in a priveledged mode"),
        };
    }

    pub fn get_n(&self) -> bool {
        self.registers.cpsr >> 31 & 1 > 0
    }

    pub fn get_z(&self) -> bool {
        self.registers.cpsr >> 30 & 1 > 0
    }

    pub fn get_c(&self) -> bool {
        self.registers.cpsr >> 29 & 1 > 0
    }

    pub fn get_v(&self) -> bool {
        self.registers.cpsr >> 28 & 1 > 0
    }

    pub fn set_nzcv(&mut self, n: bool, z: bool, c: bool, v: bool) {
        let mut flags = 0;

        if n { flags |= 0x8 }
        if z { flags |= 0x4 }
        if c { flags |= 0x2 }
        if v { flags |= 0x1 }

        self.registers.cpsr = (self.registers.cpsr & 0x0fffffff) | (flags << 28);
    }

    pub fn set_fiq_disable(&mut self, disabled: bool) {
        if disabled {
            self.registers.cpsr |= Arm7TdmiRegisters::FIQ_DISABLE;
        } else {
            self.registers.cpsr &= !Arm7TdmiRegisters::FIQ_DISABLE;
        }
    }

    pub fn is_fiq_disabled(&self) -> bool {
        (self.registers.cpsr & Arm7TdmiRegisters::FIQ_DISABLE) > 0
    }

    pub fn set_irq_disable(&mut self, disabled: bool) {
        if disabled {
            self.registers.cpsr |= Arm7TdmiRegisters::IRQ_DISABLE;
        } else {
            self.registers.cpsr &= !Arm7TdmiRegisters::IRQ_DISABLE;
        }
    }

    pub fn is_irq_disabled(&self) -> bool {
        (self.registers.cpsr & Arm7TdmiRegisters::IRQ_DISABLE) > 0
    }

    pub fn set_thumb_bit(&mut self, thumb: bool) {
        if thumb {
            self.registers.cpsr |= Arm7TdmiRegisters::THUMB_BIT;
        } else {
            self.registers.cpsr &= !Arm7TdmiRegisters::THUMB_BIT;
        }
    }

    pub fn get_thumb_bit(&self) -> bool {
        (self.registers.cpsr & Arm7TdmiRegisters::THUMB_BIT) > 0
    }

    pub fn set_operation_mode(&mut self, mode: Arm7OperationModes) {
        // When switching to priviledge, should store cpsr in spsr, as
        // well as the current PC (r15) in LR (r14), and then change modes
        // When switching back, should load cpsr from spsr
        let mode_flags: u32 = mode.into();
        self.registers.cpsr = (self.registers.cpsr & 0xffffffe0) | mode_flags;
    }

    pub fn get_operation_mode(&self) -> Option<Arm7OperationModes> {
        match Arm7OperationModes::try_from(self.registers.cpsr & 0b11111) {
            Ok(mode) => Some(mode),
            Err(_) => None,
        }
    }

    pub fn check_condition(&self, cond: Arm7ConditionCodes) -> bool {
        use Arm7ConditionCodes::*;

        match cond {
            EQ => self.get_z(),  // Equals - Z set
            NE => !self.get_z(), // Not equal - Z clear
            CS => self.get_c(),  // Carry set/unsigned higher or same - C set
            CC => !self.get_c(), // Carry clear/unsigned lower - C clear
            MI => self.get_n(),  // Minus/negative - N set
            PL => !self.get_n(), // Plus/positive and zero - N clear
            VS => self.get_v(),  // Overflow set - V set
            VC => !self.get_v(), // Overflow clear - V clear

            // Unsigned higher - C set and Z clear
            HI => self.get_c() && !self.get_z(),
            // Unsigned lower or same - C clear or Z set
            LS => !self.get_c() || self.get_z(),
            // Signed greater than or equal - N set and V set, or N clear and V clear (N == V)
            GE => self.get_n() == self.get_v(),
            // Signed less than - N set and V clear, or N clear and V set (N != V)
            LT => self.get_n() != self.get_v(),
            // Signed greater than - Z clear, and either N set and V set, or N clear and V clear (Z == 0,N == V)
            GT => !self.get_z() && (self.get_n() == self.get_v()),
            // Signed less than or equal - Z set, or N set and V clear, or N clear and V set (Z == 1 or N != V)
            LE => self.get_z() && (self.get_n() != self.get_v()),

            AL => true,                                     // Always, unconditional
            UND => panic!("Unpredictable condition code!"), // Undefined, unpredictable
        }
    }
}

#[derive(Default)]
pub struct Arm7TdmiRegisters {
    // General purpose registers
    pub r0: u32,
    pub r1: u32,
    pub r2: u32,
    pub r3: u32,
    pub r4: u32,
    pub r5: u32,
    pub r6: u32,
    pub r7: u32,
    pub r8: u32,
    pub r8_fiq: u32,
    pub r9: u32,
    pub r9_fiq: u32,
    pub r10: u32,
    pub r10_fiq: u32,
    pub r11: u32,
    pub r11_fiq: u32,
    pub r12: u32,
    pub r12_fiq: u32,

    // Stack pointer, but only by convention, not actually enforced.
    // Known as sp in thumb
    pub r13: u32,
    pub r13_fiq: u32,
    pub r13_svc: u32,
    pub r13_abt: u32,
    pub r13_irq: u32,
    pub r13_und: u32,

    // Link register
    // Basically, the address to jump back to once a subroutine
    // or interupt has completed execution.
    // Known as lr in thumb
    pub r14: u32,
    pub r14_fiq: u32,
    pub r14_svc: u32,
    pub r14_abt: u32,
    pub r14_irq: u32,
    pub r14_und: u32,

    // Program counter
    // Known as pc in thumb
    // In ARM, the 2 least significant bits should always be zero
    // In Thumb, the least significant bit should always be zero
    // If in a branch instruction bit 0 is set to 1, then the thumb bit should flip.
    pub r15: u32,

    // Status registers
    // Current program state register
    // https://developer.arm.com/docs/ddi0210/latest/programmers-model/the-program-status-registers
    // # Condition codes
    // Set by arithmatic instructions, as well as MSR and LDM
    // N - Negative or less than [31]
    // Z - Zero [30]
    // C - Carry, borrow, extend [29]
    // V - Overflow [28]
    // # Reserved [27:8]
    // IRQ Disable [7]
    // FIQ Disable [6]
    // Thumb bit [5]
    // Mode [4:0]
    // An illegal mode value should trigger a reset
    pub cpsr: u32,
    // Stored program state register
    // Stores information on the user program while in a priveledged mode.
    // Should probably be restored to cpsr when leaving priviledge.
    // Stored from cpsr before mode switching.
    pub spsr_fiq: u32,
    pub spsr_svc: u32,
    pub spsr_abt: u32,
    pub spsr_irq: u32,
    pub spsr_und: u32,
}

impl Arm7TdmiRegisters {
    const IRQ_DISABLE: u32 = 1 << 7;
    const FIQ_DISABLE: u32 = 1 << 6;
    const THUMB_BIT: u32 = 1 << 5;

    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u32)]
pub enum Arm7OperationModes {
    USR = 0b10000, // Normal execution
    FIQ = 0b10001, // Fast interupt
    IRQ = 0b10010, // Interupt
    SVC = 0b10011, // Service/supervisor
    SYS = 0b11111, // System operation, can only be entered from another priviledged mode
    ABT = 0b10111, // Abort
    UND = 0b11011, // Undefined, entered from invalid opcodes
}

// These are just the names, not the actual register values. They are used
// in conjunction with the {get,set}_register_value functions to automatically
// handle using the correct register for the execution mode.
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u32)]
pub enum Arm7RegisterNames {
    r0,
    r1,
    r2,
    r3,
    r4,
    r5,
    r6,
    r7,
    r8,
    r9,
    r10,
    r11,
    r12,
    r13,
    r14,
    r15,

    cpsr,
    spsr,
}

#[derive(Copy, Clone, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u32)]
pub enum Arm7ConditionCodes {
    EQ = 0b0000, // Equals - Z set
    NE = 0b0001, // Not equal - Z clear
    CS = 0b0010, // Carry set/unsigned higher or same - C set
    CC = 0b0011, // Carry clear/unsigned lower - C clear
    MI = 0b0100, // Minus/negative - N set
    PL = 0b0101, // Plus/positive and zero - N clear
    VS = 0b0110, // Overflow set - V set
    VC = 0b0111, // Overflow clear - V clear

    HI = 0b1000, // Unsigned higher - C set and Z clear
    LS = 0b1001, // Unsigned lower or same - C clear or Z set
    GE = 0b1010, // Signed greater than or equal - N set and V set, or N clear and V clear (N == V)
    LT = 0b1011, // Signed less than - N set and V clear, or N clear and V set (N != V)
    GT = 0b1100, // Signed greater than - Z clear, and either N set and V set, or N clear and V clear (Z == 0,N == V)
    LE = 0b1101, // Signed less than or equal - Z set, or N set and V clear, or N clear and V set (Z == 1 or N != V)

    AL = 0b1110,  // Always, no conditions
    UND = 0b1111, // Undefined, unpredictable
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn set_frequency() {
        let mut cpu = Arm7Tdmi::init();

        cpu.set_frequency(16780000);
        assert_eq!(cpu.frequency, 16780000);
    }

    #[test]
    fn set_operation_mode() {
        let mut cpu = Arm7Tdmi::init();

        // SYS by default
        assert_eq!(cpu.get_operation_mode(), Some(Arm7OperationModes::SYS));

        // Change into UND mode
        cpu.set_operation_mode(Arm7OperationModes::UND);
        assert_eq!(cpu.get_operation_mode(), Some(Arm7OperationModes::UND));

        // Change into USR mode
        cpu.set_operation_mode(Arm7OperationModes::USR);
        assert_eq!(cpu.get_operation_mode(), Some(Arm7OperationModes::USR));
    }

    #[test]
    fn thumb_bit() {
        let mut cpu = Arm7Tdmi::init();

        // Off by default
        assert_eq!(cpu.get_thumb_bit(), false);
        cpu.set_thumb_bit(true);
        assert_eq!(cpu.get_thumb_bit(), true);
        cpu.set_thumb_bit(false);
        assert_eq!(cpu.get_thumb_bit(), false);
    }

    #[test]
    fn register_mapping() {
        let mut cpu = Arm7Tdmi::init();

        cpu.set_operation_mode(Arm7OperationModes::SVC);
        cpu.set_register_value(Arm7RegisterNames::r13, 0xdeadbeef);
        assert_eq!(cpu.get_register_value(Arm7RegisterNames::r13), 0xdeadbeef);

        cpu.set_operation_mode(Arm7OperationModes::USR);
        assert_eq!(cpu.get_register_value(Arm7RegisterNames::r13), 0);
    }

    #[test]
    fn interupts() {
        let mut cpu = Arm7Tdmi::init();

        // Disabled by default
        assert!(cpu.is_fiq_disabled());
        assert!(cpu.is_irq_disabled());

        // Enable them
        cpu.set_fiq_disable(false);
        cpu.set_irq_disable(false);
        assert!(!cpu.is_fiq_disabled());
        assert!(!cpu.is_irq_disabled());
    }

    #[test]
    fn get_condition_bits() {
        let mut cpu = Arm7Tdmi::init();

        // Should all be off by default
        assert!(!cpu.get_n());
        assert!(!cpu.get_z());
        assert!(!cpu.get_c());
        assert!(!cpu.get_v());

        // Set all of the flag bits.
        cpu.registers.cpsr |= 0xf0000000;

        // Should now all be on
        assert!(cpu.get_n());
        assert!(cpu.get_z());
        assert!(cpu.get_c());
        assert!(cpu.get_v());
    }

    #[test]
    fn set_condition_bits() {
        let mut cpu = Arm7Tdmi::init();

        // Should all be off by default
        assert!(!cpu.get_n());
        assert!(!cpu.get_z());
        assert!(!cpu.get_c());
        assert!(!cpu.get_v());

        // Make sure they stay off
        cpu.set_nzcv(false, false, false, false);

        // Should still all be off
        assert!(!cpu.get_n());
        assert!(!cpu.get_z());
        assert!(!cpu.get_c());
        assert!(!cpu.get_v());

        // Set all of the flag bits
        cpu.set_nzcv(true, true, true, true);

        // Should now all be on
        assert!(cpu.get_n());
        assert!(cpu.get_z());
        assert!(cpu.get_c());
        assert!(cpu.get_v());

        // Set all of the flag bits
        cpu.set_nzcv(true, true, true, true);

        // Should still all be on
        assert!(cpu.get_n());
        assert!(cpu.get_z());
        assert!(cpu.get_c());
        assert!(cpu.get_v());
    }

    #[test]
    fn conditions() {
        let mut cpu = Arm7Tdmi::init();

        // z bit should be zero by default, so this condition is true
        assert!(cpu.check_condition(Arm7ConditionCodes::NE));

        // Turn on the z bit
        cpu.registers.cpsr = cpu.registers.cpsr | 0x40000000;

        // EQ should pass, NE should not
        assert!(cpu.check_condition(Arm7ConditionCodes::EQ));
        assert!(!cpu.check_condition(Arm7ConditionCodes::NE));
    }
}

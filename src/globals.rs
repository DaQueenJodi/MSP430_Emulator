pub const PC: u8 = 0; // Program Counter
pub const SP: u8 = 1; // Stack Pointer
pub const SR: u8 = 2; // Status Register
pub const ZR: u8 = 3; // Zero Register

#[derive(Clone, Copy, Debug)]
pub enum Word {
    Signed(i16),
    Unigned(u16),
}
#[derive(Clone, Copy, Debug)]
pub struct Address(pub u64);

#[derive(Clone, Copy, Debug)]
pub enum JmpCondition {
    JNE,
    JEQ,
    JLO,
    JHS,
    JN,
    JGE,
    JL,
    JMP,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OneOpcode {
    RRC,
    SWPB,
    RRA,
    SXT,
    PUSH,
    CALL,
    RETI,
}
#[derive(Clone, Copy, Debug)]
pub enum TwoOpcode {
    MOV,
    ADD,
    ADDC,
    SUBC,
    SUB,
    CMP,
    DADD,
    BIT,
    BIC,
    BIS,
    XOR,
    AND,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AddressMode {
    // GLOBAL
    Direct,
    Indirect,
    IndirectIncrement,
    Indexed,

    // SR
    AbsoluteAddressing,
    Const4,
    Const8,

    // ZR
    Const0,
    Const1,
    Const2,
    ConstNeg1,
}

#[derive(Clone, Copy, Debug)]
pub struct Indexing(pub u8);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Register {
    pub value: u64,
    pub reg_num: u8,
}
#[derive(Clone, Copy, Debug)]
pub struct RegisterArg {
    pub reg_num: u8,
    pub am: AddressMode,
    pub value: u64,
}

#[derive(Clone, Copy, Debug)]
pub struct Bbit(pub bool);

#[derive(Clone, Copy, Debug)]
pub struct Offset(pub i16);

#[derive(Clone, Copy, Debug)]
pub enum Instruction {
    Invalid,
    JMP {
        condition: JmpCondition,
        offset: Offset,
    },
    ONE {
        opcode: OneOpcode,
        b: Bbit,
        dest: RegisterArg,
    },
    TWO {
        opcode: TwoOpcode,
        src: RegisterArg,
        b: Bbit,
        dest: RegisterArg,
    },
    PSEUDO {
        opcode: PsuedoOpcode,
        b: Bbit,
        dest: Option<RegisterArg>,
    },
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PsuedoOpcode {
    NOP,
    POP,

    BR,
    RET,

    CLRC,
    SETC,
    CLRZ,
    SETZ,
    CLRN,
    SETN,
    DINT,
    EINT,

    RLA,
    RLC,

    INV,
    CLR,
    TST,

    DEC,
    DECD,
    INC,
    INCD,

    ADC,
    DADC,
    SBC,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct SignedWord(i16);

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum InstructionFlavor {
    JMP,
    ONE,
    TWO,
}

pub enum RegisterFlavor {
    Address,
    RegisterNumber,
}

use core::panic;
use std::ops::{Add, AddAssign};

use bitvec::{order::Lsb0, view::BitView};

use crate::{
    emu_logic::{
        get_next_word, get_word_at, Ctx, ProgramCounter, StackPointer, StatusRegister, Statuses,
        ZeroRegister, STATUS_C, STATUS_CPUOFF, STATUS_GIE, STATUS_N, STATUS_OSOFF, STATUS_SCGI,
        STATUS_SCGO, STATUS_V, STATUS_Z,
    },
    globals::{
        Address, AddressMode, Bbit, Instruction, InstructionFlavor, JmpCondition, Offset,
        OneOpcode, Register, RegisterArg, RegisterFlavor, TwoOpcode, Word, PC, SR, ZR,
    },
};

impl Word {
    pub fn get_unsigned(&self) -> u16 {
        match *self {
            Word::Signed(n) => n as u16,
            Word::Unigned(n) => n,
        }
    }
    pub fn get_signed(&self) -> i16 {
        match *self {
            Word::Signed(n) => n,
            Word::Unigned(n) => n as i16,
        }
    }
}

impl JmpCondition {
    pub fn is_true(&self, ctx: &mut Ctx) -> bool {
        use JmpCondition::*;
        match *self {
            JEQ => ctx.check_eq(),
            JGE => ctx.check_eq(),
            JHS => ctx.check_hs(),
            JL => ctx.check_l(),
            JLO => ctx.check_lo(),
            JMP => true,
            JN => ctx.check_n(),
            JNE => ctx.check_ne(),
        }
    }
}

impl Ctx {
    pub fn new() -> Ctx {
        Ctx {
            pc: ProgramCounter(0),
            sp: StackPointer(0),
            sr: StatusRegister(0),
            zr: ZeroRegister(0),
            r4: Register::new(4),
            r5: Register::new(5),
            r6: Register::new(6),
            r7: Register::new(7),
            r8: Register::new(8),
            r9: Register::new(9),
            r10: Register::new(10),
            r11: Register::new(11),
            r12: Register::new(12),
            r13: Register::new(13),
            r14: Register::new(14),
            r15: Register::new(15),
        }
    }
}

impl StatusRegister {
    pub fn check(&self, status: Statuses) -> bool {
        let bits = self.0.view_bits::<Lsb0>();

        bits[status.get_offset()]
    }
}
use Statuses::*;
impl Statuses {
    pub fn get_offset(&self) -> usize {
        match self {
            C => STATUS_C,
            Z => STATUS_Z,
            N => STATUS_N,
            GIE => STATUS_GIE,
            CPUOFF => STATUS_CPUOFF,
            OSOFF => STATUS_OSOFF,
            SCGO => STATUS_SCGO,
            SCGI => STATUS_SCGI,
            V => STATUS_V,
        }
    }
}

impl AddAssign<Offset> for ProgramCounter {
    fn add_assign(&mut self, rhs: Offset) {
        *self = *self + rhs;
    }
}

impl Add<Offset> for ProgramCounter {
    type Output = ProgramCounter;

    fn add(self, rhs: Offset) -> Self::Output {
        ProgramCounter(self.0 + rhs.0 as u64)
    }
}

impl Register {
    pub fn new(num: u8) -> Register {
        Register {
            value: 0,
            reg_num: num,
        }
    }
}
use AddressMode::*;
impl RegisterArg {
    pub fn get_real_src(&mut self, ctx: &mut Ctx) -> i64 {
        match self.am {
            Indexed => {
                let offset = get_next_word(ctx).get_signed();
                let num = get_word_at(Address(self.value + offset as u64))
                    .get_unsigned()
                    .into();
                num
            }
            Direct => {
                let num = self.value as i64;
                num
            }
            Indirect => {
                let num;
                if self.reg_num == PC {
                    num = get_next_word(ctx).get_unsigned().into();
                } else {
                    num = get_word_at(Address(self.value)).get_unsigned().into()
                }
                num
            }
            AbsoluteAddressing => {
                let num = get_word_at(Address(get_next_word(ctx).get_unsigned().into()))
                    .get_unsigned()
                    .into();
                num
            }
            Const0 => 0,
            Const1 => 1,
            Const2 => 2,
            Const4 => 4,
            Const8 => 8,
            ConstNeg1 => -1,
            IndirectIncrement => {
                let num;
                if self.reg_num == PC {
                    num = get_next_word(ctx).get_unsigned().into();
                } else {
                    num = get_word_at(Address(self.value)).get_unsigned().into();
                }
                self.increment_real(ctx);

                num
            }
        }
    }
    pub fn get_real_dest(&mut self, ctx: &mut Ctx) -> (RegisterFlavor, u64) {
        match self.am {
            Const0 => (RegisterFlavor::RegisterNumber, ZR as u64),
            Indexed => {
                // TODO make exception for PC
                let offset = get_next_word(ctx).get_signed();
                (RegisterFlavor::Address, self.value + (offset as u64))
            }
            AbsoluteAddressing => (
                RegisterFlavor::Address,
                get_next_word(ctx).get_unsigned().into(),
            ),
            Direct => (RegisterFlavor::RegisterNumber, self.reg_num.into()),
            _ => panic!("calling get_real_dest on non-dest register!"),
        }
    }
}

impl Into<Address> for u16 {
    fn into(self) -> Address {
        Address(self as u64)
    }
}

impl RegisterArg {
    pub fn increment_real(&self, ctx: &mut Ctx) {
        match self.reg_num {
            PC => ctx.pc.increment(),
            SP => ctx.sp.increment(),
            SR => ctx.sr.increment(), // when the hell would you use this lmao
            ZR => (),
            4 => ctx.r4.increment(),
            5 => ctx.r5.increment(),
            6 => ctx.r6.increment(),
            7 => ctx.r7.increment(),
            8 => ctx.r8.increment(),
            9 => ctx.r9.increment(),
            10 => ctx.r10.increment(),
            11 => ctx.r11.increment(),
            12 => ctx.r12.increment(),
            13 => ctx.r13.increment(),
            14 => ctx.r14.increment(),
            15 => ctx.r15.increment(),
        }
    }
}
impl Register {
    pub fn increment(&mut self) {
        self.value += 1;
    }
}
impl ProgramCounter {
    pub fn increment(&mut self) {
        self.0 += 2; // step by 2 bytes
    }
}
impl StatusRegister {
    pub fn increment(&mut self) {
        todo!()
    }
}
impl StackPointer {
    pub fn increment(&mut self) {
        self.0 += 2;
    }
}

impl Instruction {
    pub fn new_one(opcode: OneOpcode, b: Bbit, dam: AddressMode, dest_num: u8) -> Instruction {
        Instruction::ONE {
            opcode,
            b,
            dest: RegisterArg {
                reg_num: dest_num,
                am: dam,
                value: 0,
            },
        }
    }
    pub fn new_two(
        opcode: TwoOpcode,
        b: Bbit,
        sam: AddressMode,
        src_num: u8,
        dam: AddressMode,
        dest_num: u8,
    ) -> Instruction {
        Instruction::TWO {
            opcode,
            src: RegisterArg {
                reg_num: src_num,
                am: sam,
                value: 0,
            },
            b,
            dest: RegisterArg {
                reg_num: dest_num,
                am: dam,
                value: 0,
            },
        }
    }
}

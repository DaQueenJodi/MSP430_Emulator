use crate::globals::{
    Address, AddressMode, Bbit, Instruction, JmpCondition, Offset, Register, RegisterArg, Word,
};

pub const STATUS_C: usize = 0;
pub const STATUS_Z: usize = 1;
pub const STATUS_N: usize = 2;
pub const STATUS_GIE: usize = 3;
pub const STATUS_CPUOFF: usize = 4;
pub const STATUS_OSOFF: usize = 5;
pub const STATUS_SCGO: usize = 6;
pub const STATUS_SCGI: usize = 7;
pub const STATUS_V: usize = 8;

#[derive(Clone, Copy)]
pub struct ProgramCounter(pub u64);
#[derive(Clone, Copy)]
pub struct StackPointer(pub u64);

#[derive(Clone, Copy)]
pub struct StatusRegister(pub u64);
#[derive(Clone, Copy)]
pub struct ZeroRegister(pub u8); // always 0!

pub struct Ctx {
    pub pc: ProgramCounter,
    pub sp: StackPointer,
    pub sr: StatusRegister,
    pub zr: ZeroRegister,
    pub r4: Register,
    pub r5: Register,
    pub r6: Register,
    pub r7: Register,
    pub r8: Register,
    pub r9: Register,
    pub r10: Register,
    pub r11: Register,
    pub r12: Register,
    pub r13: Register,
    pub r14: Register,
    pub r15: Register,
}

pub fn do_jump_instruction(ctx: &mut Ctx, instruction: Instruction) {
    match instruction {
        Instruction::JMP { condition, offset } => {
            if condition.is_true(ctx) {
                relative_jump(ctx, offset);
            }
        }
        _ => panic!("non-JMP function called 'do_jump_instruction'"),
    }
}

fn relative_jump(ctx: &mut Ctx, offset: Offset) {
    ctx.pc += offset;
}

use Statuses::*;
impl Ctx {
    pub fn check_eq(&self) -> bool {
        self.sr.check(Z)
    }
    pub fn check_ge(&self) -> bool {
        self.sr.check(V) == self.sr.check(N)
    }
    pub fn check_hs(&self) -> bool {
        self.sr.check(C)
    }
    pub fn check_l(&self) -> bool {
        self.sr.check(N) != self.sr.check(V)
    }
    pub fn check_lo(&self) -> bool {
        !self.sr.check(C)
    }
    pub fn check_n(&self) -> bool {
        self.sr.check(N)
    }
    pub fn check_ne(&self) -> bool {
        !self.sr.check(Z)
    }
}

pub enum Statuses {
    C,
    Z,
    N,
    GIE,
    CPUOFF,
    OSOFF,
    SCGO,
    SCGI,
    V,
}

pub fn get_next_word(ctx: &mut Ctx) -> Word {
    let adddress = Address((ctx.pc + Offset(2)).0);
    todo!()
}

pub fn get_word_at(address: Address) -> Word {
    todo!()
}

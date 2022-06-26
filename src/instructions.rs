use crate::{
    emu_logic::Ctx,
    globals::{Bbit, RegisterArg},
};

pub fn mov(ctx: &mut Ctx, b: Bbit, src: RegisterArg, dest: RegisterArg) {
    let src = src.get_real_src(ctx);
    let dest = dest.get_real_dest(ctx);
}

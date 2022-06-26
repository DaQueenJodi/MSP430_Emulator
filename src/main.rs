use emu_logic::Ctx;

mod decoder;
mod emu_logic;
mod globals;
mod implementations;
mod instructions;
mod maps;

fn main() {
    let ctx = Ctx::new();
    while true {
        decoder::decode_instruction();
    }
}

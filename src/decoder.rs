use bitvec::order::Lsb0;
use bitvec::prelude::*;
use std::{
    collections::btree_map::Range,
    ops::{Index, RangeInclusive},
};

use crate::{
    globals::{
        Address, AddressMode, Bbit, Instruction, InstructionFlavor, Offset, Word, PC, SR, ZR,
    },
    maps::{
        ADDRESS_MODE_MAP, ADDRESS_MODE_SR_MAP, ADDRESS_MODE_ZR_MAP, JUMP_MAP, ONE_MAP, TWO_MAP,
    },
};

const ONE_DEST: RangeInclusive<usize> = 0..=3;
const ONE_AD: RangeInclusive<usize> = 4..=5;
const ONE_B: usize = 5;
const ONE_OP: RangeInclusive<usize> = 7..=9;

const JMP_OFFSET: RangeInclusive<usize> = 0..=9;
const JMP_COND: RangeInclusive<usize> = 10..=12;

const TWO_DEST: RangeInclusive<usize> = 0..=3;
const TWO_AS: RangeInclusive<usize> = 4..=5;
const TWO_B: usize = 6;
const TWO_AD: usize = 7;
const TWO_SRC: RangeInclusive<usize> = 8..=11;
const TWO_OP: RangeInclusive<usize> = 12..=15;

pub fn decode_instruction(word: Word) -> Instruction {
    use InstructionFlavor::*;
    let flavor = get_flavor(word);
    let word = word.get_unsigned();
    let bits = word.view_bits::<Lsb0>();

    match flavor {
        ONE => {
            let opcode = *ONE_MAP.get(&bits[ONE_OP].load()).unwrap();

            let b = Bbit(bits[ONE_B]);

            let dest = bits[ONE_DEST].load();

            let possible_dam = bits[ONE_AD].load();
            let dam = *match dest {
                SR => ADDRESS_MODE_SR_MAP.get(&possible_dam).unwrap(),
                ZR => ADDRESS_MODE_ZR_MAP.get(&possible_dam).unwrap(),
                _ => ADDRESS_MODE_MAP.get(&possible_dam).unwrap(),
            };

            return Instruction::new_one(opcode, b, dam, dest);
        }
        TWO => {
            let opcode = *TWO_MAP.get(&bits[TWO_OP].load()).unwrap();
            let src = bits[TWO_SRC].load();
            let b = Bbit(bits[TWO_B]);
            let dest = bits[TWO_DEST].load();

            let possible_sam = bits[TWO_AS].load();
            let sam = *match src {
                SR => ADDRESS_MODE_SR_MAP.get(&possible_sam).unwrap(),
                ZR => ADDRESS_MODE_ZR_MAP.get(&possible_sam).unwrap(),
                _ => ADDRESS_MODE_MAP.get(&possible_sam).unwrap(),
            };

            let possible_dam = bits[TWO_AD] as u8;
            let dam = *match dest {
                SR => ADDRESS_MODE_SR_MAP.get(&possible_dam).unwrap(),
                ZR => ADDRESS_MODE_ZR_MAP.get(&possible_dam).unwrap(),
                _ => ADDRESS_MODE_MAP.get(&possible_dam).unwrap(),
            };

            Instruction::new_two(opcode, b, sam, src, dam, dest)
        }
        JMP => {
            let condition = *JUMP_MAP.get(&bits[JMP_COND].load()).unwrap();
            let offset = Offset(bits[JMP_OFFSET].load());

            Instruction::JMP { condition, offset }
        }
    }
}

fn get_flavor(word: Word) -> InstructionFlavor {
    use InstructionFlavor::*;
    let word = word.get_unsigned();
    let bits = word.view_bits::<Lsb0>();
    if bits[10..16] == bits![0, 0, 1, 0, 0, 0] {
        ONE
    } else if bits[13..16] == bits![1, 0, 0] {
        JMP
    } else {
        TWO
    }
}

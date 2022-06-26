use std::collections::HashMap;

use crate::globals::*;
use lazy_static::lazy_static;
lazy_static! {
    pub static ref JUMP_MAP: HashMap<u8, JmpCondition> = {
        use JmpCondition::*;
        let mut m = HashMap::new();
        m.insert(0b000, JNE);
        m.insert(0b001, JEQ);
        m.insert(0b010, JLO);
        m.insert(0b011, JHS);
        m.insert(0b100, JN);
        m.insert(0b101, JGE);
        m.insert(0b110, JL);
        m.insert(0b111, JMP);
        m
    };
    pub static ref ONE_MAP: HashMap<u8, OneOpcode> = {
        use OneOpcode::*;
        let mut m = HashMap::new();
        m.insert(0b000, RRC);
        m.insert(0b001, SWPB);
        m.insert(0b010, RRA);
        m.insert(0b011, SXT);
        m.insert(0b100, PUSH);
        m.insert(0b101, CALL);
        m.insert(0b110, RETI);
        m
    };
    pub static ref TWO_MAP: HashMap<u8, TwoOpcode> = {
        use TwoOpcode::*;
        let mut m = HashMap::new();
        m.insert(0b0100, MOV);
        m.insert(0b0101, ADD);
        m.insert(0b0110, ADDC);
        m.insert(0b0111, SUBC);
        m.insert(0b1000, SUB);
        m.insert(0b1001, CMP);
        m.insert(0b1010, DADD);
        m.insert(0b1011, BIT);
        m.insert(0b1100, BIC);
        m.insert(0b1101, BIS);
        m.insert(0b1110, XOR);
        m.insert(0b1111, AND);
        m
    };
    pub static ref ADDRESS_MODE_MAP: HashMap<u8, AddressMode> = {
        use AddressMode::*;
        let mut m = HashMap::new();
        // 2 bit for src)
        m.insert(0b00, Direct);
        m.insert(0b01, Indexed);
        m.insert(0b10, Indirect);
        m.insert(0b11, IndirectIncrement);
        m
    };

    pub static ref ADDRESS_MODE_SR_MAP: HashMap<u8, AddressMode> = {
        use AddressMode::*;
        let mut m = HashMap::new();
        m.insert(0b00, Direct);
        m.insert(0b01, AbsoluteAddressing);
        m.insert(0b10, Const4);
        m.insert(0b11, Const8);
        m
    };
    pub static ref ADDRESS_MODE_ZR_MAP: HashMap<u8, AddressMode> = {
        use AddressMode::*;
        let mut m = HashMap::new();

        m.insert(0b00, Const0);
        m.insert(0b01, Const1);
        m.insert(0b10, Const2);
        m.insert(0b11, ConstNeg1);
        m
    };

}

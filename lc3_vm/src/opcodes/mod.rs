use std::io;

use crate::Registers;
//import enum ConditionFlags from flags.rs
use crate::flags::ConditionFlags;

#[derive(Debug)]
pub enum Opcode{
    BR = 0, // branch
    ADD,    // add
    LD,     // load
    ST,     // store
    JSR,    // jump register
    AND,    // bitwise and
    LDR,    // load register
    STR,    // store register
    RTI,    // unused
    NOT,    // bitwise not
    LDI,    // load indirect
    STI,    // store indirect
    JMP,    // jump
    RES,    // reserved (unused)
    LEA,    // load effective address
    TRAP,   // execute trap
}

pub fn sign_extend(x: u16, bit_count: u16) -> u16 {
    if (x >> (bit_count - 1)) & 1 == 1 {
        x | (0xFFFF << bit_count)
    } else {
        x
    }
}


fn update_flags(r: u16, register: &mut Registers) ->Result<(),io::Error> {
    if register.get(r)? == 0 {
        let _ = register.update_cond(ConditionFlags::ZRO as u16);
    } else if (register.get(r)? >> 15) == 1 {
        let _ = register.update_cond(ConditionFlags::NEG as u16);
    } else {
        let _ = register.update_cond(ConditionFlags::POS as u16);
    }
    Ok(())
}


pub fn op_add(instr:u16,register:&mut Registers) ->Result<(),io::Error> {
    /* destination register (DR) */
    let r0 = (instr >> 9) & 0x7;
    /* first operand (SR1) */
    let r1 = (instr >> 6) & 0x7;
    /* whether we are in immediate mode */
    let imm_flag = (instr >> 5) & 0x1;

    if imm_flag == 1 {
        let imm5 = sign_extend(instr & 0x1F, 5);
        let _ = register.update(r0, register.get(r1)? + imm5);
    } else {
        let r2 = instr & 0x7;
        let val: u32 = register.get(r1)? as u32 + register.get(r2)? as u32;
        let _ = register.update(r0, val as u16);
    }

    update_flags(r0, register)?;
    Ok(())
}

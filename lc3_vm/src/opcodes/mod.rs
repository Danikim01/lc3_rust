use std::io;

use crate::Registers;
use crate::flags::ConditionFlags;
use crate::memory::*;

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

pub fn op_and(instr:u16,register:&mut Registers) ->Result<(),io::Error> {
    let r0 = (instr >> 9) & 0x7;
    let r1 = (instr >> 6) & 0x7;
    let imm_flag = (instr >> 5) & 0x1;

    if imm_flag == 1 {
        let imm5 = sign_extend(instr & 0x1F, 5);
        let _ = register.update(r0, register.get(r1)? & imm5);
    } else {
        let r2 = instr & 0x7;
        let val: u32 = register.get(r1)? as u32 & register.get(r2)? as u32;
        let _ = register.update(r0, val as u16);
    }

    update_flags(r0, register)?;
    Ok(())   
}

pub fn op_not(instr:u16,register:&mut Registers) ->Result<(),io::Error> {
    let r0 = (instr >> 9) & 0x7;
    let r1 = (instr >> 6) & 0x7;

    let val = !register.get(r1)?;
    let _ = register.update(r0, val);

    update_flags(r0, register)?;
    Ok(())
}

pub fn op_br(instr:u16,register:&mut Registers) ->Result<(),io::Error> {
    let pc_offset = sign_extend(instr & 0x1FF, 9);
    let cond_flag = (instr >> 9) & 0x7;

    if cond_flag & register.get_cond() != 0 {
        let _ = register.update_pc(register.get_pc() + pc_offset);
    }

    Ok(())
}

pub fn op_jmp(instr:u16,register:&mut Registers) ->Result<(),io::Error> {
    let r1 = (instr >> 6) & 0x7;
    let _ = register.update_pc(register.get(r1)?);
    Ok(())
}

pub fn op_jsr(instr:u16,register:&mut Registers) ->Result<(),io::Error> {
    let long_flag = (instr >> 11) & 1;
    let r1 = (instr >> 6) & 0x7;

    let _ = register.update(7, register.get_pc());
    if long_flag == 1 {
        let long_pc_offset = sign_extend(instr & 0x7FF, 11);
        let _ = register.update_pc(register.get_pc() + long_pc_offset);
    } else {
        let _ = register.update_pc(register.get(r1)?);
    }

    Ok(())
}

pub fn op_ld(instr:u16,register:&mut Registers,memory:&mut Vec<u16>) ->Result<(),io::Error> {
    let r0 = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1FF, 9);
    let value = mem_read(register.get_pc() + pc_offset, memory);
    let _ = register.update(r0, value);
    update_flags(r0, register)?;
    Ok(())
}

pub fn op_ldi(instr:u16,register:&mut Registers,memory:&mut Vec<u16>) ->Result<(),io::Error> {
    let r0 = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1FF, 9);
    let value = mem_read(mem_read(register.get_pc() + pc_offset, memory), memory);
    let _ = register.update(r0, value);
    update_flags(r0, register)?;
    Ok(())
}

pub fn op_ldr(instr:u16,register:&mut Registers,memory:&mut Vec<u16>) ->Result<(),io::Error> {
    let r0 = (instr >> 9) & 0x7;
    let r1 = (instr >> 6) & 0x7;
    let offset = sign_extend(instr & 0x3F, 6);
    let value = mem_read(register.get(r1)? + offset, memory);
    let _ = register.update(r0, value);
    update_flags(r0, register)?;
    Ok(())
}

pub fn op_lea(instr:u16,register:&mut Registers) ->Result<(),io::Error> {
    let r0 = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1FF, 9);
    let _ = register.update(r0, register.get_pc() + pc_offset);
    update_flags(r0, register)?;
    Ok(())
}

pub fn op_st(instr:u16,register:&mut Registers,memory:&mut Vec<u16>) ->Result<(),io::Error> {
    let r0 = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1FF, 9);
    let _ = mem_write(register.get_pc() + pc_offset, register.get(r0)?, memory);
    Ok(())
}

pub fn op_sti(instr:u16,register:&mut Registers,memory:&mut Vec<u16>) ->Result<(),io::Error> {
    let r0 = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1FF, 9);
    let _ = mem_write(mem_read(register.get_pc() + pc_offset, memory), register.get(r0)?, memory);
    Ok(())
}

pub fn op_str(instr:u16,register:&mut Registers,memory:&mut Vec<u16>) ->Result<(),io::Error> {
    let r0 = (instr >> 9) & 0x7;
    let r1 = (instr >> 6) & 0x7;
    let offset = sign_extend(instr & 0x3F, 6);
    let _ = mem_write(register.get(r1)? + offset, register.get(r0)?, memory);
    Ok(())
}
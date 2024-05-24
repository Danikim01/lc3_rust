use std::io;

use crate::Registers;
use crate::flags::ConditionFlags;
use crate::memory::*;

#[derive(Debug)]
pub enum Opcode{
    BR = 0, // branch
    ADD = 1,    // add
    LD = 2,     // load
    ST = 3,     // store
    JSR = 4,    // jump register
    AND = 5,    // bitwise and
    LDR = 6,    // load register
    STR = 7,    // store register
    RTI = 8,    // unused
    NOT = 9,    // bitwise not
    LDI = 10,    // load indirect
    STI = 11,    // store indirect
    JMP = 12,    // jump
    RES = 13,    // reserved (unused)
    LEA = 14,    // load effective address
    TRAP = 15,   // execute trap
}

pub fn sign_extend(mut x: u16, bit_count: u16) -> u16 {
    if (x >> (bit_count - 1)) & 1 != 0 {
        x |= 0xFFFF << bit_count;
    } 
    x
}


fn update_flags(r: u16, register: &mut Registers) ->Result<(),io::Error> {
    if register.get(r)? == 0 {
        let _ = register.update_cond(ConditionFlags::ZRO as u16);
    } else if (register.get(r)? >> 15) != 0 {
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
        let val = register.get(r1)? as u32 + imm5 as u32;
        let _ = register.update(r0, val as u16);
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
        let val = register.get_pc() as u32 + pc_offset as u32;
        let _ = register.update_pc(val as u16);
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
        let val = register.get_pc() as u32 + long_pc_offset as u32;
        let _ = register.update_pc(val as u16);
    } else {
        let _ = register.update_pc(register.get(r1)?);
    }

    Ok(())
}

pub fn op_ld(instr:u16,register:&mut Registers,memory:&mut Vec<u16>) ->Result<(),io::Error> {
    let r0 = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1FF, 9);
    let mem = register.get_pc() as u32 + pc_offset as u32;
    let value = mem_read(mem as u16, memory);
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
    let val = register.get(r1)? as u32 + offset as u32;
    let mem_value = mem_read(val as u16, memory);
    let _ = register.update(r0, mem_value);
    update_flags(r0, register)?;
    Ok(())
}

pub fn op_lea(instr:u16,register:&mut Registers) ->Result<(),io::Error> {
    let r0 = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1FF, 9);
    let val = register.get_pc() as u32 + pc_offset as u32;
    let _ = register.update(r0, val as u16);
    update_flags(r0, register)?;
    Ok(())
}

pub fn op_st(instr:u16,register:&mut Registers,memory:&mut Vec<u16>) ->Result<(),io::Error> {
    let r0 = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1FF, 9);
    let val = register.get_pc() as u32 + pc_offset as u32;
    let _ = mem_write(val as u16, register.get(r0)?, memory);
    Ok(())
}

pub fn op_sti(instr:u16,register:&mut Registers,memory:&mut Vec<u16>) ->Result<(),io::Error> {
    let r0 = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1FF, 9);
    let val = register.get_pc() as u32 + pc_offset as u32;
    let _ = mem_write(mem_read(val as u16, memory), register.get(r0)?, memory);
    Ok(())
}

pub fn op_str(instr:u16,register:&mut Registers,memory:&mut Vec<u16>) ->Result<(),io::Error> {
    let r0 = (instr >> 9) & 0x7;
    let r1 = (instr >> 6) & 0x7;
    let offset = sign_extend(instr & 0x3F, 6);
    let address = register.get(r1)? as u32 + offset as u32;
    let address = address as u16;
    let _ = mem_write(address, register.get(r0)?, memory);
    Ok(())
}
use std::error::Error;
use std::io::{self, BufReader};

use std::fs::File;
use std::io::Read;
use std::mem;
use std::path::Path;
use std::sync::TryLockResult;
use byteorder::{BigEndian, ReadBytesExt};

use crate::MEMORY_SIZE;


//memory mapped registers
pub enum MemoryMappedRegisters{
    KBSR = 0xFE00, // keyboard status
    KBDR = 0xFE02, // keyboard data
}


pub fn read_image(image: &str, memory: &mut Vec<u16>) -> Result<bool, std::io::Error>{
    let path = Path::new(image);
    let mut file = File::open(&path)?;
    let mut origin_buf = [0u8; 2];
    file.read_exact(&mut origin_buf)?;
    let origin = u16::from_be_bytes(origin_buf);
    let max_read = MEMORY_SIZE - origin as usize;
    let p = &mut memory[origin as usize..origin as usize + max_read];
    let mut read_buf = vec![0u8; max_read * 2];
    let read_bytes = file.read(&mut read_buf)?;
    for i in 0..read_bytes / 2 {
        p[i] = u16::from_be_bytes([read_buf[2 * i], read_buf[2 * i + 1]]);
    }
    return Ok(true);    
}




pub fn mem_read(address:u16,memory: &mut Vec<u16>) -> u16{
    if address == MemoryMappedRegisters::KBSR as u16{
        let mut buffer = [0u8; 1];
        std::io::stdin().read_exact(&mut buffer).unwrap();
        //if a key was pressed
        if buffer[0] != 0 {
            memory[MemoryMappedRegisters::KBSR as usize] = 1 << 15;
            memory[MemoryMappedRegisters::KBDR as usize] = buffer[0] as u16;
        }else{
            memory[MemoryMappedRegisters::KBSR as usize] = 0;
        
        }
    }
    return memory[address as usize];
}

pub fn mem_write(address:u16,value:u16,memory: &mut Vec<u16>){
    memory[address as usize] = value;
}
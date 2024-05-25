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

fn swap16(x: u16) -> u16 {
    (x << 8) | (x >> 8)
}

pub fn read_image(image: &str, memory: &mut Vec<u16>) -> Result<bool, std::io::Error>{
    let path = Path::new(image);
    let file = File::open(&path)?;
    let mut reader = BufReader::new(file);
    let mut address = reader.read_u16::<BigEndian>()? as usize;
    loop {
        match reader.read_u16::<BigEndian>() {
            Ok(instruction) => {
                memory[address] = instruction;
                address += 1;
            }
            Err(_e) => {
                break;
            }
        }

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
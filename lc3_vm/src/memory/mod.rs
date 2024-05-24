use std::io::BufReader;

use std::fs::File;
use std::io::Read;
use std::path::Path;


//memory mapped registers
pub enum MemoryMappedRegisters{
    KBSR = 0xFE00, // keyboard status
    KBDR = 0xFE02, // keyboard data
}

fn swap16(x: u16) -> u16 {
    (x << 8) | (x >> 8)
}

pub fn read_image(image: &str, memory: &mut Vec<u16>) -> bool {
    let path = Path::new(image);
    let file = match File::open(&path) {
        Err(why) => {
            println!("Failed to open file: {}", why);
            return false;
        },
        Ok(file) => file,
    };

    let mut reader = BufReader::new(file);
    let mut buffer = [0u8; 2];

    let mut address = 0;
    while let Ok(n) = reader.read(&mut buffer) {
        if n == 0 {
            break;
        }
        let value = u16::from_le_bytes(buffer);
        memory[address] = swap16(value);
        address += 1;
    }

    true
}


pub fn mem_read(address:u16,memory: &mut Vec<u16>) -> u16{
    if address == MemoryMappedRegisters::KBDR as u16{
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
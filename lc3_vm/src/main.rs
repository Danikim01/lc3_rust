extern crate termios;

use termios::*;
use std::io;
use std::result::Result;
use std::io::BufReader;

use std::fs::File;
use std::io::{Read,Write};
use std::path::Path;
use std::env;
use std::process;

mod registers;
mod opcodes;
mod trapcodes;
mod flags;

use crate::opcodes::*;

use registers::Registers;
use trapcodes::Trapcodes;

pub const MEMORY_SIZE: usize = 1 << 16;
pub const PC_START: u16 = 0x3000; //default starting address for PC


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


fn mem_read(address:u16,memory: &mut Vec<u16>) -> u16{
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


fn main() {
    let mut memory = vec![0u16; 65536]; //0u16 stands for 0 of type u16


    //Load arguments
    let args: Vec<String> = env::args().collect();

    if args.len() < 2  {
        println!("Error: provide atleast one VM image");
        println!("Usage: rust-vm <image-file1> [image-file2]..");
        process::exit(2);
    }

    for i in 1..args.len() {
        if !read_image(&args[i], &mut memory)  {
            println!("Failed to load image: {}", args[i]);
            process::exit(1);
        }
    }

    //Setup
    
    let stdin = 0;
    let termios = termios::Termios::from_fd(stdin).unwrap();

    let mut new_termios = termios.clone();
    new_termios.c_iflag &= IGNBRK | BRKINT | PARMRK | ISTRIP | INLCR | IGNCR | ICRNL | IXON;
    new_termios.c_lflag &= !(ICANON | ECHO); // no echo and canonical mode

    tcsetattr(stdin, TCSANOW, &mut new_termios).unwrap();
    

    let mut registers = Registers::new();
    
    let running = true;

    while running{
        let instr:u16 = mem_read(registers.r_pc, &mut memory);
        let op = instr >> 12;

        match op {
            op if op == Opcode::ADD as u16 =>{
                op_add(instr, &mut registers);
            }
            _ => {
                println!("Unkown opcode: {}", op);
                break;
            }
        }
    }

    // reset the stdin to
    // original termios data
    tcsetattr(stdin, TCSANOW, &termios).unwrap();


}

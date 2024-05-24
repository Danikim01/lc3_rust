pub mod trapcodes{

    use crate::memory::mem_read;
    use crate::registers::Registers;
    use std::io::{self, Read,Write};
    use std::process;
    
    pub const TRAP_GETC: u16 = 0x20; // get character from keyboard, not echoed onto the terminal
    pub const TRAP_OUT: u16 = 0x21; // output a character
    pub const TRAP_PUTS: u16 = 0x22; // output a word string
    pub const TRAP_IN: u16 = 0x23; // get character from keyboard, echoed onto the terminal
    pub const TRAP_PUTSP: u16 = 0x24; // output a byte string
    pub const TRAP_HALT: u16 = 0x25; // halt the program

    pub fn execute_trapcodes(trap_vector: u16,register:&mut Registers,memory:&mut Vec<u16>,running:&mut bool){
        let _ = register.update(7, register.get_pc());

        let instr = trap_vector & 0xFF;

        match instr {
            TRAP_GETC => {
                let mut buffer = [0u8; 1];
                std::io::stdin().read_exact(&mut buffer);
                let value = buffer[0] as u16;
                register.update(0, value);
            },
            TRAP_OUT => {
                let value = register.get(0).unwrap();
                //putc((char)reg[R_R0], stdout);
                print!("{}", value as u8 as char);
                //flush to stdout
                io::stdout().flush();
            },
            TRAP_PUTS => {
                let mut address = register.get(0).unwrap();
                let mut value = mem_read(address, memory);
                while value != 0 {
                    print!("{}", value as u8 as char);
                    address += 1;
                    value = mem_read(address, memory);
                }
            },
            TRAP_IN => {
                print!("Enter a character: ");
                let mut buffer = [0u8; 1];
                std::io::stdin().read_exact(&mut buffer).unwrap();
                let value = buffer[0] as u16;
                println!("{}", value as u8 as char);
                io::stdout().flush();
                register.update(0, value);
            },
            TRAP_PUTSP => {
                let mut address = register.get(0).unwrap();
                let mut value = mem_read(address, memory);
                while value != 0x000 {
                    let c1 = (value & 0xFF) as u8;
                    print!("{}", c1 as char);
                    let c2 = (value >> 8) as u8;
                    if c2 != 0 {
                        print!("{}", c2 as char);
                    }
                    address += 1;
                    value = mem_read(address, memory);
                }
                io::stdout().flush().unwrap();
            },
            TRAP_HALT => {
                println!("HALT");
                io::stdout().flush().unwrap();
                *running = false;
            },
            _ => {
                process::exit(1);
            }
        }       
    }

}
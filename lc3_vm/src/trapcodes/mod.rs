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
        //let _ = register.update(7, register.get_pc());

        let instr = trap_vector & 0xFF;

        match instr {
            TRAP_GETC => {
                println!("Entra al getc");
                let mut buffer = [0u8; 1];
                std::io::stdin().read_exact(&mut buffer);
                let value = buffer[0] as u16;
                register.update(0, value);
            },
            TRAP_OUT => {
                println!("Entra al out");
                let value = register.get(0).unwrap();
                //putc((char)reg[R_R0], stdout);
                print!("{}", value as u8 as char);
                //flush to stdout
                io::stdout().flush();
            },
            TRAP_PUTS => {
                println!("Entra al puts");
                let mut address = register.get(0).unwrap();
                let mut value = mem_read(address, memory);
                while value != 0 {
                    print!("{}", value as u8 as char);
                    address += 1;
                    value = mem_read(address, memory);
                }
                io::stdout().flush().expect("failed to flush");
            },
            TRAP_IN => {
                print!("Enter a character: ");
                io::stdout().flush().expect("failed to flush");
                let val = io::stdin().bytes().next().unwrap().unwrap() as u16;
                register.update(0, val);
            },
            TRAP_PUTSP => {
                println!("Entra al putsp");
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


#[cfg(test)]
    mod tests {
        use super::*;
        use crate::registers::Registers;
        use std::io::{self, Read, Write};
        use crate::execute_trapcodes;
        use crate::trapcodes::trapcodes::*;

        #[test]
        fn test_trap_getc() {
            let mut registers = Registers::new();
            let mut memory = vec![0; 65536];
            let mut running = true;

            // Simulate user input
            let input: u8 = b'A';
            let mut stdin = io::stdin();
            stdin.lock().read_exact(&mut [input]).unwrap();

            // Execute trap GETC
            execute_trapcodes(TRAP_GETC, &mut registers, &mut memory, &mut running);

            // Check if register R0 contains the correct value
            assert_eq!(registers.get(0).unwrap(), input as u16);
        }

        // #[test]
        // fn test_trap_out() {
        //     let mut registers = Registers::new();
        //     let mut memory = vec![0; 65536];
        //     let mut running = true;

        //     // Set register R0 to a character value
        //     registers.update(0, b'H' as u16);

        //     // Capture the output
        //     let mut output = Vec::new();
        //     io::stdout().flush().unwrap();
        //     io::stdout().read_to_end(&mut output).unwrap();

        //     // Execute trap OUT
        //     execute_trapcodes(TRAP_OUT, &mut registers, &mut memory, &mut running);

        //     // Check if the output contains the correct character
        //     assert_eq!(output, vec![b'H']);
        // }

        // #[test]
        // fn test_trap_puts() {
        //     let mut registers = Registers::new();
        //     let mut memory = vec![0; 65536];
        //     let mut running = true;

        //     // Set register R0 to the address of a null-terminated string
        //     let string = "Hello, World!\0";
        //     let address = memory.len() as u16;
        //     memory.extend(string.bytes().map(|b| b as u16));

        //     registers.update(0, address);

        //     // Capture the output
        //     let mut output = Vec::new();
        //     io::stdout().flush().unwrap();
        //     io::stdout().read_to_end(&mut output).unwrap();

        //     // Execute trap PUTS
        //     execute_trapcodes(TRAP_PUTS, &mut registers, &mut memory, &mut running);

        //     // Check if the output contains the correct string
        //     assert_eq!(output, string.as_bytes());
        // }

        // #[test]
        // fn test_trap_in() {
        //     let mut registers = Registers::new();
        //     let mut memory = vec![0; 65536];
        //     let mut running = true;

        //     // Simulate user input
        //     let input: u8 = b'A';
        //     let mut stdin = io::stdin();
        //     stdin.lock().read_exact(&mut [input]).unwrap();

        //     // Execute trap IN
        //     execute_trapcodes(TRAP_IN, &mut registers, &mut memory, &mut running);

        //     // Check if register R0 contains the correct value
        //     assert_eq!(registers.get(0), Some(input as u16));
        // }

        // #[test]
        // fn test_trap_putsp() {
        //     let mut registers = Registers::new();
        //     let mut memory = vec![0; 65536];
        //     let mut running = true;

        //     // Set register R0 to the address of a null-terminated string
        //     let string = "Hello, World!\0";
        //     let address = memory.len() as u16;
        //     memory.extend(string.bytes().map(|b| b as u16));

        //     registers.update(0, address);

        //     // Capture the output
        //     let mut output = Vec::new();
        //     io::stdout().flush().unwrap();
        //     io::stdout().read_to_end(&mut output).unwrap();

        //     // Execute trap PUTSP
        //     execute_trapcodes(TRAP_PUTSP, &mut registers, &mut memory, &mut running);

        //     // Check if the output contains the correct string
        //     assert_eq!(output, string.as_bytes());
        // }

    }
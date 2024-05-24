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

use signal_hook::{iterator::Signals};
//import SIGNINT from signal_hook
use signal_hook::consts::signal::*;


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


fn disable_input_buffering() -> Result<(),io::Error>{
    let stdin = 0;
    let termios = termios::Termios::from_fd(stdin)?;
    let mut new_termios = termios.clone();
    new_termios.c_lflag &= !(ICANON | ECHO); // no echo and canonical mode
    tcsetattr(stdin, TCSANOW, &mut new_termios)?;
    Ok(())
}

fn restore_input_buffering() -> Result<(),io::Error>{
    let stdin = 0;
    let termios = termios::Termios::from_fd(stdin)?;
    let mut new_termios = termios.clone();
    new_termios.c_lflag |= ICANON | ECHO; // echo and canonical mode
    tcsetattr(stdin, TCSANOW, &mut new_termios)?;
    Ok(())
}


fn handle_control_c(sig:i32) -> Result<(),io::Error> {
    restore_input_buffering()?;
    println!("\n\n");
    println!("The LC3 VM received Ctrl-C interrupt signal (= {}).", sig);
    println!("So, exiting the process with exit code = 128 + 2 = 130.\n");
    std::process::exit(130);
}

/// When the program is interrupted (with pressing `Control-C` keys), we want to restore the terminal settings back to normal.
/// `spawn_control_c_handler` function will spawn a thread to handle `Control-C` (interrupt signal).
/// When user presses `Control-C` keys, it restores terminal to its original, i.e. it turns value of `ICANON` and `ECHO` modes to 1.
/// And exists the process with process code = 130 (as mentioned here, http://tldp.org/LDP/abs/html/exitcodes.html).
pub fn spawn_control_c_handler() -> Result<(), Box<dyn std::error::Error>> {
    //setup for interrupt handling.
    let mut signals = Signals::new(&[SIGINT])?;
    std::thread::spawn(move || {
        for sig in signals.forever() {
            //Interrupt (Ctrl + C) is handled as follows...
            //Terminal is restored to its original configuration
            //Process is exited with (130)
            let _ = handle_control_c(sig);
        }
    });
    Ok(())
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
    spawn_control_c_handler().unwrap();
    disable_input_buffering().unwrap();
   

    let mut registers = Registers::new();
    
    //Main Loop
    let running = true;

    while running{
        let instr:u16 = mem_read(registers.r_pc, &mut memory);
        let op = instr >> 12;

        match op {
            op if op == Opcode::ADD as u16 =>{
                op_add(instr, &mut registers);
            },
            op if op == Opcode::AND as u16 =>{
                op_and(instr, &mut registers);
            },
            op if op == Opcode::NOT as u16 =>{
                op_not(instr, &mut registers);
            },
            op if op == Opcode::BR as u16 =>{
                op_br(instr, &mut registers);
            },
            op if op == Opcode::JMP as u16 =>{
                op_jmp(instr, &mut registers);
            },
            op if op == Opcode::JSR as u16 =>{
                op_jsr(instr, &mut registers);
            },
            op if op == Opcode::LD as u16 =>{
                op_ld(instr, &mut registers, &mut memory);
            },
            op if op == Opcode::LDI as u16 =>{
                op_ldi(instr, &mut registers, &mut memory);
            },
            op if op == Opcode::LDR as u16 =>{
                op_ldr(instr, &mut registers, &mut memory);
            },
            op if op == Opcode::LEA as u16 =>{
                op_lea(instr, &mut registers);
            },
            op if op == Opcode::ST as u16 =>{
                op_st(instr, &mut registers, &mut memory);
            },
            op if op == Opcode::STI as u16 =>{
                op_sti(instr, &mut registers, &mut memory);
            },
            op if op == Opcode::STR as u16 =>{
                op_str(instr, &mut registers, &mut memory);
            },
            op if op == Opcode::TRAP as u16 =>{
                op_trap(instr, &mut registers);
            },
            op if op == Opcode::RTI as u16 =>{
                println!("RTI is not implemented yet.");
                break;
            },
            op if op == Opcode::RES as u16 =>{
                println!("RES is not implemented yet.");
                break;
            },
            _ => {
                println!("Unkown opcode: {}", op);
                break;
            }
        }

        //reg[R_PC]++
        registers.r_pc += 1;
    }

    restore_input_buffering().unwrap();


}

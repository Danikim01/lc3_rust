extern crate termios;

use termios::*;
use std::io;
use std::mem;
use std::result::Result;

use std::env;
use std::process;

use signal_hook::{iterator::Signals};
//import SIGNINT from signal_hook
use signal_hook::consts::signal::*;


mod registers;
mod opcodes;
mod trapcodes;
mod flags;
mod memory;

use crate::opcodes::*;

use registers::Registers;
use crate::trapcodes::trapcodes::*;
use memory::*;

const MEMORY_SIZE: usize = u16::MAX as usize;
pub const PC_START: u16 = 0x3000; //default starting address for PC


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
    std::process::exit(130);
}


pub fn spawn_control_c_handler() -> Result<(), Box<dyn std::error::Error>> {
    //setup for interrupt handling.
    let mut signals = Signals::new(&[SIGINT])?;
    std::thread::spawn(move || {
        for sig in signals.forever() {
            //Interrupt (Ctrl + C) is handled as follows...
            //Terminal is restored to its original configuration
            //Process is exited with (130)
            println!("Interrupted by Ctrl + C");
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
        match read_image(&args[i], &mut memory) {
            Ok(true) => {
                println!("Loaded image: {}", args[i]);
            },
            Ok(false) => {
                println!("Failed to load image: {}", args[i]);
                process::exit(1);
            },
            Err(e) => {
                println!("Error: {}", e);
                process::exit(1);
            }
        }
    }

    
    //Setup
    spawn_control_c_handler().unwrap();
    disable_input_buffering().unwrap();


    let mut registers = Registers::new();

    //Main Loop
    let mut running = true;

    while registers.get_pc() < MEMORY_SIZE as u16 && running{
        let instr:u16 = mem_read(registers.r_pc, &mut memory);
        let op = instr >> 12;
        registers.update_pc(registers.get_pc() + 1);
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
                execute_trapcodes(instr, &mut registers, &mut memory,&mut running);
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
    }

    restore_input_buffering().unwrap();


}

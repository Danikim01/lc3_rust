use std::io;


pub const PC_START: u16 = 0x3000;

#[derive(Debug)]
pub struct Registers {
    /// `r_00` is a general purpose register.
    pub r_00: u16, // general purpose register
    /// `r_01` is a general purpose register.
    pub r_01: u16, // general purpose register
    /// `r_02` is a general purpose register.
    pub r_02: u16, // general purpose register
    /// `r_03` is a general purpose register.
    pub r_03: u16, // general purpose register
    /// `r_04` is a general purpose register.
    pub r_04: u16, // general purpose register
    /// `r_05` is a general purpose register.
    pub r_05: u16, // general purpose register
    /// `r_06` is a general purpose register.
    pub r_06: u16, // general purpose register
    /// `r_07` is a general purpose register.
    pub r_07: u16, // general purpose register
    /// `r_pc` is a register for program counter.
    pub r_pc: u16, // program counter
    /// `r_cond` is a register to store cinformation about the previous calculation.
    pub r_cond: u16, // condition flag
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            r_00: 0,
            r_01: 0,
            r_02: 0,
            r_03: 0,
            r_04: 0,
            r_05: 0,
            r_06: 0,
            r_07: 0,
            r_pc: PC_START,
            r_cond: 0,
        }
    }

    pub fn update(&mut self, index: u16, value: u16) -> Result<(),io::Error> {
        match index {
            0 => self.r_00 = value,
            1 => self.r_01 = value,
            2 => self.r_02 = value,
            3 => self.r_03 = value,
            4 => self.r_04 = value,
            5 => self.r_05 = value,
            6 => self.r_06 = value,
            7 => self.r_07 = value,
            8 => self.r_pc = value,
            9 => self.r_cond = value,
            _ => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Index out of bound."))
        }
        Ok(())
    }

    pub fn update_cond(&mut self, value: u16) {
        self.r_cond = value;
    }

    pub fn get(&self, index: u16) -> Result<u16,io::Error>{
        match index {
            0 => Ok(self.r_00),
            1 => Ok(self.r_01),
            2 => Ok(self.r_02),
            3 => Ok(self.r_03),
            4 => Ok(self.r_04),
            5 => Ok(self.r_05),
            6 => Ok(self.r_06),
            7 => Ok(self.r_07),
            8 => Ok(self.r_pc),
            9 => Ok(self.r_cond),
            _ => Err(io::Error::new(io::ErrorKind::InvalidInput, "Index out of bound."))
        }
    }

}
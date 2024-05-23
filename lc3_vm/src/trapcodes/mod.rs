pub mod Trapcodes{
    pub const TRAP_GETC: u16 = 0x20; // get character from keyboard, not echoed onto the terminal
    pub const TRAP_OUT: u16 = 0x21; // output a character
    pub const TRAP_PUTS: u16 = 0x22; // output a word string
    pub const TRAP_IN: u16 = 0x23; // get character from keyboard, echoed onto the terminal
    pub const TRAP_PUTSP: u16 = 0x24; // output a byte string
    pub const TRAP_HALT: u16 = 0x25; // halt the program
}
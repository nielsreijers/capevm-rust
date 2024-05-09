use core::ptr::{addr_of_mut, write_volatile};

const AVRORA_PRINT_2BYTE_HEXADECIMALS: u8        = 0x1;
const AVRORA_PRINT_2BYTE_UNSIGNED_INTEGERS: u8   = 0x3;
const AVRORA_PRINT_2BYTE_SIGNED_INTEGERS: u8     = 0x8;
const AVRORA_PRINT_4BYTE_HEXADECIMALS: u8        = 0x4;
const AVRORA_PRINT_4BYTE_UNSIGNED_INTEGERS: u8   = 0x5;
const AVRORA_PRINT_4BYTE_SIGNED_INTEGERS: u8     = 0x9;
const AVRORA_PRINT_R1: u8                        = 0xC;
const AVRORA_PRINT_SP: u8                        = 0xD;
const AVRORA_PRINT_REGS: u8                      = 0xE;
const AVRORA_PRINT_PC: u8                        = 0x12;

#[allow(non_upper_case_globals)]
#[no_mangle]
static mut debugbuf1: [u8; 5] = [0, 0, 0, 0, 0];

fn signal_avrora_c_print(instruction: u8) {
    unsafe {
        write_volatile(addr_of_mut!(debugbuf1[0]), instruction);
    }
}

fn signal_avrora_c_print_16(instruction: u8, payload: u16) {
    unsafe {
        write_volatile(addr_of_mut!(debugbuf1[1]), payload as u8);
        write_volatile(addr_of_mut!(debugbuf1[2]), (payload >> 8) as u8); 
        write_volatile(addr_of_mut!(debugbuf1[0]), instruction);
    }
}

fn signal_avrora_c_print_32(instruction: u8, payload: u32) {
    unsafe {
        write_volatile(addr_of_mut!(debugbuf1[1]), payload as u8);
        write_volatile(addr_of_mut!(debugbuf1[2]), (payload >> 8) as u8); 
        write_volatile(addr_of_mut!(debugbuf1[3]), (payload >> 16) as u8); 
        write_volatile(addr_of_mut!(debugbuf1[4]), (payload >> 24) as u8); 
        write_volatile(addr_of_mut!(debugbuf1[0]), instruction);
    }
}

/// Uses Avrora's c-print monitor to print a 16 bit unsigned int as hex
#[allow(dead_code)]
pub fn print_u16_hex(val: u16) {
    signal_avrora_c_print_16(AVRORA_PRINT_2BYTE_HEXADECIMALS, val);
}

/// Uses Avrora's c-print monitor to print a 16 bit unsigned int
#[allow(dead_code)]
pub fn print_u16(val: u16) {
    signal_avrora_c_print_16(AVRORA_PRINT_2BYTE_UNSIGNED_INTEGERS, val);
}

/// Uses Avrora's c-print monitor to print a 16 bit signed int
#[allow(dead_code)]
pub fn print_i16(val: i16) {
    signal_avrora_c_print_16(AVRORA_PRINT_2BYTE_SIGNED_INTEGERS, val as u16);
}

/// Uses Avrora's c-print monitor to print a 32 bit unsigned int as hex
#[allow(dead_code)]
pub fn print_u32_hex(val: u32) {
    signal_avrora_c_print_32(AVRORA_PRINT_4BYTE_HEXADECIMALS, val);
}

/// Uses Avrora's c-print monitor to print a 32 bit unsigned int
#[allow(dead_code)]
pub fn print_u32(val: u32) {
    signal_avrora_c_print_32(AVRORA_PRINT_4BYTE_UNSIGNED_INTEGERS, val);
}

/// Uses Avrora's c-print monitor to print a 32 bit signed int
#[allow(dead_code)]
pub fn print_i32(val: i32) {
    signal_avrora_c_print_32(AVRORA_PRINT_4BYTE_SIGNED_INTEGERS, val as u32);
}

/// Uses Avrora's c-print monitor to print the contents of the R1 register
#[allow(dead_code)]
pub fn print_r1() {
    signal_avrora_c_print(AVRORA_PRINT_R1);
}

/// Uses Avrora's c-print monitor to print the contents of the SP register
#[allow(dead_code)]
pub fn print_sp() {
    signal_avrora_c_print(AVRORA_PRINT_SP);
}

/// Uses Avrora's c-print monitor to print the contents of the PC register
#[allow(dead_code)]
pub fn print_pc() {
    signal_avrora_c_print(AVRORA_PRINT_PC);
}

/// Uses Avrora's c-print monitor to print the contents of the registers R0 to R31
#[allow(dead_code)]
pub fn print_all_regs() {
    signal_avrora_c_print(AVRORA_PRINT_REGS);
}
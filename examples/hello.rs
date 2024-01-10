//! Prints "Hello, world!" on the host console using semihosting
#![allow(unused_imports)]
#![no_main]
#![no_std]

use panic_halt as _;

use core::arch::asm;
use cortex_m_rt::entry;
use volatile_register::RW;
use cortex_m_semihosting::{debug, hprintln}; 

#[no_mangle]
pub extern "C" fn main() -> ! {
   

    //hprintln!("Hello, world!").unwrap();
   
    let mut addr1 = 0x20001020;
    let mut addr2 = 0x20001022;
    let mut addr3 = 0x20001024;


    unsafe {
        asm!("mov r1, #8
            str r1, [{0}]",
            inlateout(reg) addr1);
    }
    unsafe {
        asm!("mov r2, #16
            str r2, [{0}]",
            inlateout(reg) addr2);
    }
    unsafe {
        asm!("mov r3, #32
            str r3, [{0}]",
            inlateout(reg) addr3);
    }
    hprintln!("hello").unwrap();

    // exit QEMU
    // NOTE do not run this on hardware; it can corrupt OpenOCD state
    debug::exit(debug::EXIT_SUCCESS);

    loop {}
}

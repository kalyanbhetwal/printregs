//! Prints "Hello, world!" on the host console using semihosting
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(dead_code)]
#![no_main]
#![no_std]

use panic_halt as _;
use core::arch::asm;
use cortex_m_rt::entry;
use volatile_register::RW;
use core::mem::MaybeUninit;
use cortex_m_semihosting::{debug, hprintln}; 

const VALUE1: u32 = 5;
const VALUE2: u32 = 3;

// // Define the stack size
const STACK_SIZE: usize = 1024; // Adjust the size based on your requirements

// fn rest(){
//     unsafe{asm!("NOP")}
// }


#[no_mangle]
pub extern "C" fn main() -> ! {
    let r0_value: u32;

    let mut uninitialized_array: [MaybeUninit<u32>; 10];
    unsafe {
        asm!(
            "MOV {0}, r0",
            out(reg) r0_value
        );
    }


    // exit QEMU
    // NOTE do not run this on hardware; it can corrupt OpenOCD state
    debug::exit(debug::EXIT_SUCCESS);

    loop {}
}

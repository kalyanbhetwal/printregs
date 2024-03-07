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
use cortex_m_semihosting::{debug, hprintln}; 

#[link_section = ".custom_section"]
static mut MY_VARIABLE_1: u32 = 2;
static mut MY_VARIABLE_2: u32 = 1;
static mut MY_VARIABLE_3: u32 = 5;

#[no_mangle]
pub extern "C" fn main() -> ! {
      // Access the address of the global variable
      let a = 10;
      let b = 20;
      let address = unsafe { &MY_VARIABLE_1 as *const u32 };
      let address_2 = unsafe { &MY_VARIABLE_2 as *const u32 };
      let address_3 = unsafe { &MY_VARIABLE_3 as *const u32 };

      // Print the address
      hprintln!("Address of MY_GLOBAL_VAR: {:p}", address).unwrap();
      hprintln!("Address of MY_GLOBAL_VAR2: {:p}", address_2).unwrap();
      hprintln!("Address of MY_GLOBAL_VAR3: {:p}", address_3).unwrap();
   //hprintln!("hello world!").unwrap();
    // exit QEMU
    // NOTE do not run this on hardware; it can corrupt OpenOCD state
    debug::exit(debug::EXIT_SUCCESS);

    loop {}
}

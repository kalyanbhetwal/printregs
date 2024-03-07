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

static  mut ARR:[i32;10] =[2050; 10];
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
      let arr_ptr: *const i32 = unsafe{ARR.as_ptr()};
        
    // Print the address of ARR
      hprintln!("Address of ARR: {:?}", arr_ptr).unwrap();
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

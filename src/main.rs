#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

//use cortex_m::asm;
use core::arch::asm;
use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln}; 

#[entry]
fn main() -> ! {
  //let r0: u32;
    // let  operand1 = 10;
    // let  operand2 = 20;
    // let mut result = 0;
    // result = operand1 + operand2;
    // hprintln!("Result: {}", result).unwrap();

    // unsafe {
    //     asm!(
    //         "mov {0}, r1",
    //         out(reg) r0,
    //         options(nostack)
    //     );
    // }

    unsafe {
        asm!(
            // Store the value of r0 in RAM at the specified address without modifying any other register
           "NOP",
            "MOV r1, {0}",
            in(reg) 1235,  // Replace with your desired RAM address
            options(nomem, nostack, preserves_flags),
            
        );
    }

    unsafe {
        asm!(
            "NOP",
            // Store the value of r0 in RAM at the specified address without modifying any other register
            "MOV r2, {0}",
            in(reg) 1236,  // Replace with your desired RAM address
            options(nomem, nostack, preserves_flags),
            
        );
    }
    // unsafe {
    //     asm!(
    //         // Store the value of r0 in RAM at the specified address without modifying any other register
    //         "MOV r0, {0}",
    //         in(reg) 0x20001020,  // Replace with your desired RAM address       
    //     );
    // }

    // unsafe {
    //     asm!(
    //         "NOP",
    //         "STR r1, [r0, #0]", // Store the value of r0 in RAM at the specified address
    //     );
    // }


    unsafe {
        asm!(
            // Store the value of r0 in RAM at the specified address without modifying any other register
            "NOP",
            "STR r1, [{0}]",
            in(reg) 0x20001020  // Replace with your desired RAM address
        );
    }


    unsafe {
        asm!(
            // Store the value of r0 in RAM at the specified address without modifying any other register
           
            "NOP",
            "STR r2, [{0}]",
            in(reg) 0x20001022  // Replace with your desired RAM address
        );
    }


    hprintln!("Hello, world!").unwrap();

    // exit QEMU
    // NOTE do not run this on hardware; it can corrupt OpenOCD state
    debug::exit(debug::EXIT_SUCCESS);

    loop {}
}

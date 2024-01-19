//! Prints "Hello, world!" on the host console using semihosting
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![no_main]
#![no_std]

use panic_halt as _;
use core::arch::asm;
use cortex_m_rt::entry;
use volatile_register::RW;
use cortex_m_semihosting::{debug, hprintln}; 

// // Define the stack size
const STACK_SIZE: usize = 1024; // Adjust the size based on your requirements

// // Define the stack and specify its section
#[link_section = ".stack"]
static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

// fn rest(){
//     unsafe{asm!("NOP")}
// }

fn checkpoint() {    

    unsafe {
        asm!(
            "add sp, #80"
        );
    }
    unsafe {
        asm!(
            "pop {{r7}}"
        );
    }
    unsafe {
        asm!(
            "push {{r7}}"
        );
    }
    unsafe {
        asm!(
            "sub sp, #80"
        );
    }
   
   let r0_value: u32;
    let r1_value: u32;
    let r2_value: u32;
    let r3_value: u32;
    let r4_value: u32;
    let r5_value: u32;
    let r6_value: u32;
    let r7_value: u32;
    let r8_value: u32;
    let r9_value: u32;
    let r10_value: u32;
    let r11_value: u32;
    let r12_value: u32;
    let r13_sp: u32;
    let r14_lr: u32;
    let r15_pc: u32;

    // let (
    //     r0_value, r1_value, r2_value, r3_value, r4_value, r5_value, r6_value, r7_value,
    //     r8_value, r9_value, r10_value, r11_value, r12_value, r13_sp, r14_lr, r15_pc,
    // ): (u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32);
    
    unsafe {
        asm!(
            "MOV {0}, r0",
            out(reg) r0_value
        );
    }
    unsafe {
        asm!(
            "MOV {0}, r1",
            out(reg) r1_value
        );
    }
    unsafe {
        asm!(
            "MOV {0}, r2",
            out(reg) r2_value
        );
    }
    unsafe {
        asm!(
            "MOV {0}, r3",
            out(reg) r3_value
        );
    }
    unsafe {
        asm!(
            "MOV {0}, r4",
            out(reg) r4_value
        );
    }
    unsafe {
        asm!(
            "MOV {0}, r5",
            out(reg) r5_value
        );
    }
    unsafe {
        asm!(
            "MOV {0}, r6",
            out(reg) r6_value
        );
    }
    unsafe {
        asm!(
            "MOV {0}, r7",
            out(reg) r7_value
        );
    }
    unsafe {
        asm!(
            "MOV {0}, r8",
            out(reg) r8_value
        );
    }  
    unsafe {
        asm!(
            "MOV {0}, r9",
            out(reg) r9_value
        );
    }   

    unsafe {
        asm!(
            "MOV {0}, r10",
            out(reg) r10_value
        );
    }              
    unsafe {
        asm!(
            "MOV {0}, r11",
            out(reg) r11_value
        );
    }               
    unsafe {
        asm!(
            "MOV {0}, r12",
            out(reg) r12_value
        );
    }     
    // correct the sp for later use  (just used as a place holder)    
    // actual_sp = current_sp + 88
    unsafe {
        asm!(
            "MOV {0}, r13",
            out(reg) r13_sp
        );
    }           
    unsafe {
        asm!(
            "MOV {0}, r14",
            out(reg) r14_lr
        );
    }       
    unsafe {
        asm!(
            "MOV {0}, r15",
            out(reg) r15_pc
        );
    } 

    unsafe{
       //let  start_address: u32 = 0x2000_fffc as u32;
       let mut end_address = r13_sp+88;
    

    while end_address <= 0x2000_fffc as u32 {
        let value = core::ptr::read_volatile(end_address as * const u32);
        //write_to_flash();
        //fn write_to_flash(flash: &mut FLASH, addr: u32, data: u32)
        // Move to the next address based on the size of the type
       // hprintln!("stack val: {:#X}",value).unwrap();
        end_address = end_address+4;
        
    }
}
    // hprintln!("r0: {:#X}", r0_value).unwrap();  
    // hprintln!("r1: {:#X}", r1_value).unwrap();  
    // hprintln!("r2: {:#X}", r2_value).unwrap();  
    // hprintln!("r3: {:#X}", r3_value).unwrap();
    // hprintln!("r4: {:#X}", r4_value).unwrap();
    // hprintln!("r5: {:#X}", r5_value).unwrap();
    // hprintln!("r6: {:#X}", r6_value).unwrap();
   
    // hprintln!("r8: {:#X}", r8_value).unwrap();
    // hprintln!("r9: {:#X}", r9_value).unwrap();
    // hprintln!("r10: {:#X}", r10_value).unwrap();
    // hprintln!("r11: {:#X}", r11_value).unwrap();
    // hprintln!("r12: {:#X}", r12_value).unwrap();
    // hprintln!("r13: {:#X}", r13_sp).unwrap();
    // hprintln!("r14: {:#X}", r14_lr).unwrap();
    // hprintln!("r15: {:#X}", r15_pc).unwrap();          

}

// #[no_mangle]
// pub extern "C" fn change_stack_to_psp()->!{
//     unsafe {
//         asm!(
//             // change PSP
//             "LDR R0, = STACK
//             MSR PSP, R0
//             MRS R1, CONTROL
//             ORR R1, R1, #2
//             MSR CONTROL, R1
//             BX LR",
//             options(noreturn)
//         );
//     }
// }

#[no_mangle]
pub extern "C" fn change_stack_to_psp(){
  
    unsafe{
    let stack_ptr = &STACK as *const _ as u32;
    asm!(
        "MSR MSP, {}", 
        in(reg) stack_ptr
    )
}
}

#[no_mangle]
pub extern "C" fn main() -> ! {
   
    //change_stack_to_psp();

    //hprintln!("Hello, world!").unwrap();
   
    // let mut addr1 = 0x20001020;
    // let mut addr2 = 0x20001022;
    // let mut addr3 = 0x20001024;


    // unsafe {
    //     asm!("mov r1, #8
    //         str r1, [{0}]",
    //         inlateout(reg) addr1);
    // }
    // unsafe {
    //     asm!("mov r2, #16
    //         str r2, [{0}]",
    //         inlateout(reg) addr2);
    // }
    // unsafe {
    //     asm!("mov r3, #32
    //         str r3, [{0}]",
    //         inlateout(reg) addr3);
    // }

    let a = 1;
    let b = 2;
     unsafe {
        asm!("mov r0, #20
                mov r1, #8
                mov r2, #16
                mov r3, #32
                mov r4, #40
                mov r7, #99")
        }
     
     checkpoint();
     let c = a + b;

    // hprintln!("r0: {:#X}", r0_value).unwrap();


    // hprintln!("hello").unwrap();

    // exit QEMU
    // NOTE do not run this on hardware; it can corrupt OpenOCD state
    debug::exit(debug::EXIT_SUCCESS);

    loop {}
}

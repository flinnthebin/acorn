use core::arch::asm;
use crate::start;

#[no_mangle]
pub extern "C" fn _entry() -> ! {
    unsafe {
        asm!(
            // set up a stack frame
            "la sp, stack0",
            // 4096-byte stack size
            "li a0, 1024*4",
            // read mhartid
            "csrr a1, mhartid",
            // increment hartid (zero stack avoidance)
            "addi a1, a1, 1",
            // offset = stacksize * mhartid
            "mul a0, a0, a1",
            // CPU stack pointer = frame + offset
            "add sp, sp, a0",
            // jump to start()
            "call {start}",
            start = sym start::start,
            options(noreturn)
        );
    }
}

#[no_mangle]
pub extern "C" fn spin() -> ! {
    loop {
        unsafe {
            asm!("j spin", options(noreturn));
        }
    }
}


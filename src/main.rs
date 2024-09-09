#![no_std]

mod arch;
mod console;
mod entry;
mod kalloc;
mod proc;
mod sleeplock;
mod spinlock;
mod start;
mod syscall;
mod sysproc;
mod trampoline;
mod trap;
mod uart;
mod vm;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn main() {
    start::init();
}

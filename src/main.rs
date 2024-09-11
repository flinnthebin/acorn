#![no_std]

use core::panic::PanicInfo;

mod arch;
mod console;
mod entry;
mod kalloc;
mod proc;
mod safety;
mod sleeplock;
mod spinlock;
mod start;
mod syscall;
mod sysproc;
mod trampoline;
mod trap;
mod uart;
mod vm;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn main() {
    start::init();
}

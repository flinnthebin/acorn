use core::arch::asm;

// Machine Status Register
// Machine Previous Privilege: MPP[1:0]
// 2-bit field within the Machine Status register indicating previous privilege before a trap
//
// Machine Interrupt Enable
// Controls whether interrupts are globally enabled/disabled to be accepted by CPU

const MSTATUS_MPP_MASK: u64 = 0b11 << 11; // isolates the MPP field
const MSTATUS_MPP_U: u64 = 0b00 << 11; // isolates user-mode value
const MSTATUS_MPP_S: u64 = 0b01 << 11; // isolates supervisor-mode value
const MSTATUS_MPP_M: u64 = 0b11 << 11; // isolates machine-mode value
const MSTATUS_MIE: u64 = 1 << 3; // sets bit 3 of mstatus register. 1 = enabled, 0 = disabled

pub fn read_machine_hartid() -> u64 {
    let hartid: u64;
    unsafe {
        asm!(
            "csrr {0}, 0xf14",
            out(reg) hartid
        );
    }
    hartid
}

fn main() {
    let hartid = read_machine_hartid();
    println!("{}", hartid);
}

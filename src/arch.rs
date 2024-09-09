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

// For RISC-V architecture, Machine Hardware Thread ID (MHARTID) = 0xf14
const CSR_MHARTID: u64 = 0xf14;

pub fn read_mhartid() -> usize {
    let hartid: usize;
    unsafe {
        asm!(
            "csrr {0}, {1}",
            out(reg) hartid,
            CSR_MHARTID,
        );
    }
    hartid
}



use core::arch::asm;

// Machine Status Register (MSTATUS)
// CPU Control and Status Information
// - Machine Previous Privilege (MPP[1:0]): 2-bit field indicating the previous privilege mode (U/S/M) before a trap
// - Machine Interrupt Enable (MIE): Controls whether interrupts are globally enabled (1) or disabled (0).

const MSTATUS_MPP_MASK: usize = 0b11 << 11; // Mask to isolate the MPP field
const MSTATUS_MPP_U: usize = 0b00 << 11; // User-mode value
const MSTATUS_MPP_S: usize = 0b01 << 11; // Supervisor-mode value
const MSTATUS_MPP_M: usize = 0b11 << 11; // Machine-mode value
const MSTATUS_MIE: usize = 1 << 3; // Bit 3 of MSTATUS. 1 = enabled, 0 = disabled

// For RISC-V architecture, Machine Hardware Thread ID (MHARTID) CSR address = 0xf14
const CSR_MHARTID: usize = 0xf14;

// Returns ID of hardware thread
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

// Returns current value of MSTATUS
pub fn read_mstatus() -> usize {
    let status: usize;
    unsafe {
        asm!(
            "csrr {0}, mstatus",
            out(reg) status,
        );
    }
    status
}

// Writes some value to MSTATUS
pub fn write_mstatus(val: usize) {
    unsafe {
        asm!(
            "csrw mstatus, {0}",
            in(reg) val,
        );
    }
}

// Machine Exception Program Counter
// Holds the address of an instruction that caused a machine-level exception
// Address is saved when exception occurs and can be used to resume excecution or handle the exception



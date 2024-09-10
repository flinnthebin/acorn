use core::arch::asm;

// Control and Status Register (CSR)

// Machine Status Register (MSTATUS) (CSR)
// - Machine Previous Privilege (MPP[1:0]): 2-bit field indicating the previous privilege mode (U/S/M) before a trap

const MSTATUS_MPP_MASK: usize = 0b11 << 11; // Mask to isolate the MPP field
const MSTATUS_MPP_U: usize = 0b00 << 11; // User-mode value
const MSTATUS_MPP_S: usize = 0b01 << 11; // Supervisor-mode value
const MSTATUS_MPP_M: usize = 0b11 << 11; // Machine-mode value
const MSTATUS_MIE: usize = 1 << 3; // Machine Interrupt Enable (1 = Enabled, 0 = Disabled)

// For RISC-V architecture, Machine Hardware Thread ID (MHARTID) CSR address = 0xf14
const CSR_MHARTID: usize = 0xf14;

// Returns ID of hardware thread
pub fn read_mhartid() -> usize {
    let hartid: usize;
    unsafe {
        asm!(
            "csrr {0}, {1}",
            out(reg) hartid,
            const CSR_MHARTID,
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
// Address is saved when exception occurs and can be used to resume exceution or handle the exception

// Writes instruction address to MEPC
pub fn write_mepc(addr: usize) {
    unsafe {
        asm!(
            "csrw mepc, {0}",
            in(reg) addr,
        );
    }
}

// Supervisor Status Register (SSTATUS) (CSR)

const SSTATUS_SPP: usize = 0b01 << 8; // Supervisor Previous Privilege (1 = Supervisor, 0 = User)
const SSTATUS_SPIE: usize = 0b01 << 5; // Supervisor Previous Interrupt Enable
const SSTATUS_UPIE: usize = 0b01 << 4; // User Previous Interrupt Enable
const SSTATUS_SIE: usize = 0b01 << 1; // Supervisor Interrupt Enable
const SSTATUS_UIE: usize = 0b01 << 0; // User Interrupt Enable

// Read current value of SSTATUS
pub fn read_sstatus() -> usize {
    let status: usize;
    unsafe {
        asm!(
            "csrr {0}, sstatus",
            out(reg) status,
        );
    }
    status
}

// Write some value to SSTATUS
pub fn write_sstatus(val: usize) {
    unsafe {
        asm!(
            "csrw sstatus, {0}",
            in(reg) val,
        );
    }
}

// Supervisor Interrupt Pending (CSR)
// - Supervisor Software Interrupt Bit (SSIP) [Bit 1] software
// - Supervisor Timer Interrupt Pending (STIP) [Bit 5] hardware
// - Supervisor External Interrupt Pending (SEIP) [Bit 9] I/O devices

// Read some value from SIP
pub fn read_sip() -> usize {
    let pend: usize;
    unsafe {
        asm!(
            "csrr {0}, sip",
            out(reg) pend,
        );
    }
    pend
}

// Write some value to SIP
pub fn write_sip(val: usize) {
    unsafe {
        asm!(
            "csrw sip, {0}",
            in(reg) val,
        );
    }
}

// Supervisor Interrupt Enable

const SIE_SSIE: usize = 0b01 << 1; // Software
const SIE_STIE: usize = 0b01 << 5; // Timer (Hardware)
const SIE_SEIE: usize = 0b01 << 9; // External (Hardware [I/O])

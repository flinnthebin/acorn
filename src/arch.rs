use core::arch::asm;

// Control and Status Register (CSR)

//  __  __            _     _                  _                   _
// |  \/  | __ _  ___| |__ (_)_ __   ___      | |    _____   _____| |
// | |\/| |/ _` |/ __| '_ \| | '_ \ / _ \_____| |   / _ \ \ / / _ \ |
// | |  | | (_| | (__| | | | | | | |  __/_____| |__|  __/\ V /  __/ |
// |_|  |_|\__,_|\___|_| |_|_|_| |_|\___|     |_____\___| \_/ \___|_|
//

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

// Machine Status Register (MSTATUS) (CSR)
// - Machine Previous Privilege (MPP[1:0]): 2-bit field indicating the previous privilege mode (M/S/U) before a trap

const MSTATUS_MPP_MASK: usize = 0b11 << 11; // Mask to isolate the MPP field

#[repr(usize)]
#[derive(Copy, Clone)]
pub enum MStatVal {
    MMV = 0b11 << 11, // Machine-mode value
    UMV = 0b00 << 11, // User-mode value
    SMV = 0b01 << 11, // Supervisor-mode value
    MIE = 1 << 3,     // Machine Interrupt Enable (1 = Enabled, 0 = Disabled)
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
pub fn write_mstatus(val: MStatVal) {
    unsafe {
        asm!(
            "csrw mstatus, {0}",
            in(reg) val as usize,
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

//  ____                              _                     _                   _
// / ___| _   _ _ __   ___ _ ____   _(_)___  ___  _ __     | |    _____   _____| |
// \___ \| | | | '_ \ / _ \ '__\ \ / / / __|/ _ \| '__|____| |   / _ \ \ / / _ \ |
//  ___) | |_| | |_) |  __/ |   \ V /| \__ \ (_) | | |_____| |__|  __/\ V /  __/ |
// |____/ \__,_| .__/ \___|_|    \_/ |_|___/\___/|_|       |_____\___| \_/ \___|_|
//             |_|

// Supervisor Status Register (SSTATUS) (CSR)
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum SStatVal {
    SPP = 0b01 << 8,  // Supervisor Previous Privilege (1 = Supervisor, 0 = User)
    SPIE = 0b01 << 5, // Supervisor Previous Interrupt Enable
    UPIE = 0b01 << 4, // User Previous Interrupt Enable
    SIE = 0b01 << 1,  // Supervisor Interrupt Enable
    UIE = 0b01 << 0,  // User Interrupt Enable
}

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
pub fn write_sstatus(val: SStatVal) {
    unsafe {
        asm!(
            "csrw sstatus, {0}",
            in(reg) val as usize,
        );
    }
}

// Supervisor Interrupt Pending (CSR)
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum SipVal {
    SSIP = 0b01 << 1, // Software
    STIP = 0b01 << 5, // Timer (Hardware)
    SEIP = 0b01 << 9, // Exterrnal (Hardware [I/O])
}

// Read current state of SIP
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
pub fn write_sip(val: SipVal) {
    unsafe {
        asm!(
            "csrw sip, {0}",
            in(reg) val as usize,
        );
    }
}

// Supervisor Interrupt Enable
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum SieVal {
    SSIE = 0b01 << 1, // Software
    STIE = 0b01 << 5, // Timer (Hardware)
    SEIE = 0b01 << 9, // Exterrnal (Hardware [I/O])
}

const SIE_SSIE: usize = 0b01 << 1; // Software
const SIE_STIE: usize = 0b01 << 5; // Timer (Hardware)
const SIE_SEIE: usize = 0b01 << 9; // External (Hardware [I/O])

// Read current state of SIE
pub fn read_sie() -> usize {
    let interrupt: usize;
    unsafe {
        asm!("csrr {0}, sie",
            out(reg) interrupt,
        );
    }
    interrupt
}

// Write some value to SIE
pub fn write_sie(val: SieVal) {
    unsafe {
        asm!(
            "csrw sie, {0}",
            in(reg) val as usize,
        )
    }
}

use core::arch::asm;

// Control and Status Register (CSR) Addresses

// Machine Level
const MHARTID: usize = 0xf14;
const MSTATUS: usize = 0x300;
const MEDELEG: usize = 0x302;
const MIDELEG: usize = 0x303;
const MIE: usize = 0x304;
const MCOUNTEREN: usize = 0x306;
const MENVCFG: usize = 0x30A;
const MEPC: usize = 0x341;
const MCYCLE: usize = 0xB00;
// Supervisor Level
const SSTATUS: usize = 0x100;
const SIE: usize = 0x104;
const STVEC: usize = 0x105;
const SEPC: usize = 0x141;
const SCAUSE: usize = 0x142;
const STVAL: usize = 0x143;
const SIP: usize = 0x144;
const SATP: usize = 0x180;
// Core Local Interruptor Address (Access with CSRR/CSRW)
const STIMECMP: usize = 0x14d;
// Physical Memory Protection
const PMPCFG0: usize = 0x3A0;
const PMPADDR0: usize = 0x3B0;

// Read some value from a CSR register
macro_rules! read_csr {
    ($csr:expr) => {{
        let value: usize;
        unsafe {
            asm!(
                "csrr {0}, {1}",
                out(reg) value,
                const $csr,
                options(nostack, preserves_flags)
            );
        }
        value
    }};
}

// Write some value to a CSR register
macro_rules! write_csr {
    ($csr:expr, $val:expr) => {{
        unsafe {
            asm!(
                "csrw {0}, {1}",
                const $csr,
                in(reg) $val as usize,
                options(nostack, preserves_flags)
            );
        }
    }};
}

//  __  __            _     _                  _                   _
// |  \/  | __ _  ___| |__ (_)_ __   ___      | |    _____   _____| |
// | |\/| |/ _` |/ __| '_ \| | '_ \ / _ \_____| |   / _ \ \ / / _ \ |
// | |  | | (_| | (__| | | | | | | |  __/_____| |__|  __/\ V /  __/ |
// |_|  |_|\__,_|\___|_| |_|_|_| |_|\___|     |_____\___| \_/ \___|_|
//

// Returns Machine Hardware Thread ID
pub fn read_mhartid() -> usize {
    read_csr!(MHARTID)
}

// Read/Write thread pointer, in this architecture holds core hartid
// Core hartid serves as an index into cpus[]
pub fn read_threadptr() -> usize {
    let thread: usize;
    unsafe {
        asm!(
            "mv {0}, tp",
            out(reg) thread,
            options(nostack, preserves_flags)
        );
    }
    thread
}

pub fn write_threadptr(val: usize) {
    unsafe {
        asm!(
            "mv tp, {0}",
            in(reg) val,
            options(nostack, preserves_flags)
        );
    }
}

// Machine Status Register (MSTATUS)
// - Machine Previous Privilege (MPP[1:0]): 2-bit field indicating the previous privilege mode (U/S/M) before a trap

trait MStatusField {
    fn to_usize(self) -> usize;
}

const MPP_MASK: usize = 0b11 << 11; // Mask to isolate the MPP field

#[repr(usize)]
#[derive(Copy, Clone)]
pub enum PrivilegeMode {
    UMV = 0b00 << 11, // User-mode value
    SMV = 0b01 << 11, // Supervisor-mode value
    MMV = 0b11 << 11, // Machine-mode value
}

impl MStatusField for PrivilegeMode {
    fn to_usize(self) -> usize {
        self as usize
    }
}

#[repr(usize)]
#[derive(Copy, Clone)]
pub enum InterruptEnable {
    UIE = 1 << 0, // User Interrupt Enable (1 = Enabled, 0 = Disabled)
    SIE = 1 << 1, // Supervisor Interrupt Enable (1 = Enabled, 0 = Disabled)
    MIE = 1 << 3, // Machine Interrupt Enable (1 = Enabled, 0 = Disabled)
}

impl MStatusField for InterruptEnable {
    fn to_usize(self) -> usize {
        self as usize
    }
}

#[repr(usize)]
#[derive(Copy, Clone)]
pub enum PreviousInterruptEnable {
    UPIE = 1 << 4, // User Previous Interrupt Enable
    SPIE = 1 << 5, // Supervisor Previous Interrupt Enable
    MPIE = 1 << 7, // Machine Previous Interrupt Enable
}

impl MStatusField for PreviousInterruptEnable {
    fn to_usize(self) -> usize {
        self as usize
    }
}

#[repr(usize)]
#[derive(Copy, Clone)]
pub enum FloatingPointStatus {
    OFF = 0b00 << 13,     // Floating-point unit off
    INITIAL = 0b01 << 13, // Floating-point unit initial
    CLEAN = 0b10 << 13,   // Floating-point unit clean
    DIRTY = 0b11 << 13,   // Floating-point unit dirty
}

impl MStatusField for FloatingPointStatus {
    fn to_usize(self) -> usize {
        self as usize
    }
}

#[repr(usize)]
#[derive(Copy, Clone)]
pub enum ExtensionStatus {
    OFF = 0b00 << 15,     // Floating-point unit off
    INITIAL = 0b01 << 15, // Floating-point unit initial
    CLEAN = 0b10 << 15,   // Floating-point unit clean
    DIRTY = 0b11 << 15,   // Floating-point unit dirty
}

impl MStatusField for ExtensionStatus {
    fn to_usize(self) -> usize {
        self as usize
    }
}

#[repr(usize)]
#[derive(Copy, Clone)]
pub enum AdditionalStatus {
    MPRV = 1 << 17, // Modify Privilege
    SUM = 1 << 18,  // Supervisor User Memory Access
    MXR = 1 << 19,  // Make Executable Readable
    TVM = 1 << 20,  // Trap Virtual Memory
    TW = 1 << 21,   // Timeout Wait
    TSR = 1 << 22,  // Trap SRET
}

impl MStatusField for AdditionalStatus {
    fn to_usize(self) -> usize {
        self as usize
    }
}

pub fn read_mstatus() -> usize {
    read_csr!(MSTATUS)
}

pub fn write_mstatus<T: MStatusField>(val: T) {
    write_csr!(MSTATUS, val.to_usize());
}

// Machine Exception Delegation
// Delegates exceptions from machine mode to supervisor mode

#[repr(usize)]
#[derive(Copy, Clone)]
pub enum MedelegVal {
    InstructionAddressMisaligned = 0b01 << 0,
    InstructionAccessFault = 0b01 << 1,
    IllegalInstruction = 0b01 << 2,
    Breakpoint = 0b01 << 3,
    LoadAddressMisaligned = 0b01 << 4,
    LoadAccessFault = 0b01 << 5,
    StoreAddressMisaligned = 0b01 << 6,
    StoreAccessFault = 0b01 << 7,
    EnvironmentCallFromUMode = 0b01 << 8,
    EnvironmentCallFromSMode = 0b01 << 9,
    InstructionPageFault = 0b01 << 12,
    LoadPageFault = 0b01 << 13,
    StorePageFault = 0b01 << 15,
}

impl MedelegField for MedelegVal {
    fn to_usize(self) -> usize {
        self as usize
    }
}

pub fn read_medeleg() -> usize {
    read_csr!(MEDELEG)
}

pub fn write_medeleg<T: MedelegField>(val: T) {
    write_csr!(MEDELEG, val.to_usize());
}
// Machine Interrupt Delegation
// Delegates interrupts from machine mode to supervisor mode
//

trait MidelegField {
    fn to_usize(self) -> usize;
}

#[repr(usize)]
#[derive(Copy, Clone)]
pub enum MidelegVal {
    // Supervisor Level Machine-Mode
    SSIE = 1 << 1, // Software
    STIE = 1 << 5, // Timer (Hardware)
    SEIE = 1 << 9, // External (Hardware [I/O])
}

impl MidelegField for MidelegVal {
    fn to_usize(self) -> usize {
        self as usize
    }
}

pub fn read_mideleg() -> usize {
    read_csr!(MIDELEG)
}

pub fn write_mideleg<T: MidelegField>(val: T) {
    write_csr!(MIDELEG, val.to_usize());
}
// Machine Interrupt Enable
// Controls the enabling/disabling of various interrupts in machine mode

trait MieField {
    fn to_usize(self) -> usize;
}

#[repr(usize)]
#[derive(Copy, Clone)]
pub enum MieVal {
    // Machine Level Machine-Mode
    MSIE = 0b01 << 3,  // Software
    MTIE = 0b01 << 7,  // Timer (Hardware)
    MEIE = 0b01 << 11, // External (Hardware [I/O])
    // Software Level Machine-Mode
    SSIE = 0b01 << 1, // Software
    STIE = 0b01 << 5, // Timer (Hardware)
    SEIE = 0b01 << 9, // External (Hardware [I/O])
}

impl MieField for MieVal {
    fn to_usize(self) -> usize {
        self as usize
    }
}

// Function to read the MIE CSR
pub fn read_mie() -> usize {
    read_csr!(MIE)
}

// Function to write to the MIE CSR
pub fn write_mie<T: MieField>(val: T) {
    write_csr!(MIE, val.to_usize());
}
// Machine-Mode Counter Enable
// Controls the availability of performance counters (cycle, time, instruction) to lower privilege modes

pub fn read_mcounteren() -> usize {
    read_csr!(MCOUNTEREN)
}

pub fn write_mcounteren(val: usize) {
    write_csr!(MCOUNTEREN, val)
}

// Machine Environment Configuration
// Configures environment settings i.e. memory protection attributes, cacheability

pub fn read_menvcfg() -> usize {
    read_csr!(MENVCFG)
}

pub fn write_menvcfg(val: usize) {
    write_csr!(MENVCFG, val)
}

// Machine Exception Program Counter
// Holds the address of an instruction that caused a machine-level exception
// Address is saved when exception occurs and can be used to resume execution or handle the exception

pub fn read_mepc() -> usize {
    read_csr!(MEPC)
}

pub fn write_mepc(addr: usize) {
    write_csr!(MEPC, addr)
}

// Machine-Mode Cycle Counter
// Read-only register. Counts number of processor clock cycles since reset

pub fn read_mcycle() -> usize {
    read_csr!(MCYCLE)
}

//  ____                              _                     _                   _
// / ___| _   _ _ __   ___ _ ____   _(_)___  ___  _ __     | |    _____   _____| |
// \___ \| | | | '_ \ / _ \ '__\ \ / / / __|/ _ \| '__|____| |   / _ \ \ / / _ \ |
//  ___) | |_| | |_) |  __/ |   \ V /| \__ \ (_) | | |_____| |__|  __/\ V /  __/ |
// |____/ \__,_| .__/ \___|_|    \_/ |_|___/\___/|_|       |_____\___| \_/ \___|_|
//             |_|

// Supervisor Status Register (SSTATUS)
trait SStatusField {
    fn to_usize(self) -> usize;
}

#[repr(usize)]
#[derive(Copy, Clone)]
pub enum PrivilegeModeSStatus {
    SPP = 0b01 << 8, // Supervisor Previous Privilege (1 = Supervisor, 0 = User)
}

impl SStatusField for PrivilegeModeSStatus {
    fn to_usize(self) -> usize {
        self as usize
    }
}

#[repr(usize)]
#[derive(Copy, Clone)]
pub enum InterruptEnableSStatus {
    UIE = 0b01 << 0, // User Interrupt Enable (1 = Enabled, 0 = Disabled)
    SIE = 0b01 << 1, // Supervisor Interrupt Enable (1 = Enabled, 0 = Disabled)
}

impl SStatusField for InterruptEnableSStatus {
    fn to_usize(self) -> usize {
        self as usize
    }
}

#[repr(usize)]
#[derive(Copy, Clone)]
pub enum PreviousInterruptEnableSStatus {
    UPIE = 0b01 << 4, // User Previous Interrupt Enable
    SPIE = 0b01 << 5, // Supervisor Previous Interrupt Enable
}

impl SStatusField for PreviousInterruptEnableSStatus {
    fn to_usize(self) -> usize {
        self as usize
    }
}

pub fn read_sstatus() -> usize {
    read_csr!(SSTATUS)
}

pub fn write_sstatus<T: SStatusField>(val: T) {
    write_csr!(SSTATUS, val.to_usize());
}
// Supervisor Interrupt Enable
// Controls the enabling/disabling of various interrupts in supervisor mode

#[repr(usize)]
#[derive(Copy, Clone)]
pub enum SieVal {
    SSIE = 0b01 << 1, // Software
    STIE = 0b01 << 5, // Timer (Hardware)
    SEIE = 0b01 << 9, // External (Hardware [I/O])
}

pub fn read_sie() -> usize {
    read_csr!(SIE)
}

pub fn write_sie(val: SieVal) {
    write_csr!(SIE, val)
}

// Supervisor Trap-Vector Base Address
// Sets base address of trap handler routine for supervisor mode

pub fn read_stvec() -> usize {
    read_csr!(STVEC)
}

pub fn write_stvec(val: usize) {
    write_csr!(STVEC, val)
}

// Supervisor Exception Program Counter
// Holds the address of an instruction that caused a supervisor-level exception
// Address is saved when exception occurs prior to trap handler routine. Can be used to resume execution or handle the exception

pub fn read_sepc() -> usize {
    read_csr!(SEPC)
}

pub fn write_sepc(val: usize) {
    write_csr!(SEPC, val)
}

// Supervisor Trap Cause
// Holds cause of last trap (exception/interrupt) occurence in supervisor mode

pub fn read_scause() -> usize {
    read_csr!(SCAUSE)
}
pub fn write_scause(val: usize) {
    write_csr!(SCAUSE, val)
}

// Supervisor Trap Value
// Contains exception-specific information (address fault, etc) to assist debugging/exception handling

pub fn read_stval() -> usize {
    read_csr!(STVAL)
}

pub fn write_stval(val: usize) {
    write_csr!(STVAL, val)
}

// Supervisor Interrupt Pending
// Each register bit corresponds to a specific interrupt type
// If set, interrupt is pending and waiting to be serviced

#[repr(usize)]
#[derive(Copy, Clone)]
pub enum SipVal {
    SSIP = 0b01 << 1, // Software
    STIP = 0b01 << 5, // Timer (Hardware)
    SEIP = 0b01 << 9, // External (Hardware [I/O])
}

pub fn read_sip() -> usize {
    read_csr!(SIP)
}

pub fn write_sip(val: SipVal) {
    write_csr!(SIP, val)
}

// Supervisor Address Translation and Protection
// Manages address translation/protection, page table configuration and ASIDs
// Integral component in supervisor mode establishment of virtual memory space

// RISC-V Sv39 Page Table Schema
const SATP_SV39: usize = 8 << 60;

fn make_satp(pagetable: usize) -> usize {
    SATP_SV39 | (pagetable >> 12)
}

pub fn read_satp() -> usize {
    read_csr!(SATP)
}

pub fn write_satp(val: usize) {
    write_csr!(SATP, val)
}

// Supervisor Timer Comparison
// Memory-mapped register in Core Local Interruptor (CLINT), not defined in standard CSR set
// Triggers timer interrupts for supervisor mode when STIME == STIMECMP

pub fn read_stimecmp() -> usize {
    read_csr!(STIMECMP)
}

pub fn write_stimecmp(val: usize) {
    write_csr!(STIMECMP, val)
}

//  __  __
// |  \/  | ___ _ __ ___   ___  _ __ _   _
// | |\/| |/ _ \ '_ ` _ \ / _ \| '__| | | |
// | |  | |  __/ | | | | | (_) | |  | |_| |
// |_|  |_|\___|_| |_| |_|\___/|_|   \__, |
//                                   |___/

// Physical Memory Protection Configuration Register 0
// Configures regions 0-3 of PMP, controls permission settings (r/w/x) + addressing mode

pub fn read_pmpcfg0() -> usize {
    read_csr!(PMPCFG0)
}

pub fn write_pmpcfg0(val: usize) {
    write_csr!(PMPCFG0, val)
}

// Physical Memory Protection Address Register 0
// Specifies the address boundary for PMP region 0

pub fn read_pmpaddr0() -> usize {
    read_csr!(PMPADDR0)
}

pub fn write_pmpaddr0(val: usize) {
    write_csr!(PMPADDR0, val)
}

// Return Address Register
// Holds the return address of a function, continution point for program execution

pub fn read_return_addr() -> usize {
    let addr: usize;
    unsafe {
        asm!(
            "mv {0}, ra",
            out(reg) addr,
            options(nostack, preserves_flags)
        );
    }
    addr
}

pub fn write_return_addr(val: usize) {
    unsafe {
        asm!(
            "mv ra, {0}",
            in(reg) val,
            options(nostack, preserves_flags)
        );
    }
}

// Flush the Translation Lookaside Buffer

pub fn flush_tlb() {
    unsafe {
        asm!("sfence.vma zero, zero", options(nostack, preserves_flags));
    }
}

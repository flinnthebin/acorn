use crate::memset::ValidAddress;
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

trait MedelegField {
    fn to_usize(self) -> usize;
}
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

pub fn read_mie() -> usize {
    read_csr!(MIE)
}

pub fn write_mie<T: MieField>(val: T) {
    write_csr!(MIE, val.to_usize());
}
// Machine-Mode Counter Enable
// Controls the availability of performance counters (cycle, time, instruction) to lower privilege modes

trait MCounterenField {
    fn to_usize(self) -> usize;
}

#[repr(usize)]
#[derive(Copy, Clone)]
pub enum MCounterenVal {
    CY = 0b01 << 0,     // Cycle counter
    TM = 0b01 << 1,     // Timer
    IR = 0b01 << 2,     // Instructions-retired counter
    HPM3 = 0b01 << 3,   // Performance-monitoring counter 3
    HPM4 = 0b01 << 4,   // Performance-monitoring counter 4
    HPM5 = 0b01 << 5,   // Performance-monitoring counter 5
    HPM6 = 0b01 << 6,   // Performance-monitoring counter 6
    HPM7 = 0b01 << 7,   // Performance-monitoring counter 7
    HPM8 = 0b01 << 8,   // Performance-monitoring counter 8
    HPM9 = 0b01 << 9,   // Performance-monitoring counter 9
    HPM10 = 0b01 << 10, // Performance-monitoring counter 10
    HPM11 = 0b01 << 11, // Performance-monitoring counter 11
    HPM12 = 0b01 << 12, // Performance-monitoring counter 12
    HPM13 = 0b01 << 13, // Performance-monitoring counter 13
    HPM14 = 0b01 << 14, // Performance-monitoring counter 14
    HPM15 = 0b01 << 15, // Performance-monitoring counter 15
    HPM16 = 0b01 << 16, // Performance-monitoring counter 16
    HPM17 = 0b01 << 17, // Performance-monitoring counter 17
    HPM18 = 0b01 << 18, // Performance-monitoring counter 18
    HPM19 = 0b01 << 19, // Performance-monitoring counter 19
    HPM20 = 0b01 << 20, // Performance-monitoring counter 20
    HPM21 = 0b01 << 21, // Performance-monitoring counter 21
    HPM22 = 0b01 << 22, // Performance-monitoring counter 22
    HPM23 = 0b01 << 23, // Performance-monitoring counter 23
    HPM24 = 0b01 << 24, // Performance-monitoring counter 24
    HPM25 = 0b01 << 25, // Performance-monitoring counter 25
    HPM26 = 0b01 << 26, // Performance-monitoring counter 26
    HPM27 = 0b01 << 27, // Performance-monitoring counter 27
    HPM28 = 0b01 << 28, // Performance-monitoring counter 28
    HPM29 = 0b01 << 29, // Performance-monitoring counter 29
    HPM30 = 0b01 << 30, // Performance-monitoring counter 30
    HPM31 = 0b01 << 31, // Performance-monitoring counter 31
}

impl MCounterenField for MCounterenVal {
    fn to_usize(self) -> usize {
        self as usize
    }
}

pub fn read_mcounteren() -> usize {
    read_csr!(MCOUNTEREN)
}

pub fn write_mcounteren<T: MCounterenField>(val: T) {
    write_csr!(MCOUNTEREN, val.to_usize());
}

// Machine Environment Configuration
// Configures environment settings i.e. memory protection attributes, cacheability

trait MenvcfgField {
    fn to_usize(self) -> usize;
}

#[repr(usize)]
#[derive(Copy, Clone)]
pub enum MenvcfgVal {
    FIOM = 1 << 0,   // Fast I/O Memory
    CBIE = 1 << 1,   // Control Block Interrupt Enable
    CBZE = 1 << 2,   // Control Block Zero Enable
    PMA = 1 << 3,    // Physical Memory Attributes
    PMA1 = 1 << 4,   // Physical Memory Attributes 1
    PMA2 = 1 << 5,   // Physical Memory Attributes 2
    PMA3 = 1 << 6,   // Physical Memory Attributes 3
    PMA4 = 1 << 7,   // Physical Memory Attributes 4
    PMA5 = 1 << 8,   // Physical Memory Attributes 5
    PMA6 = 1 << 9,   // Physical Memory Attributes 6
    PMA7 = 1 << 10,  // Physical Memory Attributes 7
    PMA8 = 1 << 11,  // Physical Memory Attributes 8
    PMA9 = 1 << 12,  // Physical Memory Attributes 9
    PMA10 = 1 << 13, // Physical Memory Attributes 10
    PMA11 = 1 << 14, // Physical Memory Attributes 11
    PMA12 = 1 << 15, // Physical Memory Attributes 12
    PMA13 = 1 << 16, // Physical Memory Attributes 13
    PMA14 = 1 << 17, // Physical Memory Attributes 14
    PMA15 = 1 << 18, // Physical Memory Attributes 15
}

impl MenvcfgField for MenvcfgVal {
    fn to_usize(self) -> usize {
        self as usize
    }
}

pub fn read_menvcfg() -> usize {
    read_csr!(MENVCFG)
}

pub fn write_menvcfg<T: MenvcfgField>(val: T) {
    write_csr!(MENVCFG, val.to_usize());
}

// Machine Exception Program Counter
// Holds the address of an instruction that caused a machine-level exception
// Address is saved when exception occurs and can be used to resume execution or handle the exception

pub fn read_mepc() -> usize {
    read_csr!(MEPC)
}

pub fn write_mepc(addr: ValidAddress) {
    write_csr!(MEPC, addr.get());
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

trait SieField {
    fn to_usize(self) -> usize;
}

#[repr(usize)]
#[derive(Copy, Clone)]
pub enum SieVal {
    SSIE = 1 << 1, // Software
    STIE = 1 << 5, // Timer (Hardware)
    SEIE = 1 << 9, // External (Hardware [I/O])
}

impl SieField for SieVal {
    fn to_usize(self) -> usize {
        self as usize
    }
}

pub fn read_sie() -> usize {
    read_csr!(SIE)
}

pub fn write_sie<T: SieField>(val: T) {
    write_csr!(SIE, val.to_usize());
}
// Supervisor Trap-Vector Base Address
// Sets base address of trap handler routine for supervisor mode

pub fn read_stvec() -> usize {
    read_csr!(STVEC)
}

pub fn write_stvec(addr: ValidAddress) {
    write_csr!(STVEC, addr.get());
}
// Supervisor Exception Program Counter
// Holds the address of an instruction that caused a supervisor-level exception
// Address is saved when exception occurs prior to trap handler routine. Can be used to resume execution or handle the exception

pub fn read_sepc() -> usize {
    read_csr!(SEPC)
}

pub fn write_sepc(addr: ValidAddress) {
    write_csr!(SEPC, addr.get())
}

// Supervisor Trap Cause
// Holds cause of last trap (exception/interrupt) occurence in supervisor mode

trait ScauseField {
    fn to_usize(self) -> usize;
}

#[repr(usize)]
#[derive(Copy, Clone, Debug)]
pub enum ScauseVal {
    // Exception codes
    InstructionAddressMisaligned = 0,
    InstructionAccessFault = 1,
    IllegalInstruction = 2,
    Breakpoint = 3,
    LoadAddressMisaligned = 4,
    LoadAccessFault = 5,
    StoreAddressMisaligned = 6,
    StoreAccessFault = 7,
    EnvironmentCallFromUMode = 8,
    EnvironmentCallFromSMode = 9,
    InstructionPageFault = 12,
    LoadPageFault = 13,
    StorePageFault = 15,
    // Interrupt codes (bit 63 set to 1)
    UserSoftwareInterrupt = 0x8000000000000000 | 0,
    SupervisorSoftwareInterrupt = 0x8000000000000000 | 1,
    UserTimerInterrupt = 0x8000000000000000 | 4,
    SupervisorTimerInterrupt = 0x8000000000000000 | 5,
    UserExternalInterrupt = 0x8000000000000000 | 8,
    SupervisorExternalInterrupt = 0x8000000000000000 | 9,
}

impl ScauseField for ScauseVal {
    fn to_usize(self) -> usize {
        self as usize
    }
}

pub fn read_scause() -> usize {
    read_csr!(SCAUSE)
}

// Typically not used as SCAUSE is set by hardware when trap occurs, but included for completeness
pub fn write_scause<T: ScauseField>(val: T) {
    write_csr!(SCAUSE, val.to_usize());
}

// Supervisor Trap Value
// Contains exception-specific information (address fault, etc) to assist debugging/exception handling

pub fn read_stval() -> usize {
    read_csr!(STVAL)
}

pub fn write_stval(addr: ValidAddress) {
    write_csr!(STVAL, addr.get())
}

// Supervisor Interrupt Pending
// Each register bit corresponds to a specific interrupt type
// If set, interrupt is pending and waiting to be serviced

trait SipField {
    fn to_usize(self) -> usize;
}

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

pub fn write_sip<T: SipField>(val: T) {
    write_csr!(SIP, val.to_usize());
}

// Supervisor Address Translation and Protection
// Manages address translation/protection, page table configuration and ASIDs
// Integral component in supervisor mode establishment of virtual memory space

trait SatpField {
    fn to_usize(self) -> usize;
}

// RISC-V Address Translation Modes
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum SatpMode {
    Bare = 0,       // No translation or protection
    Sv39 = 8 << 60, // Sv39 page-based 39-bit virtual addressing
    Sv48 = 9 << 60, // Sv48 page-based 48-bit virtual addressing
}

impl SatpField for SatpMode {
    fn to_usize(self) -> usize {
        self as usize
    }
}

// Create an SATP value given a page table base address and mode
pub fn make_satp<T: SatpField>(pagetable: usize, mode: T) -> usize {
    mode.to_usize() | (pagetable >> 12)
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

pub fn write_stimecmp(val: TimerCompareValue) {
    write_csr!(STIMECMP, val.get())
}

//  __  __
// |  \/  | ___ _ __ ___   ___  _ __ _   _
// | |\/| |/ _ \ '_ ` _ \ / _ \| '__| | | |
// | |  | |  __/ | | | | | (_) | |  | |_| |
// |_|  |_|\___|_| |_| |_|\___/|_|   \__, |
//                                   |___/

// Physical Memory Protection Configuration Register 0
// Configures regions 0-3 of PMP, controls permission settings (r/w/x) + addressing mode

trait PmpcfgField {
    fn to_usize(self) -> usize;
}

#[repr(usize)]
#[derive(Copy, Clone)]
pub enum PmpcfgVal {
    R = 1 << 0,  // Read permission
    W = 1 << 1,  // Write permission
    X = 1 << 2,  // Execute permission
    A = 1 << 3,  // Address-matching mode
    L = 1 << 7,  // Lock bit
}

impl PmpcfgField for PmpcfgVal {
    fn to_usize(self) -> usize {
        self as usize
    }
}

pub fn read_pmpcfg0() -> usize {
    read_csr!(PMPCFG0)
}

pub fn write_pmpcfg0<T: PmpcfgField>(val: T) {
    write_csr!(PMPCFG0, val.to_usize());
}

// Physical Memory Protection Address Register 0
// Specifies the address boundary for PMP region 0

pub fn read_pmpaddr0() -> usize {
    read_csr!(PMPADDR0)
}

pub fn write_pmpaddr0(val: ValidAddress) {
    write_csr!(PMPADDR0, addr.get())
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

pub fn write_return_addr(val: ValidAddress) {
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

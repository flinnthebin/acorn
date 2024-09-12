pub const KERNEL_BASE_ADDRESS: usize = 0x80000000;
pub const PHYSICAL_MEMORY_LIMIT: usize = KERNEL_BASE_ADDRESS + 128 * 1024 * 1024;

#[derive(Debug, Copy, Clone)]
pub struct ValidAddress(usize);

impl ValidAddress {
    pub fn new(addr: usize) -> Result<Self, &'static str> {
        if addr >= KERNEL_BASE_ADDRESS && addr < PHYSICAL_MEMORY_LIMIT {
            Ok(ValidAddress(addr))
        } else {
            Err("Invalid memory address")
        }
    }

    pub fn get(self) -> usize {
        self.0
    }
}

#[derive(Debug, Copy, Clone)]
pub struct TimerCompareValue(usize);

impl TimerCompareValue {
    pub fn new(val: usize) -> Result<Self, &'static str> {
        if val <= usize::MAX {
            Ok(TimerCompareValue(val))
        } else {
            Err("Invalid timer compare value")
        }
    }

    pub fn get(self) -> usize {
        self.0
    }
}

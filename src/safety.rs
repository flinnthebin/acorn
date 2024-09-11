//  _   _                  __             ___
// | | | |_ __  ___  __ _ / _| ___       / _ \ _ __  ___
// | | | | '_ \/ __|/ _` | |_ / _ \_____| | | | '_ \/ __|
// | |_| | | | \__ \ (_| |  _|  __/_____| |_| | |_) \__ \
//  \___/|_| |_|___/\__,_|_|  \___|      \___/| .__/|___/
//                                            |_|

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

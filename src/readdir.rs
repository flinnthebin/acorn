use libc::{
    AT_FDCWD, DT_BLK, DT_CHR, DT_DIR, DT_FIFO, DT_LNK, DT_REG, DT_SOCK, O_DIRECTORY, O_RDONLY,
};
use std::ffi::{CStr, c_char};
use std::ptr::addr_of;
use syscalls::{Errno, Sysno, syscall};

/// Fixed-size header of the records getdents64 writes into the buffer.
/// Note this is `linux_dirent64`, not the `linux_dirent` from the man page
/// example: d_type is a real field here rather than the last byte of the record.
#[repr(C)]
struct LinuxDirent64 {
    d_ino: u64,
    d_off: i64,
    d_reclen: u16,
    d_type: u8,
    d_name: [c_char; 0], // NUL-terminated name follows the header
}

const BUF_SIZE: usize = 4096;

/// The kernel pads every record to an 8-byte boundary, so an 8-aligned buffer
/// means every record in it is correctly aligned for LinuxDirent64.
#[repr(C, align(8))]
struct DirentBuf([u8; BUF_SIZE]);

/// Lists `path`, printing each entry. Returns the number of entries seen.
pub fn procread(path: &CStr) -> Result<usize, Errno> {
    let mut buf = DirentBuf([0; BUF_SIZE]);
    let mut entries = 0;

    // openat rather than open: SYS_open doesn't exist on aarch64/riscv64.
    let fd = unsafe {
        syscall!(
            Sysno::openat,
            AT_FDCWD,
            path.as_ptr(),
            O_RDONLY | O_DIRECTORY
        )?
    };

    // Run the loop in a closure so the fd is closed on the error path too.
    let result = (|| {
        loop {
            let nread = unsafe { syscall!(Sysno::getdents64, fd, buf.0.as_mut_ptr(), BUF_SIZE)? };
            if nread == 0 {
                break; // end of directory
            }

            println!("--------------- nread={nread} ---------------");
            println!("inode#    file type  d_reclen  d_off   d_name");

            let mut bpos = 0;
            while bpos < nread {
                // SAFETY: the kernel wrote a complete record at bpos, and
                // bpos is a multiple of 8 within an 8-aligned buffer.
                let (ino, off, reclen, d_type, name) = unsafe {
                    let d = buf.0.as_ptr().add(bpos) as *const LinuxDirent64;
                    let name_ptr = addr_of!((*d).d_name) as *const c_char;
                    (
                        (*d).d_ino,
                        (*d).d_off,
                        (*d).d_reclen,
                        (*d).d_type,
                        CStr::from_ptr(name_ptr),
                    )
                };

                let kind = match d_type {
                    DT_REG => "regular",
                    DT_DIR => "directory",
                    DT_FIFO => "FIFO",
                    DT_SOCK => "socket",
                    DT_LNK => "symlink",
                    DT_BLK => "block dev",
                    DT_CHR => "char dev",
                    _ => "???",
                };

                println!(
                    "{ino:8}  {kind:<10} {reclen:4} {off:10}  {}",
                    name.to_string_lossy()
                );

                entries += 1;
                bpos += reclen as usize;
            }
        }
        Ok(entries)
    })();

    unsafe {
        let _ = syscall!(Sysno::close, fd);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_procread() {
        let n = procread(c"/proc").expect("procread failed");
        println!("{n} entries");
        assert!(n > 2); // at least ".", ".." and something else
    }
}

use libc::{O_CREAT, O_RDWR, O_TRUNC, O_WRONLY, S_IRGRP, S_IROTH, S_IRUSR, S_IWUSR, SEEK_SET};
use std::ffi::CStr;
use syscalls::{Errno, Sysno, syscall};

pub fn open_create() -> Result<usize, Errno> {
    let path: &CStr = c"/home/archer/openfile.txt";
    let buf = "Alice and Bob are going exploring in the filesystem. Good luck, Alice and Bob!";
    let mut fer = [0u8; 128];

    unsafe {
        let _f = syscall!(
            Sysno::open,
            path.as_ptr(),
            O_WRONLY | O_CREAT | O_TRUNC,
            S_IRUSR | S_IWUSR | S_IRGRP | S_IROTH
        ); // this is a userspace implementation of the creat() syscall with flags!

        let fd = syscall!(Sysno::open, path.as_ptr(), O_RDWR)?;
        println!("file descriptor: {fd}");

        syscall!(Sysno::write, fd, buf.as_ptr(), buf.len())?;

        syscall!(Sysno::lseek, fd, 0, SEEK_SET)?;
        let n = syscall!(Sysno::read, fd, fer.as_mut_ptr(), fer.len())?;
        println!("{}", String::from_utf8_lossy(&fer[..n]));

        syscall!(Sysno::ftruncate, fd, 0)?;

        Ok(fd)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let fd = open_create().expect("open failed");
        println!("The file descriptor is: {fd}");
        unsafe { syscall!(Sysno::close, fd).unwrap() };
    }
}

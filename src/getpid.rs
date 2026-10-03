use syscalls::{Sysno, syscall};

pub fn getpid() -> usize {
    let mut proc_id: usize = 0;
    if let Ok(pid) = unsafe { syscall!(Sysno::getpid) } {
        proc_id = pid;
    }
    proc_id
}

pub fn getppid() -> usize {
    let mut parent_id: usize = 0;
    if let Ok(ppid) = unsafe { syscall!(Sysno::getppid) } {
        parent_id = ppid
    }
    parent_id
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let child = getpid();
        let parent = getppid();
        println!("\nThe child process is: {child} and the parent process is: {parent}")
    }
}

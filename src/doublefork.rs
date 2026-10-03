use syscalls::{Sysno, syscall};

#[repr(C)]
struct Timespec {
    tv_sec: i64,
    tv_nsec: i64,
}

/// Forks a child, which forks a grandchild and exits. The orphaned grandchild
/// reports its new parent's pid back through a pipe.
pub fn child_of_init() -> Option<usize> {
    unsafe {
        // fds[0] = read end, fds[1] = write end
        let mut fds = [0i32; 2];
        syscall!(Sysno::pipe2, fds.as_mut_ptr(), 0).ok()?;
        let (rd, wr) = (fds[0], fds[1]);

        match syscall!(Sysno::fork) {
            // ---- child ----
            Ok(0) => {
                // New session, no controlling terminal.
                let _ = syscall!(Sysno::setsid);
                let child_pid = syscall!(Sysno::getpid).unwrap_or(0);

                match syscall!(Sysno::fork) {
                    // ---- grandchild ----
                    Ok(0) => {
                        // Wait until we have actually been reparented.
                        let ts = Timespec {
                            tv_sec: 0,
                            tv_nsec: 1_000_000,
                        };
                        while syscall!(Sysno::getppid).unwrap_or(0) == child_pid {
                            let _ = syscall!(
                                Sysno::nanosleep,
                                &ts as *const Timespec,
                                std::ptr::null_mut::<Timespec>()
                            );
                        }

                        let new_parent: usize = syscall!(Sysno::getppid).unwrap_or(0);
                        let _ = syscall!(
                            Sysno::write,
                            wr,
                            &new_parent as *const usize,
                            std::mem::size_of::<usize>()
                        );
                        let _ = syscall!(Sysno::exit_group, 0);
                        unreachable!();
                    }
                    // ---- child: exit straight away to orphan the grandchild ----
                    Ok(_) => {
                        let _ = syscall!(Sysno::exit_group, 0);
                        unreachable!();
                    }
                    Err(_) => {
                        let _ = syscall!(Sysno::exit_group, 1);
                        unreachable!();
                    }
                }
            }

            // ---- original process ----
            Ok(child_pid) => {
                // Close our write end so read() sees EOF if the grandchild
                // dies without writing.
                let _ = syscall!(Sysno::close, wr);

                // Reap the child so it doesn't linger as a zombie.
                let mut status: i32 = 0;
                let _ = syscall!(
                    Sysno::wait4,
                    child_pid,
                    &mut status as *mut i32,
                    0,
                    std::ptr::null_mut::<u8>()
                );

                let mut new_parent: usize = 0;
                let n = syscall!(
                    Sysno::read,
                    rd,
                    &mut new_parent as *mut usize,
                    std::mem::size_of::<usize>()
                );
                let _ = syscall!(Sysno::close, rd);

                match n {
                    Ok(n) if n == std::mem::size_of::<usize>() => Some(new_parent),
                    _ => None,
                }
            }

            Err(_) => {
                let _ = syscall!(Sysno::close, rd);
                let _ = syscall!(Sysno::close, wr);
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let parent = child_of_init().expect("fork/pipe failed");
        println!("\nThe grandchild was reparented to pid {parent}");
        assert_ne!(parent, 0);
    }
}

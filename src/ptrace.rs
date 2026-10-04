use nix::sys::ptrace;
use nix::sys::signal::Signal;
use nix::sys::wait::{WaitStatus, waitpid};
use nix::unistd::Pid;
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // 1. Parse the target PID from command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <PID>", args[0]);
        std::process::exit(1);
    }
    let target_pid = Pid::from_raw(args[1].parse::<i32>()?);

    println!("[*] Attaching to process {}...", target_pid);

    // 2. Attach to the target process.
    // This sends a SIGSTOP to the target, pausing its execution.
    ptrace::attach(target_pid)?;

    // 3. Wait for the target process to actually stop
    match waitpid(target_pid, None)? {
        WaitStatus::Stopped(_, Signal::SIGSTOP) => {
            println!("[+] Successfully attached and paused target.");
        }
        status => {
            eprintln!("[-] Unexpected wait status: {:?}", status);
            ptrace::detach(target_pid, None)?;
            return Ok(());
        }
    }

    // 4. Read the current CPU registers
    // The 'user_regs_struct' contains x86_64 registers like RIP, RAX, RSP, etc.
    let regs = ptrace::getregs(target_pid)?;

    // On x86_64, 'rip' is the Instruction Pointer (the current execution address)
    println!("[+] Target Instruction Pointer (RIP): 0x{:x}", regs.rip);
    println!("[+] Target Stack Pointer (RSP): 0x{:x}", regs.rsp);

    // --- Conceptually, this is where you would inject code ---
    // In a full injector, you would use `ptrace::poke_text` or `process_vm_writev`
    // to write shellcode to memory, change `regs.rip` to point to it,
    // and use `ptrace::setregs` to update the CPU before resuming.
    // ---------------------------------------------------------

    println!("[*] Detaching and resuming target process...");

    // 5. Detach from the process and let it run normally again
    ptrace::detach(target_pid, None)?;

    println!("[+] Done!");
    Ok(())
}

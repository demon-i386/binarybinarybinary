extern crate libc;
extern crate nix;

use nix::sys::signal::*;
use std::ptr;
use std::arch::asm;

#[no_mangle]
fn doMath(){
	let a = 1;
	let b = 2;
	println!("Result: {}", a+b);
}

#[no_mangle]
unsafe fn segfault() {
    let p: *const u32 = ptr::null();

    // read_volative não é otimizado, portanto será executado
    let _ = std::ptr::read_volatile(p);
}

#[no_mangle]
extern "C" fn handle_sigsegv(_: libc::c_int, _: *mut libc::siginfo_t, context: *mut libc::c_void) {
    unsafe {
        let context = &mut *(context as *mut libc::ucontext_t);

        // definindo RIP para o nopslide
        context.uc_mcontext.gregs[libc::REG_RIP as usize] += 3;
    }    
    println!("Caught segfault, but continuing execution!");
}

fn main() {
    let sa = SigAction::new(SigHandler::SigAction(handle_sigsegv), SaFlags::SA_ONSTACK, SigSet::empty());
    unsafe { sigaction(Signal::SIGSEGV, &sa).expect("Failed to set signal handler"); }

    unsafe { segfault(); }
    unsafe {
        asm!(
            "nop",
            "nop",
            "nop",
            "nop",
            "nop",
            "nop",
            "nop",
            "nop",
            "nop",
            "nop",
            "nop",
            "nop",
            "nop",
            "nop",
            "nop",
            "nop",
            "nop",
            "nop",
            "nop",
            "nop",
            "nop",
            "nop",
        );
    }

    println!("Program continues running");
    doMath();
}

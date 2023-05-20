extern crate winapi;
use std::arch::asm;
use winapi::vc::excpt::EXCEPTION_CONTINUE_EXECUTION;
use winapi::um::winnt::PEXCEPTION_POINTERS;
use winapi::um::errhandlingapi::AddVectoredExceptionHandler;
use winapi::um::winnt::CONTEXT;
use winapi::um::minwinbase::EXCEPTION_ACCESS_VIOLATION;
use winapi::vc::excpt::ExceptionContinueExecution;
use winapi::um::winnt::EXCEPTION_POINTERS;
use winapi::um::winnt::EXCEPTION_RECORD;
use winapi::vc::excpt::EXCEPTION_DISPOSITION;
use std::ptr;

#[no_mangle]
pub unsafe extern "system" fn seh_handler(exception_info: PEXCEPTION_POINTERS) -> i32 {
    if (*(*exception_info).ExceptionRecord).ExceptionCode == EXCEPTION_ACCESS_VIOLATION {
        println!("Caught segfault, but continuing execution!");

       //  Define RIP to nopslide
        (*(*exception_info).ContextRecord).Rip += 3;
        
        return EXCEPTION_CONTINUE_EXECUTION;
    }
//    Return ExceptionContinueSearch if not EXCEPTION_ACCESS_VIOLATION
    0
}

fn segfault() {
    unsafe {
        let p: *const u32 = ptr::null();

        // Read_volatile is not optimized, so it will be executed
        let _ = std::ptr::read_volatile(p);
    }
}

fn main() {
    // Set up a Vectored Exception Handler
    unsafe {
        AddVectoredExceptionHandler(1, Some(seh_handler));
    }
    segfault();

    // A series of no operation (nop) instructions
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
    do_math();
    segfault();
}

#[no_mangle]
pub fn do_math() {
    let a = 1;
    let b = 2;
    println!("Result: {}", a + b);
}

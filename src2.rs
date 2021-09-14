#![no_std]
#![no_main]
#![allow(unused)]
#![feature(option_result_unwrap_unchecked, lang_items, asm)]
use core::panic::PanicInfo;
#[lang = "eh_personality"]
extern "C" fn eh_personality() {}
unsafe fn exit_err(){
    asm!("mov rdi,0x1");
    asm!("mov rax,0x60");
    asm!("syscall");
}
#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    unsafe{exit_err();}
    loop {}
}
#[no_mangle]
pub extern "C" fn __libc_csu_fini() {}
#[no_mangle]
pub extern "C" fn __libc_csu_init() {}
#[no_mangle]
pub extern "C" fn __libc_start_main() {
    unsafe {
        asm!("pop rdi");
        asm!("mov rsi,rsp");
        asm!("push rdi");
        asm!("call main");
        asm!("mov rdi,rax");
        asm!("mov rax,0x60");
        asm!("syscall");
    }
}
#[no_mangle]
pub extern "C" fn mainCRTStartup(){}
#[no_mangle]
pub extern "C" fn main(_argc: isize, _argv: *const *const u8) -> isize {
    let x: Option<u8> = None;
    0
}

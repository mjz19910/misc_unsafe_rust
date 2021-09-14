#![no_std]
#![no_main]
#![allow(unused)]
#![feature(option_result_unwrap_unchecked, lang_items, asm)]
use core::panic::PanicInfo;
#[lang = "eh_personality"]
extern "C" fn eh_personality() {}
#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    extern "C" {
        #[cfg_attr(
            target_family = "unix", link_name = "\n\n\x1b[s\x1b[1000D\x1b[0;31m\x1b[1merror\x1b[0m\x1b[1m: the static assertion that no panics are present has failed\x1b[0m\x1b[u\n\n"
        )]
        #[cfg_attr(not(target_family = "unix"), link_name = "\n\nerror: the static assertion that no panics are present has failed\n\n")]
        fn never_panic() -> !;
    }
    
    unsafe { never_panic() }
}
#[no_mangle]
pub extern fn __libc_csu_fini(){}
#[no_mangle]
pub extern fn __libc_csu_init(){}
#[no_mangle]
pub extern "C" fn __libc_start_main(){
    unsafe{
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
pub extern "C" fn main(_argc: isize, _argv: *const *const u8) -> isize {
let x: Option<u8> = None;
return 0;
let _ = unsafe{x.unwrap_unchecked()};
0
}
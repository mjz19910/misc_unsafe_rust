#![feature(lang_items, libc, no_core, static_nobundle)]
#![no_std]
#![no_core]
#![no_main]
#![allow(unused)]
#[lang = "panic_info"]
struct PanicInfo {}
#[panic_handler]
fn panic(info: &PanicInfo) -> ! { loop {} }
#[lang = "eh_personality"]
extern fn eh_personality() {}
#[lang = "sized"]
trait Sized {}
#[lang = "copy"]
trait Copy {}
#[lang = "freeze"]
unsafe trait Freeze {}

#[link(name = "c")]
extern "C" {
    fn write(fd: i32, buf: *const i8, count: usize) -> isize;
    fn exit(status: i32) -> !;
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    let s = b"Hello, World!\n";
    unsafe{
        write(1, s as *const u8 as *const i8, 14);
        exit(0)
    }
}
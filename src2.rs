#![feature(lang_items, libc, no_core, static_nobundle)]
#![feature(rustc_attrs)]
#![feature(arbitrary_self_types)]
#![feature(never_type)]
#![feature(intrinsics, decl_macro)]
#![no_std]
#![no_core]
#![no_main]
#![allow(unused)]

#[lang = "panic_info"]
struct PanicInfo<'a> {
    panic_str:&'a str
}
fn panic_handler(info: &PanicInfo<'_>) -> ! {
    unsafe{
        exit(1);
    }
}
#[lang ="panic_location"]
pub struct Location<'a> {
    file: &'a str,
    line: u32,
    col: u32,
}

#[lang = "eh_personality"]
extern fn eh_personality() {}

#[lang = "sized"]
pub trait Sized {}
#[lang = "copy"]
trait Copy {}
#[lang = "freeze"]
unsafe trait Freeze {}
#[lang = "receiver"]
trait Recv{}


pub mod intrinsic {
    extern "rust-intrinsic" {
        pub fn abort() -> !;
        pub fn transmute<T, U>(e: T) -> U;
    }
}
struct PanicExtra {}

extern "Rust" {
    #[lang = "panic_impl"]
    fn panic_impl(pi: &PanicInfo) -> !;
}

#[rustc_builtin_macro]
pub macro asm("assembly template", $(operands,)* $(options($(option),*))?) {
    /* compiler built-in */
}

#[lang = "panic"] #[no_mangle] #[track_caller]
fn panic(expr: &'static str){
    unsafe{
        panic_impl(&PanicInfo {
            panic_str:expr
        });
    }
}

#[link(name = "c")]
extern "C" {
    fn write(fd: i32, buf: *const i8, count: usize) -> isize;
    fn exit(status: i32) -> !;
}
impl Copy for *const i8 {}
impl Copy for i8 {}
impl Copy for u8 {}
impl Copy for i32 {}
impl Copy for i64 {}
impl Copy for u32 {}
impl Copy for u64 {}

#[lang = "add_assign"]
trait AddAssign<Rhs = Self> {
    fn add_assign(&mut self, rhs: Rhs);
}

impl AddAssign for i32 {
    #[inline]
    #[rustc_inherit_overflow_checks]
    fn add_assign(&mut self, other: i32) { *self += other }
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    let s = b"Hello, World!\n";
    unsafe{
        write(1, s as *const u8 as *const i8, 14);
        exit(0);
    }
}
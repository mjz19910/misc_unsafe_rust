#![feature(layout_for_ptr)]
use std::mem;
use std::slice;

fn main() {
    let fn_ref = &[main];
    let fn_ptr = fn_ref.as_ptr();
    let ptr = unsafe {
        let ptr2 = (*fn_ptr) as *const usize;
        (ptr2 as usize, slice::from_raw_parts(ptr2 as *const u8, 12))
    };
    println!("{:x}", mem::size_of_val(fn_ref));
    println!("{:#x}", ptr.0);
    println!("{:#x}", (fn_ptr as *const usize) as usize);
    println!("{:x?}", ptr.1);
    // cspell:disable-next-line
    let x = 0x784ffcc_u64;
    let fn_2 = || (x,);
    let fn2_ptr = &fn_2 as *const _;
    println!("{:x?}", unsafe {
        let a = fn2_ptr as *const usize;
        (
            fn2_ptr,
            mem::size_of_val(&fn_2),
            slice::from_raw_parts(a, mem::size_of_val(&fn_2) / 8),
        )
    });
}

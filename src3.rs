use std::mem;
use std::slice;

fn main() {
    let fn_ref = &[main];
    let fn_ptr = fn_ref.as_ptr();
    let ptr = unsafe {
        let ptr2 = (*fn_ptr) as *const usize;
        (
            ptr2 as usize,
            (fn_ptr as *const usize) as usize,
            slice::from_raw_parts(ptr2 as *const u8, 12)
        )
    };
    assert_eq!(mem::size_of_val(fn_ref), 0);
    println!("{:#x} -> {:#x}", ptr.0, ptr.1);
    println!("{:#x} -> {:x?}", ptr.1, ptr.2);
    // cspell:disable-next-line
    let x = 0x4151u64;
    let fn_2 = &(||x) as &dyn Fn() -> _;
    let fn2_ptr = fn_2 as *const _;
    println!("{:x?}", unsafe {
        let a = fn2_ptr as *const usize;
        (
            fn2_ptr,
            mem::size_of_val(&fn_2),
            slice::from_raw_parts(a, mem::size_of_val(&fn_2) / 8),
            slice::from_raw_parts((*a) as *const usize, 1),
        )
    });
}

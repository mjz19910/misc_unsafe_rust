use std::slice;

fn main() {
    let fn_ref = &[main];
    let ptr = unsafe {
        let ptr2 = (*fn_ref.as_ptr()) as *const usize;
        (
            ptr2,
            slice::from_raw_parts(ptr2 as *const u8, 12),
        )
    };
    let print_ptr = ptr.0 as usize;
    println!("{:#x}", print_ptr);
    println!("{:x?}", ptr.1);
}

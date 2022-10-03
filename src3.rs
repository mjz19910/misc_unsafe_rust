use std::slice;

fn main() {
    let fn_ref = &&(main as fn());
    let ptr = unsafe {
        let ptr2 = fn_ref as *const _ as *const *const usize;
        (
            **ptr2,
            slice::from_raw_parts(**ptr2 as *const u8, 7),
        )
    };
    let print_ptr = ptr.0 as usize;
    println!("0x{:x}", print_ptr);
    println!("{:x?}", ptr.1);
}

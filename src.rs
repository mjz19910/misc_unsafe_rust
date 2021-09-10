macro_rules! raw_size {
	() => {
		0x8
	};
}
type UnsafeSliceU8=[u8;raw_size!()*std::mem::size_of::<usize>()];

fn main() -> () {
	let raw_slice=unsafe { std::mem::transmute::<_,&UnsafeSliceU8>(main as fn()) };
	let raw_slice_struct=unsafe { std::mem::transmute::<_,&Box<A>>(main as fn()) };
	println!("{}",format!("{:x?}",raw_slice).replace(", ","").replace('[',"").replace(']',""));
	println!("{:x?}",raw_slice_struct._v1);
	()
}

struct A {
	_v1:UnsafeSliceU8
}
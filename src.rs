macro_rules! raw_size {
	() => {
		0xc
	};
}
type UnsafeSliceU8=[u8;raw_size!()*std::mem::size_of::<usize>()];

type UnsafeSliceUsize=[usize;raw_size!()];

fn main() -> () {
	let raw_slice=unsafe { std::mem::transmute::<_,&UnsafeSliceU8>(main as fn()) };
	let _mark : u64=0xdeadbeef;
	let mut raw_slice_struct=unsafe { std::mem::transmute::<_,&A>(&(main as fn())) };
	println!("{}",format!("{:x?}",raw_slice).replace(", ","").replace('[',"").replace(']',""));
	println!("{}",format!("{:#014X?}",raw_slice_struct.0).replace(",\n    ",",\n").replace("[\n    ","").replace("\n]",""));
	()
}

struct A (UnsafeSliceUsize);
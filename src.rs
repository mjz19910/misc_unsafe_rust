macro_rules! raw_size {
	() => {
		0x8
	};
}
type UnsafeSliceU8=[[u8;std::mem::size_of::<usize>()];raw_size!()];

type UnsafeSliceUsize=[usize;raw_size!()];

fn main() -> () {
	let raw_slice=unsafe { std::mem::transmute::<_,&UnsafeSliceU8>(main as fn()) };
	let _mark : u64=0xdeadbeef;
	let _ = raw_slice;
	//println!("{}",format!("{:x?}",raw_slice).replace(", ","").replace('[',"").replace(']',""));
	looping();
	()
}

fn looping (){
	let mut raw_slice_struct=unsafe { std::mem::transmute::<_,&A>(&(main as fn())) };
	let mut i = 0;
	let mut st_i = 0;
	let mut pt2=unsafe {std::mem::transmute::<_,&A>(&raw_slice_struct)}.0;
	let mut rt:*const UnsafeSliceU8=*unsafe {std::mem::transmute::<_,&&UnsafeSliceU8>(&pt2[0])};
	let mut zc=0;
	let stpt=rt as usize/4096;
	let _=stpt;
	//println!("[{:x?}] -> {:x?}",rt,unsafe{*rt.offset(0 as isize)});
	drop(rt);
	while i < 4 {
		// dump the usize data starting at addrof(main)
		st_i+=1;
		pt2=unsafe {std::mem::transmute::<_,&A>(&raw_slice_struct)}.0;
		rt=*unsafe {std::mem::transmute::<_,&&UnsafeSliceU8>(&pt2[0])};
		if st_i < 2 {
			raw_slice_struct=unsafe { std::mem::transmute::<_,&A>(&&raw_slice_struct.0[..]) };
			continue;
		}
		//println!("{}",format!("[{}] {:016x?}",i,&raw_slice_struct.0[..]).replace(", ",","));
		println!("{{{}}} [{},{:#x?}+{:x?}] -> {:02x?}",i,(rt as usize/4096) as isize-stpt as isize,rt as usize-(rt as usize%4096),rt as usize%4096,unsafe{*rt.offset(0 as isize)});
		if &raw_slice_struct.0[0..raw_size!()] == &[0;raw_size!()]{
			zc+=1;
		} else {
			zc=0;
		}
		if zc>16 && (rt as usize%4096) > 0xf00 {
			// break the loop when we hit usize of zero
			break;
		}
		raw_slice_struct=unsafe { std::mem::transmute::<_,&A>(&&raw_slice_struct.0[8..]) };
		i+=1;
	};
	let x=main as fn();
	let x=x as *const u8;
	let x=x as *mut u8;
	let x=unsafe {
		&mut *x as &mut u8	
	};
	println!("{:?}",std::mem::discriminant(&x));
	println!("{:x?}",x);
}

struct A<'a> (& 'a UnsafeSliceUsize);
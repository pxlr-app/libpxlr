use collections::bitvec;
use document::color::I;
use document::sprite::StencilI;
use math::Extent2;

#[test]
fn it_from_buffer() {
	let s = StencilI::from_buffer(
		Extent2::new(2, 2),
		&[I::new(1), I::new(2), I::new(3), I::new(4)],
	);
	assert_eq!(*s.mask, bitvec![1, 1, 1, 1]);
	assert_eq!(*s.data, [I::new(1), I::new(2), I::new(3), I::new(4)]);
}

#[test]
fn it_debugs() {
	let s = StencilI::new(Extent2::new(3, 1));
	assert_eq!(format!("{:?}", s), "StencilI { ⠉⠁ }");
	let s = StencilI::new(Extent2::new(1, 3));
	assert_eq!(format!("{:?}", s), "StencilI { ⠇ }");
	let s = StencilI::new(Extent2::new(5, 3));
	assert_eq!(format!("{:?}", s), "StencilI { ⠿⠿⠇ }");
	let s = StencilI::new(Extent2::new(3, 5));
	assert_eq!(format!("{:?}", s), "StencilI { ⣿⡇\n          ⠉⠁ }");
	let s1 = StencilI {
		size: Extent2::new(2, 2),
		mask: bitvec![1, 0, 1, 0],
		data: vec![I::new(1), I::new(4)],
	};
	assert_eq!(format!("{:?}", s1), "StencilI { ⠃ }");
}

#[test]
fn it_iter() {
	let s = StencilI {
		size: Extent2::new(2, 2),
		mask: bitvec![1, 1, 1, 1],
		data: vec![I::new(1), I::new(2), I::new(3), I::new(4)],
	};
	let mut i = s.iter();
	assert_eq!(i.next(), Some((0, 0, &I::new(1))));
	assert_eq!(i.next(), Some((1, 0, &I::new(2))));
	assert_eq!(i.next(), Some((0, 1, &I::new(3))));
	assert_eq!(i.next(), Some((1, 1, &I::new(4))));
	assert_eq!(i.next(), None);

	let s = StencilI {
		size: Extent2::new(2, 2),
		mask: bitvec![1, 0, 0, 1],
		data: vec![I::new(1), I::new(4)],
	};
	let mut i = s.iter();
	assert_eq!(i.next(), Some((0, 0, &I::new(1))));
	assert_eq!(i.next(), Some((1, 1, &I::new(4))));
	assert_eq!(i.next(), None);
}

#[test]
fn it_combines() {
	let s1 = StencilI {
		size: Extent2::new(2, 2),
		mask: bitvec![1, 0, 0, 1],
		data: vec![I::new(1), I::new(4)],
	};
	assert_eq!(format!("{:?}", s1), "StencilI { ⠑ }");

	let s2 = StencilI {
		size: Extent2::new(2, 2),
		mask: bitvec![0, 1, 1, 0],
		data: vec![I::new(2), I::new(3)],
	};
	assert_eq!(format!("{:?}", s2), "StencilI { ⠊ }");

	let s3 = s1 + s2;
	assert_eq!(*s3.mask, bitvec![1, 1, 1, 1]);
	assert_eq!(*s3.data, [I::new(1), I::new(2), I::new(3), I::new(4)]);
	assert_eq!(format!("{:?}", s3), "StencilI { ⠛ }");

	let s1 = StencilI {
		size: Extent2::new(1, 2),
		mask: bitvec![1, 1],
		data: vec![I::new(1), I::new(3)],
	};
	assert_eq!(format!("{:?}", s1), "StencilI { ⠃ }");

	let s2 = StencilI {
		size: Extent2::new(2, 2),
		mask: bitvec![0, 1, 0, 1],
		data: vec![I::new(2), I::new(4)],
	};
	assert_eq!(format!("{:?}", s2), "StencilI { ⠘ }");

	let s3 = s1 + s2;
	assert_eq!(*s3.mask, bitvec![1, 1, 1, 1]);
	assert_eq!(*s3.data, [I::new(1), I::new(2), I::new(3), I::new(4)]);
	assert_eq!(format!("{:?}", s3), "StencilI { ⠛ }");
}

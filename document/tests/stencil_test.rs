use collections::bitvec;
use document::color::*;
use document::sprite::*;
use math::Extent2;

#[test]
fn it_from_buffer() {
	let s = StencilGrey::from_buffer(
		Extent2::new(2, 2),
		&[Grey::new(1), Grey::new(2), Grey::new(3), Grey::new(4)],
	);
	assert_eq!(*s.mask, bitvec![1, 1, 1, 1]);
	assert_eq!(
		*s.data,
		[Grey::new(1), Grey::new(2), Grey::new(3), Grey::new(4)]
	);
}

#[test]
fn it_debugs() {
	let s = StencilGrey::new(Extent2::new(3, 1));
	assert_eq!(format!("{:?}", s), "StencilGrey { ⠉⠁ }");
	let s = StencilGrey::new(Extent2::new(1, 3));
	assert_eq!(format!("{:?}", s), "StencilGrey { ⠇ }");
	let s = StencilGrey::new(Extent2::new(5, 3));
	assert_eq!(format!("{:?}", s), "StencilGrey { ⠿⠿⠇ }");
	let s = StencilGrey::new(Extent2::new(3, 5));
	assert_eq!(format!("{:?}", s), "StencilGrey { ⣿⡇\n          ⠉⠁ }");
	let s1 = StencilGrey {
		size: Extent2::new(2, 2),
		mask: bitvec![1, 0, 1, 0],
		data: vec![Grey::new(1), Grey::new(4)],
	};
	assert_eq!(format!("{:?}", s1), "StencilGrey { ⠃ }");
}

#[test]
fn it_iter() {
	let s = StencilGrey {
		size: Extent2::new(2, 2),
		mask: bitvec![1, 1, 1, 1],
		data: vec![Grey::new(1), Grey::new(2), Grey::new(3), Grey::new(4)],
	};
	let mut i = s.iter();
	assert_eq!(i.next(), Some((0, 0, &Grey::new(1))));
	assert_eq!(i.next(), Some((1, 0, &Grey::new(2))));
	assert_eq!(i.next(), Some((0, 1, &Grey::new(3))));
	assert_eq!(i.next(), Some((1, 1, &Grey::new(4))));
	assert_eq!(i.next(), None);

	let s = StencilGrey {
		size: Extent2::new(2, 2),
		mask: bitvec![1, 0, 0, 1],
		data: vec![Grey::new(1), Grey::new(4)],
	};
	let mut i = s.iter();
	assert_eq!(i.next(), Some((0, 0, &Grey::new(1))));
	assert_eq!(i.next(), Some((1, 1, &Grey::new(4))));
	assert_eq!(i.next(), None);
}

#[test]
fn it_combines() {
	let s1 = StencilGrey {
		size: Extent2::new(2, 2),
		mask: bitvec![1, 0, 0, 1],
		data: vec![Grey::new(1), Grey::new(4)],
	};
	assert_eq!(format!("{:?}", s1), "StencilGrey { ⠑ }");

	let s2 = StencilGrey {
		size: Extent2::new(2, 2),
		mask: bitvec![0, 1, 1, 0],
		data: vec![Grey::new(2), Grey::new(3)],
	};
	assert_eq!(format!("{:?}", s2), "StencilGrey { ⠊ }");

	let s3 = s1 + s2;
	assert_eq!(*s3.mask, bitvec![1, 1, 1, 1]);
	assert_eq!(
		*s3.data,
		[Grey::new(1), Grey::new(2), Grey::new(3), Grey::new(4)]
	);
	assert_eq!(format!("{:?}", s3), "StencilGrey { ⠛ }");

	let s1 = StencilGrey {
		size: Extent2::new(1, 2),
		mask: bitvec![1, 1],
		data: vec![Grey::new(1), Grey::new(3)],
	};
	assert_eq!(format!("{:?}", s1), "StencilGrey { ⠃ }");

	let s2 = StencilGrey {
		size: Extent2::new(2, 2),
		mask: bitvec![0, 1, 0, 1],
		data: vec![Grey::new(2), Grey::new(4)],
	};
	assert_eq!(format!("{:?}", s2), "StencilGrey { ⠘ }");

	let s3 = s1 + s2;
	assert_eq!(*s3.mask, bitvec![1, 1, 1, 1]);
	assert_eq!(
		*s3.data,
		[Grey::new(1), Grey::new(2), Grey::new(3), Grey::new(4)]
	);
	assert_eq!(format!("{:?}", s3), "StencilGrey { ⠛ }");
}

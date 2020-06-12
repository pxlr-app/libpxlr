use collections::bitvec;
use document::color::*;
use document::sprite::*;
use math::Extent2;

#[test]
fn it_from_buffer() {
	let s = StencilPalette::from_buffer(
		Extent2::new(2, 2),
		&[
			Palette::new(1),
			Palette::new(2),
			Palette::new(3),
			Palette::new(4),
		],
	);
	assert_eq!(*s.mask, bitvec![1, 1, 1, 1]);
	assert_eq!(
		*s.data,
		[
			Palette::new(1),
			Palette::new(2),
			Palette::new(3),
			Palette::new(4)
		]
	);
}

#[test]
fn it_debugs() {
	let s = StencilPalette::new(Extent2::new(3, 1));
	assert_eq!(format!("{:?}", s), "StencilPalette { ⠉⠁ }");
	let s = StencilPalette::new(Extent2::new(1, 3));
	assert_eq!(format!("{:?}", s), "StencilPalette { ⠇ }");
	let s = StencilPalette::new(Extent2::new(5, 3));
	assert_eq!(format!("{:?}", s), "StencilPalette { ⠿⠿⠇ }");
	let s = StencilPalette::new(Extent2::new(3, 5));
	assert_eq!(format!("{:?}", s), "StencilPalette { ⣿⡇\n          ⠉⠁ }");
	let s1 = StencilPalette {
		size: Extent2::new(2, 2),
		mask: bitvec![1, 0, 1, 0],
		data: vec![Palette::new(1), Palette::new(4)],
	};
	assert_eq!(format!("{:?}", s1), "StencilPalette { ⠃ }");
}

#[test]
fn it_iter() {
	let s = StencilPalette {
		size: Extent2::new(2, 2),
		mask: bitvec![1, 1, 1, 1],
		data: vec![
			Palette::new(1),
			Palette::new(2),
			Palette::new(3),
			Palette::new(4),
		],
	};
	let mut i = s.iter();
	assert_eq!(i.next(), Some((0, 0, &Palette::new(1))));
	assert_eq!(i.next(), Some((1, 0, &Palette::new(2))));
	assert_eq!(i.next(), Some((0, 1, &Palette::new(3))));
	assert_eq!(i.next(), Some((1, 1, &Palette::new(4))));
	assert_eq!(i.next(), None);

	let s = StencilPalette {
		size: Extent2::new(2, 2),
		mask: bitvec![1, 0, 0, 1],
		data: vec![Palette::new(1), Palette::new(4)],
	};
	let mut i = s.iter();
	assert_eq!(i.next(), Some((0, 0, &Palette::new(1))));
	assert_eq!(i.next(), Some((1, 1, &Palette::new(4))));
	assert_eq!(i.next(), None);
}

#[test]
fn it_combines() {
	let s1 = StencilPalette {
		size: Extent2::new(2, 2),
		mask: bitvec![1, 0, 0, 1],
		data: vec![Palette::new(1), Palette::new(4)],
	};
	assert_eq!(format!("{:?}", s1), "StencilPalette { ⠑ }");

	let s2 = StencilPalette {
		size: Extent2::new(2, 2),
		mask: bitvec![0, 1, 1, 0],
		data: vec![Palette::new(2), Palette::new(3)],
	};
	assert_eq!(format!("{:?}", s2), "StencilPalette { ⠊ }");

	let s3 = s1 + s2;
	assert_eq!(*s3.mask, bitvec![1, 1, 1, 1]);
	assert_eq!(
		*s3.data,
		[
			Palette::new(1),
			Palette::new(2),
			Palette::new(3),
			Palette::new(4)
		]
	);
	assert_eq!(format!("{:?}", s3), "StencilPalette { ⠛ }");

	let s1 = StencilPalette {
		size: Extent2::new(1, 2),
		mask: bitvec![1, 1],
		data: vec![Palette::new(1), Palette::new(3)],
	};
	assert_eq!(format!("{:?}", s1), "StencilPalette { ⠃ }");

	let s2 = StencilPalette {
		size: Extent2::new(2, 2),
		mask: bitvec![0, 1, 0, 1],
		data: vec![Palette::new(2), Palette::new(4)],
	};
	assert_eq!(format!("{:?}", s2), "StencilPalette { ⠘ }");

	let s3 = s1 + s2;
	assert_eq!(*s3.mask, bitvec![1, 1, 1, 1]);
	assert_eq!(
		*s3.data,
		[
			Palette::new(1),
			Palette::new(2),
			Palette::new(3),
			Palette::new(4)
		]
	);
	assert_eq!(format!("{:?}", s3), "StencilPalette { ⠛ }");
}

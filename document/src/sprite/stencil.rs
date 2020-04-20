use collections::{bitvec, braille_fmt2, BitVec};
use math::Extent2;

use crate::sprite::Pixel;

pub struct Stencil
{
	pub size: Extent2<u32>,
	pub mask: BitVec,
	pub data: Vec<Pixel>,
}

pub struct StencilDataIterator<'a> {
	bit_offset: usize,
	data_offset: usize,
	width: u32,
	height: u32,
	mask: &'a BitVec,
	data: &'a Vec<Pixel>,
}

impl<'a> Iterator for StencilDataIterator<'a> {
	type Item = (u32, u32, &'a Pixel);

	fn next(&mut self) -> Option<(u32, u32, &'a Pixel)> {
		while self.bit_offset < self.mask.len() {
			let bit_offset = self.bit_offset;
			self.bit_offset += 1;
			let bit = self.mask[bit_offset];
			if bit {
				let x = bit_offset % self.width as usize;
				let y = (bit_offset / self.width as usize) | 0;
				self.data_offset += 1;
				return Some((
					x as u32,
					y as u32,
					&self.data[(self.data_offset - 1) as usize],
				));
			}
		}
		return None;
	}
}

impl Stencil {
	pub fn new(size: Extent2<u32>) -> Stencil {
		let buffer: Vec<Pixel> = vec![Pixel::Nil; (size.w * size.h) as usize];
		Stencil::from_buffer(size, &buffer)
	}

	pub fn from_buffer(size: Extent2<u32>, buffer: &[Pixel]) -> Stencil {
		assert_eq!((size.w * size.h) as usize, buffer.len());
		let mask = bitvec![1; (size.w * size.h) as usize];
		let data = buffer.to_vec();
		Stencil {
			size: size,
			mask: mask,
			data: data,
		}
	}

	pub fn iter(&self) -> StencilDataIterator {
		StencilDataIterator {
			bit_offset: 0,
			data_offset: 0,
			width: self.size.w,
			height: self.size.h,
			mask: &self.mask,
			data: &self.data,
		}
	}
}

impl std::fmt::Debug for Stencil
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"Stencil {{ {} }}",
			braille_fmt2(
				&self.mask,
				self.size.w as usize,
				self.size.h as usize,
				"\n          "
			)
		)
	}
}

impl std::ops::Add for Stencil
{
	type Output = Self;

	fn add(self, other: Self) -> Self {
		let size = Extent2::new(self.size.w.max(other.size.w), self.size.h.max(other.size.h));
		let mut mask = bitvec![0; (size.w * size.h) as usize];
		let mut data: Vec<Pixel> = Vec::with_capacity(self.data.len() + other.data.len());
		let mut count_a: usize = 0;
		let mut count_b: usize = 0;
		for i in 0..mask.len() {
			let x = i % size.w as usize;
			let y = (i / size.w as usize) | 0;

			let bit_a = if x < (self.size.w as usize) && y < (self.size.h as usize) {
				let i = y * (self.size.w as usize) + x;
				self.mask[i]
			} else {
				false
			};

			let bit_b = if x < (other.size.w as usize) && y < (other.size.h as usize) {
				let i = y * (other.size.w as usize) + x;
				other.mask[i]
			} else {
				false
			};

			if bit_b {
				data.push(other.data[count_b]);
				mask.set(i, true);
			} else if bit_a {
				data.push(self.data[count_a]);
				mask.set(i, true);
			}

			if bit_a {
				count_a += 1;
			}
			if bit_b {
				count_b += 1;
			}
		}
		Stencil {
			size: size,
			mask: mask,
			data: data,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_from_buffer() {
		let s = Stencil::from_buffer(Extent2::new(2, 2), &[Pixel::I(1), Pixel::I(2), Pixel::I(3), Pixel::I(4)]);
		assert_eq!(*s.mask, bitvec![1, 1, 1, 1]);
		assert_eq!(*s.data, [Pixel::I(1), Pixel::I(2), Pixel::I(3), Pixel::I(4)]);
	}

	#[test]
	fn it_debugs() {
		let s = Stencil::new(Extent2::new(3, 1));
		assert_eq!(format!("{:?}", s), "Stencil { ⠉⠁ }");
		let s = Stencil::new(Extent2::new(1, 3));
		assert_eq!(format!("{:?}", s), "Stencil { ⠇ }");
		let s = Stencil::new(Extent2::new(5, 3));
		assert_eq!(format!("{:?}", s), "Stencil { ⠿⠿⠇ }");
		let s = Stencil::new(Extent2::new(3, 5));
		assert_eq!(format!("{:?}", s), "Stencil { ⣿⡇\n          ⠉⠁ }");
		let s1 = Stencil {
			size: Extent2::new(2, 2),
			mask: bitvec![1, 0, 1, 0],
			data: vec![Pixel::I(1), Pixel::I(4)],
		};
		assert_eq!(format!("{:?}", s1), "Stencil { ⠃ }");
	}

	#[test]
	fn it_iter() {
		let s = Stencil {
			size: Extent2::new(2, 2),
			mask: bitvec![1, 1, 1, 1],
			data: vec![Pixel::I(1), Pixel::I(2), Pixel::I(3), Pixel::I(4)],
		};
		let mut i = s.iter();
		assert_eq!(i.next(), Some((0, 0, &Pixel::I(1))));
		assert_eq!(i.next(), Some((1, 0, &Pixel::I(2))));
		assert_eq!(i.next(), Some((0, 1, &Pixel::I(3))));
		assert_eq!(i.next(), Some((1, 1, &Pixel::I(4))));
		assert_eq!(i.next(), None);

		let s = Stencil {
			size: Extent2::new(2, 2),
			mask: bitvec![1, 0, 0, 1],
			data: vec![Pixel::I(1), Pixel::I(4)],
		};
		let mut i = s.iter();
		assert_eq!(i.next(), Some((0, 0, &Pixel::I(1))));
		assert_eq!(i.next(), Some((1, 1, &Pixel::I(4))));
		assert_eq!(i.next(), None);
	}

	#[test]
	fn it_combines() {
		let s1 = Stencil {
			size: Extent2::new(2, 2),
			mask: bitvec![1, 0, 0, 1],
			data: vec![Pixel::I(1), Pixel::I(4)],
		};
		assert_eq!(format!("{:?}", s1), "Stencil { ⠑ }");

		let s2 = Stencil {
			size: Extent2::new(2, 2),
			mask: bitvec![0, 1, 1, 0],
			data: vec![Pixel::I(2), Pixel::I(3)],
		};
		assert_eq!(format!("{:?}", s2), "Stencil { ⠊ }");

		let s3 = s1 + s2;
		assert_eq!(*s3.mask, bitvec![1, 1, 1, 1]);
		assert_eq!(*s3.data, [Pixel::I(1), Pixel::I(2), Pixel::I(3), Pixel::I(4)]);
		assert_eq!(format!("{:?}", s3), "Stencil { ⠛ }");

		let s1 = Stencil {
			size: Extent2::new(1, 2),
			mask: bitvec![1, 1],
			data: vec![Pixel::I(1), Pixel::I(3)],
		};
		assert_eq!(format!("{:?}", s1), "Stencil { ⠃ }");

		let s2 = Stencil {
			size: Extent2::new(2, 2),
			mask: bitvec![0, 1, 0, 1],
			data: vec![Pixel::I(2), Pixel::I(4)],
		};
		assert_eq!(format!("{:?}", s2), "Stencil { ⠘ }");

		let s3 = s1 + s2;
		assert_eq!(*s3.mask, bitvec![1, 1, 1, 1]);
		assert_eq!(*s3.data, [Pixel::I(1), Pixel::I(2), Pixel::I(3), Pixel::I(4)]);
		assert_eq!(format!("{:?}", s3), "Stencil { ⠛ }");
	}
}

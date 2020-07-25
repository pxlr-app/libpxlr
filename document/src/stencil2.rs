use crate::prelude::*;
use collections::{bitvec, braille_fmt2, BitVec, Lsb0};
use nom::{multi::many_m_n, number::complete::le_u8};

#[derive(Clone)]
pub struct Stencil2 {
	pub size: Extent2<u32>,
	pub mask: BitVec<Lsb0, u8>,
	pub color_mode: ColorMode,
	pub color: Vec<u8>,
}

impl Stencil2 {
	pub fn new(size: Extent2<u32>, color_mode: ColorMode) -> Stencil2 {
		let buffer: Vec<u8> = vec![0u8; (size.w * size.h * color_mode.stride() as u32) as usize];
		Stencil2::from_buffer(size, color_mode, &buffer)
	}

	pub fn from_buffer(size: Extent2<u32>, color_mode: ColorMode, buffer: &[u8]) -> Stencil2 {
		assert_eq!(
			(size.w * size.h * color_mode.stride() as u32) as usize,
			buffer.len()
		);
		let mask = bitvec![Lsb0, u8; 1; (size.w * size.h) as usize];
		let color = buffer.to_vec();
		Stencil2 {
			size,
			mask,
			color_mode,
			color,
		}
	}
}

impl std::fmt::Debug for Stencil2 {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"Stencil2::{:<4?} ( {} )",
			self.color_mode,
			braille_fmt2(
				&self.mask,
				self.size.w as usize,
				self.size.h as usize,
				"\n                 "
			)
		)
	}
}

impl std::ops::Add for Stencil2 {
	type Output = Self;

	fn add(self, other: Self) -> Self {
		assert_eq!(self.color_mode, other.color_mode);
		let stride = self.color_mode.stride() as usize;
		let size = Extent2::new(self.size.w.max(other.size.w), self.size.h.max(other.size.h));
		let mut mask = bitvec![Lsb0, u8; 0; (size.w * size.h) as usize];
		let mut color: Vec<u8> = Vec::with_capacity((size.w * size.h * stride as u32) as usize);
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
				color.extend_from_slice(&other.color[(count_b * stride)..((count_b + 1) * stride)]);
				mask.set(i, true);
			} else if bit_a {
				color.extend_from_slice(&self.color[(count_a * stride)..((count_a + 1) * stride)]);
				mask.set(i, true);
			}

			if bit_a {
				count_a += 1;
			}
			if bit_b {
				count_b += 1;
			}
		}
		Stencil2 {
			size,
			mask,
			color_mode: self.color_mode,
			color,
		}
	}
}

pub struct Stencil2Iterator<'a> {
	bit_offset: usize,
	color_offset: usize,
	width: u32,
	mask: &'a BitVec<Lsb0, u8>,
	color_stride: usize,
	color: &'a Vec<u8>,
}

impl<'a> Iterator for Stencil2Iterator<'a> {
	type Item = (u32, u32, &'a [u8]);

	fn next(&mut self) -> Option<(u32, u32, &'a [u8])> {
		while self.bit_offset < self.mask.len() {
			let bit_offset = self.bit_offset;
			self.bit_offset += 1;
			let bit = self.mask[bit_offset];
			if bit {
				let x = bit_offset % self.width as usize;
				let y = (bit_offset / self.width as usize) | 0;
				self.color_offset += 1;
				return Some((
					x as u32,
					y as u32,
					&self.color[(self.color_offset - 1 * self.color_stride)
						..(self.color_offset * self.color_stride)],
				));
			}
		}
		return None;
	}
}

impl<'a> IntoIterator for &'a Stencil2 {
	type Item = (u32, u32, &'a [u8]);
	type IntoIter = Stencil2Iterator<'a>;

	fn into_iter(self) -> Self::IntoIter {
		Stencil2Iterator {
			bit_offset: 0,
			color_offset: 0,
			width: self.size.w,
			mask: &self.mask,
			color_stride: self.color_mode.stride(),
			color: &self.color,
		}
	}
}

impl parser::Parse for Stencil2 {
	fn parse(bytes: &[u8]) -> nom::IResult<&[u8], Stencil2> {
		let (bytes, size) = Extent2::parse(bytes)?;
		let len = (((size.w * size.h) + 8 - 1) / 8) as usize;
		let (bytes, buffer) = many_m_n(len, len, le_u8)(bytes)?;
		let mask: BitVec<Lsb0, u8> = buffer.into();
		let (bytes, color_mode) = ColorMode::parse(bytes)?;
		let len = (size.w * size.h * color_mode.stride() as u32) as usize;
		let (bytes, color) = many_m_n(len, len, le_u8)(bytes)?;
		Ok((
			bytes,
			Stencil2 {
				size,
				mask,
				color_mode,
				color,
			},
		))
	}
}

impl parser::Write for Stencil2 {
	fn write(&self, writer: &mut dyn io::Write) -> io::Result<usize> {
		let mut size = self.size.write(writer)?;
		let buffer = self.mask.as_slice();
		writer.write(&buffer)?;
		size += buffer.len();
		size += self.color_mode.write(writer)?;
		let buffer = self.color.as_slice();
		writer.write(&buffer)?;
		size += buffer.len();
		Ok(size)
	}
}

#[cfg(test)]
mod tests {
	use crate::prelude::*;
	use collections::bitvec;
	use std::io;

	#[test]
	fn test_from_buffer() {
		let s = Stencil2::from_buffer(Extent2::new(2, 2), ColorMode::Grey, &[1u8, 2, 3, 4]);
		assert_eq!(*s.mask, bitvec![1, 1, 1, 1]);
		assert_eq!(*s.color, [1u8, 2, 3, 4]);
	}

	#[test]
	fn test_debug() {
		let s = Stencil2::new(Extent2::new(3, 1), ColorMode::Grey);
		assert_eq!(format!("{:?}", s), "Stencil2::Grey ( ⠉⠁ )");
		let s = Stencil2::new(Extent2::new(1, 3), ColorMode::Grey);
		assert_eq!(format!("{:?}", s), "Stencil2::Grey ( ⠇ )");
	}

	#[test]
	fn test_combine() {
		let a = Stencil2 {
			size: Extent2::new(2, 2),
			mask: bitvec![Lsb0, u8; 1, 0, 0, 1],
			color_mode: ColorMode::Grey,
			color: vec![1u8, 4],
		};
		assert_eq!(format!("{:?}", a), "Stencil2::Grey ( ⠑ )");
		let b = Stencil2 {
			size: Extent2::new(2, 2),
			mask: bitvec![Lsb0, u8; 0, 1, 1, 0],
			color_mode: ColorMode::Grey,
			color: vec![2u8, 3],
		};
		assert_eq!(format!("{:?}", b), "Stencil2::Grey ( ⠊ )");
		let c = a + b;
		assert_eq!(*c.mask, bitvec![1, 1, 1, 1]);
		assert_eq!(*c.color, [1u8, 2, 3, 4]);
		assert_eq!(format!("{:?}", c), "Stencil2::Grey ( ⠛ )");

		let a = Stencil2 {
			size: Extent2::new(1, 2),
			mask: bitvec![Lsb0, u8; 1, 1],
			color_mode: ColorMode::Grey,
			color: vec![1u8, 3],
		};
		assert_eq!(format!("{:?}", a), "Stencil2::Grey ( ⠃ )");
		let b = Stencil2 {
			size: Extent2::new(2, 2),
			mask: bitvec![Lsb0, u8; 0, 1, 0, 1],
			color_mode: ColorMode::Grey,
			color: vec![2u8, 4],
		};
		assert_eq!(format!("{:?}", b), "Stencil2::Grey ( ⠘ )");
		let c = a + b;
		assert_eq!(*c.mask, bitvec![1, 1, 1, 1]);
		assert_eq!(*c.color, [1u8, 2, 3, 4]);
		assert_eq!(format!("{:?}", c), "Stencil2::Grey ( ⠛ )");
	}

	#[test]
	fn test_iter() {
		let a = Stencil2 {
			size: Extent2::new(2, 2),
			mask: bitvec![Lsb0, u8; 1, 1, 1, 1],
			color_mode: ColorMode::Grey,
			color: vec![1u8, 2, 3, 4],
		};
		let mut i = a.into_iter();
		assert_eq!(i.next(), Some((0, 0, &[1u8][..])));
		assert_eq!(i.next(), Some((1, 0, &[2u8][..])));
		assert_eq!(i.next(), Some((0, 1, &[3u8][..])));
		assert_eq!(i.next(), Some((1, 1, &[4u8][..])));
		assert_eq!(i.next(), None);

		let a = Stencil2 {
			size: Extent2::new(2, 2),
			mask: bitvec![Lsb0, u8; 1, 0, 0, 1],
			color_mode: ColorMode::Grey,
			color: vec![1u8, 4],
		};
		let mut i = a.into_iter();
		assert_eq!(i.next(), Some((0, 0, &[1u8][..])));
		assert_eq!(i.next(), Some((1, 1, &[4u8][..])));
		assert_eq!(i.next(), None);
	}

	#[test]
	fn test_write_parse() {
		use parser::{Parse, Write};
		let a = Stencil2 {
			size: Extent2::new(2, 2),
			mask: bitvec![Lsb0, u8; 1, 1, 1, 1],
			color_mode: ColorMode::Grey,
			color: vec![1u8, 2, 3, 4],
		};
		let mut buffer: io::Cursor<Vec<u8>> = io::Cursor::new(Vec::new());
		let size = a.write(&mut buffer).expect("Could not write Stencil2");
		assert_eq!(size, 14);
		let r = Stencil2::parse(&buffer.get_ref());
		assert_eq!(r.is_ok(), true);
	}
}

use math::Extent2;

pub struct Stencil<T>
where
	T: Default + Copy,
{
	pub size: Extent2<u32>,
	pub mask: Vec<u8>,
	pub data: Vec<T>,
}

pub struct StencilDataIterator<'a, T> {
	bit_offset: u32,
	data_offset: u32,
	width: u32,
	height: u32,
	mask: &'a [u8],
	data: &'a [T],
}

impl<'a, T> Iterator for StencilDataIterator<'a, T> {
	type Item = (u32, u32, &'a T);

	fn next(&mut self) -> Option<(u32, u32, &'a T)> {
		let bit_count = (self.mask.len() * 8) as u32;
		while self.bit_offset < bit_count {
			let bit_offset = self.bit_offset;
			self.bit_offset += 1;
			let uchar_offset = (bit_offset / 8) | 0;
			let bit = self.mask[uchar_offset as usize] & (1 << (bit_offset - uchar_offset * 8));
			if bit != 0 {
				let x = bit_offset % self.width as u32;
				let y = (bit_offset / self.width as u32) | 0;
				self.data_offset += 1;
				return Some((x, y, &self.data[(self.data_offset - 1) as usize]));
			}
		}
		return None;
	}
}

impl<T> Stencil<T>
where
	T: Default + Copy,
{
	pub fn new(size: Extent2<u32>) -> Stencil<T> {
		let buffer = vec![Default::default(); (size.w * size.h) as usize];
		Stencil::from_buffer(size, &buffer)
	}

	pub fn from_buffer(size: Extent2<u32>, buffer: &[T]) -> Stencil<T> {
		assert_eq!((size.w * size.h) as usize, buffer.len());
		let closest_bytes = 1 + (((size.w * size.h) - 1) / 8); // ceil
		let mut mask: Vec<u8> = vec![255u8; closest_bytes as usize];
		for i in buffer.len()..mask.len() * 8 {
			let p = (i / 8) | 0;
			let m = 1 << (i - p * 8);
			mask[p] ^= m;
		}

		let data: Vec<T> = buffer.to_vec();
		Stencil::<T> {
			size: size,
			mask: mask,
			data: data,
		}
	}

	pub fn iter(&self) -> StencilDataIterator<T> {
		StencilDataIterator {
			bit_offset: 0,
			data_offset: 0,
			width: self.size.w,
			height: self.size.h,
			mask: &self.mask[..],
			data: &self.data[..],
		}
	}
}

impl<T> std::fmt::Debug for Stencil<T>
where
	T: Default + Copy,
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let translate: Vec<Vec<u32>> = vec![vec![1, 2, 4, 64], vec![8, 16, 32, 128]];
		let w = ((self.size.w as f32) / 2.).ceil() as usize;
		let h = ((self.size.h as f32) / 4.).ceil() as usize;
		let mut grid = vec![vec![0u32; h]; w];
		// println!("({},{})", w, h);
		for (x, y, _) in self.iter() {
			let ix = ((x as f32) / 2.).floor() as usize;
			let iy = ((y as f32) / 4.).floor() as usize;
			let tx = (x as usize) % 2;
			let ty = (y as usize) % 4;
			// println!("({},{})", ix, iy);
			grid[ix][iy] += translate[tx][ty];
		}
		let mut out: String = "".into();
		for y in 0..h {
			for x in 0..w {
				let x = x as usize;
				let y = y as usize;
				out.push(std::char::from_u32(0x2800 + grid[x][y]).unwrap());
			}
			if y + 1 < h {
				out.push_str("\n          ");
			}
		}
		write!(f, "Stencil {{ {} }}", out)
	}
}

impl<T> std::ops::Add for Stencil<T>
where
	T: Default + Copy,
{
	type Output = Self;

	fn add(self, other: Self) -> Self {
		let size = Extent2::new(self.size.w.max(other.size.w), self.size.h.max(other.size.h));
		let closest_bytes = 1 + (((size.w * size.h) - 1) / 8);
		let mut mask: Vec<u8> = vec![0u8; closest_bytes as usize];
		let mut data: Vec<T> = Vec::with_capacity(self.data.len() + other.data.len());
		let mut count_a: usize = 0;
		let mut count_b: usize = 0;
		for i in 0..mask.len() * 8 {
			let x = i % size.w as usize;
			let y = (i / size.w as usize) | 0;

			let bit_a = if x < (self.size.w as usize) && y < (self.size.h as usize) {
				let i = y * (self.size.w as usize) + x;
				let p = (i / 8) | 0;
				self.mask[p] & (1 << (i - p * 8))
			} else {
				0
			};

			let bit_b = if x < (other.size.w as usize) && y < (other.size.h as usize) {
				let i = y * (other.size.w as usize) + x;
				let p = (i / 8) | 0;
				other.mask[p] & (1 << (i - p * 8))
			} else {
				0
			};

			let p = (i / 8) | 0;
			let m = 1 << (i - p * 8);

			if bit_b != 0 {
				data.push(other.data[count_b]);
				mask[p] ^= m;
			} else if bit_a != 0 {
				data.push(self.data[count_a]);
				mask[p] ^= m;
			}

			if bit_a != 0 {
				count_a += 1;
			}
			if bit_b != 0 {
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
		let s = Stencil::from_buffer(Extent2::new(2, 2), &[1u8, 2u8, 3u8, 4u8]);
		assert_eq!(*s.mask, [15u8]);
		assert_eq!(*s.data, [1u8, 2u8, 3u8, 4u8]);
	}

	#[test]
	fn it_debugs() {
		let s = Stencil::<u8>::new(Extent2::new(3, 1));
		assert_eq!(format!("{:?}", s), "Stencil { ⠉⠁ }");
		let s = Stencil::<u8>::new(Extent2::new(1, 3));
		assert_eq!(format!("{:?}", s), "Stencil { ⠇ }");
		let s = Stencil::<u8>::new(Extent2::new(5, 3));
		assert_eq!(format!("{:?}", s), "Stencil { ⠿⠿⠇ }");
		let s = Stencil::<u8>::new(Extent2::new(3, 5));
		assert_eq!(format!("{:?}", s), "Stencil { ⣿⡇\n          ⠉⠁ }");
		let s1 = Stencil {
			size: Extent2::new(2, 2),
			mask: vec![10u8],
			data: vec![1u8, 4u8],
		};
		assert_eq!(format!("{:?}", s1), "Stencil { ⠘ }");
	}

	#[test]
	fn it_iter() {
		let s = Stencil {
			size: Extent2::new(2, 2),
			mask: vec![15u8],
			data: vec![1u8, 2u8, 3u8, 4u8],
		};
		let mut i = s.iter();
		assert_eq!(i.next(), Some((0, 0, &1u8)));
		assert_eq!(i.next(), Some((1, 0, &2u8)));
		assert_eq!(i.next(), Some((0, 1, &3u8)));
		assert_eq!(i.next(), Some((1, 1, &4u8)));
		assert_eq!(i.next(), None);

		let s = Stencil {
			size: Extent2::new(2, 2),
			mask: vec![9u8],
			data: vec![1u8, 4u8],
		};
		let mut i = s.iter();
		assert_eq!(i.next(), Some((0, 0, &1u8)));
		assert_eq!(i.next(), Some((1, 1, &4u8)));
		assert_eq!(i.next(), None);
	}

	#[test]
	fn it_combines() {
		let s1 = Stencil {
			size: Extent2::new(2, 2),
			mask: vec![9u8],
			data: vec![1u8, 4u8],
		};
		assert_eq!(format!("{:?}", s1), "Stencil { ⠑ }");

		let s2 = Stencil {
			size: Extent2::new(2, 2),
			mask: vec![6u8],
			data: vec![2u8, 3u8],
		};
		assert_eq!(format!("{:?}", s2), "Stencil { ⠊ }");

		let s3 = s1 + s2;
		assert_eq!(*s3.mask, [15u8]);
		assert_eq!(*s3.data, [1u8, 2u8, 3u8, 4u8]);
		assert_eq!(format!("{:?}", s3), "Stencil { ⠛ }");

		let s1 = Stencil {
			size: Extent2::new(1, 2),
			mask: vec![3u8],
			data: vec![1u8, 3u8],
		};
		assert_eq!(format!("{:?}", s1), "Stencil { ⠃ }");

		let s2 = Stencil {
			size: Extent2::new(2, 2),
			mask: vec![10u8],
			data: vec![2u8, 4u8],
		};
		assert_eq!(format!("{:?}", s2), "Stencil { ⠘ }");

		let s3 = s1 + s2;
		assert_eq!(*s3.mask, [15u8]);
		assert_eq!(*s3.data, [1u8, 2u8, 3u8, 4u8]);
		assert_eq!(format!("{:?}", s3), "Stencil { ⠛ }");
	}
}

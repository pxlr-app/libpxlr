use crate::prelude::*;
use collections::{bitvec, braille_fmt2, BitVec, Lsb0};
use nom::{multi::many_m_n, number::complete::le_u8};
// #[cfg(feature = "rayon")]
// use rayon::prelude::*;

#[derive(Clone, Serialize, Deserialize)]
pub struct Stencil {
	pub size: Extent2<u32>,
	pub mask: BitVec<Lsb0, u8>,
	pub channels: Channel,
	pub data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionnedStencil {
	pub position: Vec2<u32>,
	pub stencil: Stencil,
}

impl Stencil {
	/// Create a new empty stencil
	pub fn new(size: Extent2<u32>, channels: Channel) -> Self {
		let buffer: Vec<u8> = vec![0u8; size.w as usize * size.h as usize * channels.size()];
		Self::from_buffer(size, channels, buffer)
	}

	/// Create a stencil from pixel data
	pub fn from_buffer(size: Extent2<u32>, channels: Channel, buffer: Vec<u8>) -> Self {
		assert_eq!(
			size.w as usize * size.h as usize * channels.size(),
			buffer.len()
		);
		let mask = bitvec![Lsb0, u8; 1; (size.w * size.h) as usize];
		Self {
			size,
			mask,
			channels,
			data: buffer,
		}
	}

	/// Create a stencil from pixel data and masking invisible one based on alpha
	pub fn from_buffer_mask_alpha(size: Extent2<u32>, channels: Channel, buffer: Vec<u8>) -> Self {
		if channels & Channel::A != Channel::A {
			Self::from_buffer(size, channels, buffer)
		} else {
			let stride = channels.size();
			assert_eq!(size.w as usize * size.h as usize * stride, buffer.len());
			let mut mask = bitvec![Lsb0, u8; 1; (size.w * size.h) as usize];
			// #[cfg(feature = "rayon")]
			// let chunks = buffer.par_chunks(stride);
			// #[cfg(not(feature = "rayon"))]
			let chunks = buffer.chunks(stride);

			let data = chunks
				.enumerate()
				.filter_map(|(i, pixel)| match channels.a(pixel) {
					Some(A { a }) if a == &0 => {
						mask.set(i, false);
						None
					}
					_ => Some(pixel.to_vec()),
				})
				.flatten()
				.collect::<Vec<_>>();

			Self {
				size,
				mask,
				channels,
				data,
			}
		}
	}

	/// Try to retrieve a pixel at coordinate
	pub fn try_get(&self, x: u32, y: u32) -> Option<&Pixel> {
		let index = (y * self.size.w + x) as usize;
		self.try_index(index)
	}

	/// Try to retrieve a pixel at index
	pub fn try_index(&self, index: usize) -> Option<&Pixel> {
		if self.mask[index] {
			let stride = self.channels.size();
			let count: usize = self.mask[..index]
				.iter()
				.map(|b| if b == &true { 1usize } else { 0usize })
				.sum();
			Some(&self.data[(count * stride)..((count + 1) * stride)])
		} else {
			None
		}
	}

	/// Iterate over pixel of this stencil
	pub fn iter(&self) -> StencilIterator {
		StencilIterator {
			bit_offset: 0,
			data_offset: 0,
			width: self.size.w,
			mask: &self.mask,
			data_stride: self.channels.size(),
			data: &self.data,
		}
	}

	/// Iterate over pixel of this stencil
	pub fn iter_mut(&mut self) -> StencilMutIterator {
		StencilMutIterator {
			bit_offset: 0,
			data_offset: 0,
			width: self.size.w,
			mask: &self.mask,
			data_stride: self.channels.size(),
			data: &mut self.data,
		}
	}

	/// Merge two stencil and blend them together if need be
	pub fn merge(
		frt: &Self,
		frt_offset: Vec2<i32>,
		bck: &Self,
		bck_offset: Vec2<i32>,
		blend_mode: Blend,
		compose_op: Compose,
	) -> Self {
		assert_eq!(frt.channels, bck.channels);
		let stride = frt.channels.size();

		// Calculate new size
		let size = math::Aabr::<i32> {
			min: Vec2::partial_min(frt_offset, bck_offset),
			max: Vec2::partial_max(
				frt_offset + Vec2::new(frt.size.w as i32, frt.size.h as i32),
				bck_offset + Vec2::new(bck.size.w as i32, bck.size.h as i32),
			),
		}
		.size()
		.map(|x| x as u32);

		// Change offset so one is at origin
		let (frt_offset, bck_offset) = if frt_offset.y < bck_offset.y || frt_offset.x < bck_offset.x
		{
			(
				Vec2::new(0u32, 0u32),
				(bck_offset - frt_offset).map(|x| x as u32),
			)
		} else {
			(
				(frt_offset - bck_offset).map(|x| x as u32),
				Vec2::new(0u32, 0u32),
			)
		};

		// Allocate new buffers
		let mut mask = bitvec![Lsb0, u8; 0; (size.w * size.h) as usize];
		let mut data: Vec<u8> = Vec::with_capacity((size.w * size.h * stride as u32) as usize);
		let mut pixel = frt.channels.default_pixel();

		for i in 0..mask.len() {
			let x = (i % size.w as usize) as u32;
			let y = (i / size.w as usize) as u32;

			let frt_color = match (x.checked_sub(frt_offset.x), y.checked_sub(frt_offset.y)) {
				(Some(x), Some(y)) if x < frt.size.w && y < frt.size.h => frt.try_get(x, y),
				_ => None,
			};
			let bck_color = match (x.checked_sub(bck_offset.x), y.checked_sub(bck_offset.y)) {
				(Some(x), Some(y)) if x < bck.size.w && y < bck.size.h => bck.try_get(x, y),
				_ => None,
			};

			match (frt_color, bck_color) {
				(None, None) => mask.set(i, false),
				(Some(frt_pixel), None) => {
					mask.set(i, true);
					data.extend_from_slice(frt_pixel);
				}
				(None, Some(bck_pixel)) => {
					mask.set(i, true);
					data.extend_from_slice(bck_pixel);
				}
				(Some(frt_pixel), Some(bck_pixel)) => {
					mask.set(i, true);
					blend_pixel_into(
						blend_mode,
						compose_op,
						frt.channels,
						frt_pixel,
						bck_pixel,
						&mut pixel[..],
					);
					data.extend_from_slice(&pixel[..]);
				}
			}
		}

		Stencil {
			size,
			mask,
			channels: frt.channels,
			data,
		}
	}
}

impl std::fmt::Debug for Stencil {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"Stencil ( {} )",
			braille_fmt2(
				&self.mask,
				self.size.w as usize,
				self.size.h as usize,
				"\n           "
			)
		)
	}
}

impl std::ops::Add for &Stencil {
	type Output = Stencil;

	fn add(self, other: Self) -> Self::Output {
		Stencil::merge(
			self,
			Vec2::new(0, 0),
			other,
			Vec2::new(0, 0),
			Blend::Normal,
			Compose::Lighter,
		)
	}
}

pub struct StencilIterator<'a> {
	bit_offset: usize,
	data_offset: usize,
	width: u32,
	mask: &'a BitVec<Lsb0, u8>,
	data_stride: usize,
	data: &'a Vec<u8>,
}

impl<'a> Iterator for StencilIterator<'a> {
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
					&self.data[(self.data_offset.wrapping_sub(1) * self.data_stride)
						..(self.data_offset * self.data_stride)],
				));
			}
		}
		return None;
	}
}

impl<'a> IntoIterator for &'a Stencil {
	type Item = (u32, u32, &'a Pixel);
	type IntoIter = StencilIterator<'a>;

	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

pub struct StencilMutIterator<'a> {
	bit_offset: usize,
	data_offset: usize,
	width: u32,
	mask: &'a BitVec<Lsb0, u8>,
	data_stride: usize,
	data: &'a mut Vec<u8>,
}

impl<'a> Iterator for StencilMutIterator<'a> {
	type Item = (u32, u32, &'a mut Pixel);

	fn next<'b>(&'b mut self) -> Option<(u32, u32, &'a mut Pixel)> {
		while self.bit_offset < self.mask.len() {
			let bit_offset = self.bit_offset;
			self.bit_offset += 1;
			let bit = self.mask[bit_offset];
			if bit {
				let x = bit_offset % self.width as usize;
				let y = (bit_offset / self.width as usize) | 0;
				self.data_offset += 1;
				let data: &'b mut [u8] = &mut self.data[(self.data_offset.wrapping_sub(1)
					* self.data_stride)
					..(self.data_offset * self.data_stride)];
				let data = unsafe { std::mem::transmute::<&'b mut [u8], &'a mut [u8]>(data) };
				return Some((x as u32, y as u32, data));
			}
		}
		return None;
	}
}

impl parser::Parse for Stencil {
	fn parse(bytes: &[u8]) -> nom::IResult<&[u8], Stencil> {
		let (bytes, size) = Extent2::parse(bytes)?;
		let len = (((size.w * size.h) + 8 - 1) / 8) as usize;
		let (bytes, buffer) = many_m_n(len, len, le_u8)(bytes)?;
		let mask: BitVec<Lsb0, u8> = buffer.into();
		let (bytes, channels) = le_u8(bytes)?;
		let channels = Channel::from_bits(channels).unwrap();
		let len = (size.w * size.h * channels.size() as u32) as usize;
		let (bytes, data) = many_m_n(len, len, le_u8)(bytes)?;
		Ok((
			bytes,
			Stencil {
				size,
				mask,
				channels,
				data,
			},
		))
	}
}

impl parser::Write for Stencil {
	fn write(&self, writer: &mut dyn io::Write) -> io::Result<usize> {
		let mut size = 1usize;
		size += self.size.write(writer)?;
		let buffer = self.mask.as_slice();
		writer.write(&buffer)?;
		size += buffer.len();
		writer.write(&self.channels.bits().to_le_bytes())?;
		let buffer = self.data.as_slice();
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
		let s = Stencil::from_buffer(Extent2::new(2, 2), Channel::A, vec![1u8, 2, 3, 4]);
		assert_eq!(*s.mask, bitvec![1, 1, 1, 1]);
		assert_eq!(*s.data, [1u8, 2, 3, 4]);
	}

	#[test]
	fn test_from_buffer_mask_alpha() {
		let s = Stencil::from_buffer_mask_alpha(Extent2::new(2, 2), Channel::A, vec![1u8, 0, 0, 4]);
		assert_eq!(*s.mask, bitvec![1, 0, 0, 1]);
		assert_eq!(*s.data, [1u8, 4]);
	}

	#[test]
	fn test_debug() {
		let s = Stencil::new(Extent2::new(3, 1), Channel::A);
		assert_eq!(format!("{:?}", s), "Stencil ( ⠉⠁ )");
		let s = Stencil::new(Extent2::new(1, 3), Channel::A);
		assert_eq!(format!("{:?}", s), "Stencil ( ⠇ )");
	}

	#[test]
	fn test_merge() {
		let a = Stencil {
			size: Extent2::new(2, 2),
			mask: bitvec![Lsb0, u8; 1, 0, 0, 1],
			channels: Channel::A,
			data: vec![1u8, 4],
		};
		assert_eq!(format!("{:?}", a), "Stencil ( ⠑ )");
		let b = Stencil {
			size: Extent2::new(2, 2),
			mask: bitvec![Lsb0, u8; 0, 1, 1, 0],
			channels: Channel::A,
			data: vec![2u8, 3],
		};
		assert_eq!(format!("{:?}", b), "Stencil ( ⠊ )");
		let c = Stencil::merge(
			&a,
			Vec2::new(-2, 0),
			&b,
			Vec2::new(0, 0),
			Blend::Normal,
			Compose::Lighter,
		);
		assert_eq!(format!("{:?}", c), "Stencil ( ⠑⠊ )");
		assert_eq!(c.try_get(0, 0), Some(&[1u8][..]));
		assert_eq!(c.try_get(1, 0), None);
		assert_eq!(c.try_get(2, 0), None);
		assert_eq!(c.try_get(3, 0), Some(&[2u8][..]));
		assert_eq!(c.try_get(0, 1), None);
		assert_eq!(c.try_get(1, 1), Some(&[4u8][..]));
		assert_eq!(c.try_get(2, 1), Some(&[3u8][..]));
		assert_eq!(c.try_get(3, 1), None);

		let a = Stencil {
			size: Extent2::new(1, 2),
			mask: bitvec![Lsb0, u8; 1, 1],
			channels: Channel::A,
			data: vec![1u8, 3],
		};
		assert_eq!(format!("{:?}", a), "Stencil ( ⠃ )");
		let b = Stencil {
			size: Extent2::new(2, 2),
			mask: bitvec![Lsb0, u8; 0, 1, 0, 1],
			channels: Channel::A,
			data: vec![2u8, 4],
		};
		assert_eq!(format!("{:?}", b), "Stencil ( ⠘ )");
		let c = &a + &b;
		assert_eq!(*c.mask, bitvec![1, 1, 1, 1]);
		assert_eq!(*c.data, [1u8, 2, 3, 4]);
		assert_eq!(format!("{:?}", c), "Stencil ( ⠛ )");

		let a = Stencil {
			size: Extent2::new(2, 2),
			mask: bitvec![Lsb0, u8; 1, 1, 0, 1],
			channels: Channel::A,
			data: vec![255u8, 128, 64],
		};
		assert_eq!(format!("{:?}", a), "Stencil ( ⠙ )");
		let b = Stencil {
			size: Extent2::new(2, 2),
			mask: bitvec![Lsb0, u8; 0, 1, 1, 0],
			channels: Channel::A,
			data: vec![128u8, 32],
		};
		assert_eq!(format!("{:?}", b), "Stencil ( ⠊ )");
		let c = &a + &b;
		assert_eq!(*c.mask, bitvec![1, 1, 1, 1]);
		assert_eq!(*c.data, [255u8, 192, 32, 64]);
		assert_eq!(format!("{:?}", c), "Stencil ( ⠛ )");
	}

	#[test]
	fn test_iter() {
		let a = Stencil {
			size: Extent2::new(2, 2),
			mask: bitvec![Lsb0, u8; 1, 1, 1, 1],
			channels: Channel::A,
			data: vec![1u8, 2, 3, 4],
		};
		let mut i = a.iter();
		assert_eq!(i.next(), Some((0, 0, &[1u8][..])));
		assert_eq!(i.next(), Some((1, 0, &[2u8][..])));
		assert_eq!(i.next(), Some((0, 1, &[3u8][..])));
		assert_eq!(i.next(), Some((1, 1, &[4u8][..])));
		assert_eq!(i.next(), None);

		let a = Stencil {
			size: Extent2::new(2, 2),
			mask: bitvec![Lsb0, u8; 1, 0, 0, 1],
			channels: Channel::A,
			data: vec![1u8, 4],
		};
		let mut i = a.iter();
		assert_eq!(i.next(), Some((0, 0, &[1u8][..])));
		assert_eq!(i.next(), Some((1, 1, &[4u8][..])));
		assert_eq!(i.next(), None);
	}

	#[test]
	fn test_iter_mut() {
		let mut a = Stencil {
			size: Extent2::new(2, 2),
			mask: bitvec![Lsb0, u8; 1, 1, 1, 1],
			channels: Channel::A,
			data: vec![1u8, 2, 3, 4],
		};
		let mut i = a.iter_mut();
		i.next().unwrap().2.copy_from_slice(&[10u8][..]);
		assert_eq!(a.data[0], 10u8);
	}

	#[test]
	fn test_write_parse() {
		use parser::{Parse, Write};
		let a = Stencil {
			size: Extent2::new(2, 2),
			mask: bitvec![Lsb0, u8; 1, 1, 1, 1],
			channels: Channel::A,
			data: vec![1u8, 2, 3, 4],
		};
		let mut buffer: io::Cursor<Vec<u8>> = io::Cursor::new(Vec::new());
		let size = a.write(&mut buffer).expect("Could not write Stencil");
		assert_eq!(size, 14);
		let r = Stencil::parse(&buffer.get_ref());
		assert_eq!(r.is_ok(), true);
	}

	#[test]
	fn test_try_get() {
		let a = Stencil {
			size: Extent2::new(2, 2),
			mask: bitvec![Lsb0, u8; 1, 0, 0, 1],
			channels: Channel::A,
			data: vec![1u8, 4],
		};
		assert_eq!(a.try_index(0).unwrap(), &[1u8][..]);
		assert!(a.try_index(1).is_none());
		assert!(a.try_index(2).is_none());
		assert_eq!(a.try_index(3).unwrap(), &[4u8][..]);
		assert_eq!(a.try_get(0, 0).unwrap(), &[1u8][..]);
		assert!(a.try_get(0, 1).is_none());
		assert!(a.try_get(0, 1).is_none());
		assert_eq!(a.try_get(1, 1).unwrap(), &[4u8][..]);
	}
}

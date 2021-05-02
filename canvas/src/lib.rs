mod braille;
use bitvec::{bitvec, order::Lsb0, vec::BitVec};
use braille::braille_fmt2;
use color::*;
use serde::{Deserialize, Serialize};
use vek::{geom::repr_c::Rect, vec::repr_c::extent2::Extent2};

#[derive(Clone, Serialize, Deserialize)]
pub struct Stencil {
	pub rect: Rect<u32, u32>,
	pub mask: BitVec<Lsb0, u8>,
	pub channel: Channel,
	pub data: Vec<u8>,
}

impl Stencil {
	/// Create a new empty stencil
	pub fn new(size: Extent2<u32>, channel: Channel) -> Self {
		let len = (size.w * size.h) as usize;
		let mut buffer = Vec::with_capacity(len * channel.pixel_stride());
		let default_pixel = channel.default_pixel();
		for _ in 0..len {
			buffer.extend_from_slice(&default_pixel);
		}
		Self::from_buffer(size, channel, buffer)
	}

	/// Create a stencil from pixel data
	pub fn from_buffer(size: Extent2<u32>, channel: Channel, buffer: Vec<u8>) -> Self {
		let len = (size.w * size.h) as usize;
		assert_eq!(len * channel.pixel_stride(), buffer.len());
		let mut mask = bitvec![Lsb0, u8; 1; len];
		mask.set_uninitialized(false);
		Self {
			rect: Rect::new(0, 0, size.w, size.h),
			mask,
			channel,
			data: buffer,
		}
	}

	/// Create a stencil from pixel data and masking invisible one based on alpha
	pub fn from_buffer_mask_alpha(size: Extent2<u32>, channel: Channel, buffer: Vec<u8>) -> Self {
		match channel {
			Channel::Lumaa | Channel::LumaaNormal | Channel::Rgba | Channel::RgbaNormal => {
				let len = (size.w * size.h) as usize;
				let stride = channel.pixel_stride();
				assert_eq!(len * stride, buffer.len());
				let mut mask = bitvec![Lsb0, u8; 0; len];
				// #[cfg(feature = "rayon")]
				// let chunks = buffer.par_chunks(stride);
				// #[cfg(not(feature = "rayon"))]
				let chunks = buffer.chunks(stride);

				let data = chunks
					.enumerate()
					.filter_map(|(i, data)| {
						let pixel = Pixel::from_buffer(&data, channel);
						let alpha = match channel {
							Channel::Lumaa | Channel::LumaaNormal => pixel.lumaa().unwrap().alpha,
							Channel::Rgba | Channel::RgbaNormal => pixel.rgba().unwrap().alpha,
							_ => 0,
						};
						if alpha == 0 {
							None
						} else {
							mask.set(i, true);
							Some(data.to_vec())
						}
					})
					.flatten()
					.collect::<Vec<_>>();

				Self {
					rect: Rect::new(0, 0, size.w, size.h),
					mask,
					channel,
					data,
				}
			}
			_ => Self::from_buffer(size, channel, buffer),
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
				self.rect.w as usize,
				self.rect.h as usize,
				"\n           "
			)
		)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_from_buffer() {
		let s = Stencil::from_buffer(Extent2::new(2, 2), Channel::Luma, vec![1u8, 2, 3, 4]);
		assert_eq!(*s.mask, bitvec![1, 1, 1, 1]);
		assert_eq!(*s.data, [1u8, 2, 3, 4]);
	}

	#[test]
	fn test_from_buffer_mask_alpha() {
		let s = Stencil::from_buffer_mask_alpha(
			Extent2::new(2, 2),
			Channel::Lumaa,
			vec![1u8, 255, 0, 0, 0, 0, 4, 1],
		);
		assert_eq!(*s.mask, bitvec![1, 0, 0, 1]);
		assert_eq!(*s.data, [1u8, 255, 4, 1]);
	}

	// #[test]
	// fn test_debug() {
	// 	let s = Stencil::new(Extent2::new(3, 1), Channel::Luma);
	// 	assert_eq!(format!("{:?}", s), "Stencil ( ⠉⠁ )");
	// 	let s = Stencil::new(Extent2::new(1, 3), Channel::Luma);
	// 	assert_eq!(format!("{:?}", s), "Stencil ( ⠇ )");
	// }
}

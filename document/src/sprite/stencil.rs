use crate::color::*;
use collections::{bitvec, braille_fmt2, BitVec};
use math::Extent2;
use serde::{Deserialize, Serialize};
use std::default::Default;

pub struct StencilDataIterator<'a, T> {
	bit_offset: usize,
	data_offset: usize,
	width: u32,
	height: u32,
	mask: &'a BitVec,
	data: &'a Vec<T>,
}

impl<'a, T> Iterator for StencilDataIterator<'a, T> {
	type Item = (u32, u32, &'a T);

	fn next(&mut self) -> Option<(u32, u32, &'a T)> {
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

pub trait IStencil {
	type Color: IColor;

	fn new(size: Extent2<u32>, mask: BitVec, data: Vec<Self::Color>) -> Self;
}

macro_rules! define_stencil {
	($name:ident $color:ident) => {
		#[derive(Serialize, Deserialize)]
		pub struct $name
		{
			pub size: Extent2<u32>,
			pub mask: BitVec,
			pub data: Vec<$color>,
		}

		impl $name {
			pub fn new(size: Extent2<u32>) -> $name {
				let buffer: Vec<$color> = vec![Default::default(); (size.w * size.h) as usize];
				$name::from_buffer(size, &buffer)
			}

			pub fn from_buffer(size: Extent2<u32>, buffer: &[$color]) -> $name {
				assert_eq!((size.w * size.h) as usize, buffer.len());
				let mask = bitvec![1; (size.w * size.h) as usize];
				let data = buffer.to_vec();
				$name {
					size: size,
					mask: mask,
					data: data,
				}
			}

			pub fn iter(&self) -> StencilDataIterator<$color> {
				StencilDataIterator::<$color> {
					bit_offset: 0,
					data_offset: 0,
					width: self.size.w,
					height: self.size.h,
					mask: &self.mask,
					data: &self.data,
				}
			}
		}

		impl IStencil for $name {
			type Color = $color;

			fn new(size: Extent2<u32>, mask: BitVec, data: Vec<Self::Color>) -> Self {
				$name {
					size,
					mask,
					data,
				}
			}
		}

		impl std::fmt::Debug for $name
		{
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				write!(
					f,
					"{} {{ {} }}",
					stringify!($name),
					braille_fmt2(
						&self.mask,
						self.size.w as usize,
						self.size.h as usize,
						"\n          "
					)
				)
			}
		}

		impl std::ops::Add for $name
		{
			type Output = Self;

			fn add(self, other: Self) -> Self {
				let size = Extent2::new(self.size.w.max(other.size.w), self.size.h.max(other.size.h));
				let mut mask = bitvec![0; (size.w * size.h) as usize];
				let mut data: Vec<$color> = Vec::with_capacity(self.data.len() + other.data.len());
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
				$name {
					size: size,
					mask: mask,
					data: data,
				}
			}
		}
	};
}

define_stencil!(StencilPalette Palette);
define_stencil!(StencilRGBA RGBA);
define_stencil!(StencilUV UV);
define_stencil!(StencilNormal Normal);

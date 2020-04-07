use math::Extent2;
use std::rc::Rc;

pub struct Stencil<T> {
	pub size: Rc<Extent2<u16>>,
	pub mask: Rc<Vec<u8>>,
	pub data: Rc<Vec<T>>,
}

pub struct StencilDataIterator<'a, T> {
	bit_offset: u32,
	data_offset: u32,
	width: u16,
	height: u16,
	mask: &'a [u8],
	data: &'a [T],
}

impl<'a, T> Iterator for StencilDataIterator<'a, T> {
	type Item = (u32, u32, &'a T);

	fn next(&mut self) -> Option<(u32, u32, &'a T)> {
		let bit_offset = self.bit_offset;
		let bit_count = (self.width * self.height * 8) as u32;
		if bit_offset >= bit_count {
			None
		} else {
			self.bit_offset += 1;
			let uchar_offset = (bit_offset / 8) | 0;
			let bit = !!(self.mask[uchar_offset as usize] & (1 << (bit_offset - uchar_offset * 8)));
			if bit == 1 {
				let x = bit_offset % self.width as u32;
				let y = (bit_offset / self.width as u32) | 0;
				self.data_offset += 1;
				Some((x, y, &self.data[(self.data_offset - 1) as usize]))
			} else {
				None
			}
		}
	}
}

impl<T> Stencil<T>
where
	T: Clone,
{
	pub fn new(size: Extent2<u16>, mask: Vec<u8>, data: Vec<T>) -> Stencil<T> {
		Stencil::<T> {
			size: Rc::new(size),
			mask: Rc::new(mask),
			data: Rc::new(data),
		}
	}

	pub fn from_buffer(size: Extent2<u16>, buffer: &[T]) -> Stencil<T> {
		let closest_bytes = (size.w * size.h) / 8;
		let mask: Vec<u8> = vec![255; closest_bytes as usize];
		let data: Vec<T> = buffer.to_vec();
		Stencil::<T>::new(size, mask, data)
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

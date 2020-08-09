use crate::prelude::*;
use collections::{map_rm_to_brm, Bytes, BytesChain};

#[derive(Debug)]
pub struct Canvas {
	pub size: Extent2<u32>,
	pub region: Extent2<u32>,
	pub channels: Channel,
	pub data: BytesChain,
}

impl Canvas {
	pub fn new(size: Extent2<u32>, channels: Channel, data: Vec<u8>) -> Self {
		let data = {
			let mut chain = BytesChain::new();
			let (len, w, h) = (data.len(), size.w as usize, size.h as usize);
			let mut remapped: Vec<u8> = Vec::with_capacity(len);
			unsafe {
				remapped.set_len(len);
			}
			let stride = channels.len();
			for i in 0..len {
				remapped[i] = data[map_rm_to_brm(2 * stride, 2 * stride, w, h, i)];
			}
			chain.push(Bytes::from(remapped));
			chain
		};
		Canvas {
			size,
			region: size,
			channels,
			data,
		}
	}

	pub fn get_pixel_data_at(&self, coord: Vec2<u32>) -> BytesChain {
		let i = coord.x + self.size.w * coord.y;
		self.get_pixel_data(i as usize)
	}

	pub fn get_pixel_data(&self, index: usize) -> BytesChain {
		self.data.slice(index..index + self.channels.len())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	// use crate::prelude::*;

	#[test]
	fn test_blocked_data() {
		let data = (0..25).collect::<Vec<u8>>();
		let canvas = Canvas::new(Extent2::new(5, 5), Channel::A, data);
		let mut buf = vec![0u8; 25];
		canvas.data.copy_to_slice(0..25, &mut buf[..]);
		#[rustfmt::skip]
		assert_eq!(buf, &[0u8, 1, 4, 5, 8, 2, 3, 6, 7, 9, 10, 11, 14, 15, 18, 12, 13, 16, 17, 19, 20, 21, 22, 23, 24][..]);
		assert_eq!(canvas.get_pixel_data_at(Vec2::new(0, 0))[0], 0);
		assert_eq!(canvas.get_pixel_data(0)[0], 0);
	}
}

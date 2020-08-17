use crate::prelude::*;
use bytes::Bytes;

#[derive(Debug)]
pub struct Canvas {
	pub size: Extent2<u32>,
	pub channels: Channel,
	pub data: Bytes,
	pub stencils: Vec<Stencil>,
}

impl Canvas {
	pub fn new(size: Extent2<u32>, channels: Channel, data: Vec<u8>) -> Self {
		Canvas {
			size,
			channels,
			data: Bytes::from(data),
			stencils: Vec::new(),
		}
	}

	// pub fn get_pixel_data_at(&self, coord: Vec2<u32>) -> BytesChain {
	// 	let ri = coord.x + self.size.w * coord.y;
	// 	let bri = map_rm_to_brm(
	// 		self.tile.w as usize,
	// 		self.tile.h as usize,
	// 		self.size.w as usize,
	// 		self.size.h as usize,
	// 		ri as usize,
	// 	);
	// 	self.get_pixel_data(bri)
	// }

	// pub fn get_pixel_data(&self, index: usize) -> BytesChain {
	// 	let bri = map_rm_to_brm(
	// 		self.tile.w as usize,
	// 		self.tile.h as usize,
	// 		self.size.w as usize,
	// 		self.size.h as usize,
	// 		index,
	// 	);
	// 	self.data.slice(bri..bri + self.channels.len())
	// }
}

#[cfg(test)]
mod tests {
	use super::*;
	// use crate::prelude::*;
}

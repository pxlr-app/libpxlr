use crate::interpolation::Interpolation;
use collections::{bitvec, Lsb0};
use document::prelude::*;
use math::{Extent2, Lerp, Vec2};

pub struct Line {
	pub from: Vec2<u32>,
	pub to: Vec2<u32>,
	pub width: u32,
	pub channels: Channel,
	pub color: Vec<u8>,
	pub interpolation: Interpolation,
}

impl Line {
	pub fn get_stencil(&self) -> Stencil {
		let size = Extent2::new(
			self.from.x.max(self.to.x) + 1,
			self.from.y.max(self.to.y) + 1,
		);
		let mut mask = bitvec![Lsb0, u8; 0; (size.w * size.h) as usize];
		let steps = ((self.to.x as i32) - (self.from.x as i32))
			.abs()
			.max(((self.to.y as i32) - (self.from.y as i32)).abs());
		let mut data: Vec<u8> = vec![0u8; (steps + 1) as usize * self.channels.size()];

		// for step in 0..steps + 1 {
		// 	let t = if steps == 0 {
		// 		0.0
		// 	} else {
		// 		(step as f32) / (steps as f32)
		// 	};
		// 	let v = Lerp::lerp_unclamped(self.from.map(|x| x as i32), self.to.map(|x| x as i32), t);
		// 	let i = ((v.y * (size.w as i32)) + v.x) as usize;
		// 	data.push(self.color);
		// 	mask.set(i, true);
		// }
		// S::new(size.map(|x| x as u32), mask, data)
		Stencil {
			size,
			mask,
			channels: self.channels,
			data,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use document::prelude::*;

	#[test]
	fn test_it_paints() {
		let line = Line {
			from: Vec2::new(0, 0),
			to: Vec2::new(0, 9),
			width: 1,
			channels: Channel::I,
			color: vec![255],
			interpolation: Interpolation::Nearest,
		};
		let stencil = line.get_stencil();

		assert_eq!(
			format!("{:?}", stencil),
			"StencilGrey { ⡇\n          ⡇\n          ⠃ }"
		);

		let line = Line {
			from: Vec2::new(9, 0),
			to: Vec2::new(0, 9),
			width: 1,
			channels: Channel::I,
			color: vec![255],
			interpolation: Interpolation::Nearest,
		};
		let stencil = line.get_stencil();

		assert_eq!(
			format!("{:?}", stencil),
			"StencilGrey { ⠀⠀⠀⡠⠊\n          ⠀⡠⠊⠀⠀\n          ⠊⠀⠀⠀⠀ }"
		);
	}
}

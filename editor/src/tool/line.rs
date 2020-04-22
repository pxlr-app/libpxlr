use collections::bitvec;
use document::sprite::Stencil;
use math::interpolation::Interpolation;
use math::{Extent2, Lerp, Vec2};

pub struct Line<S>
where
	S: Stencil,
{
	pub from: Vec2<u32>,
	pub to: Vec2<u32>,
	pub width: u32,
	pub color: S::Color,
	pub interpolation: Interpolation,
}

impl<S> Line<S>
where
	S: Stencil,
{
	pub fn get_stencil(&self) -> S {
		let size = Extent2::new(
			self.from.x.max(self.to.x) + 1,
			self.from.y.max(self.to.y) + 1,
		);
		let mut mask = bitvec![0; (size.w * size.h) as usize];
		let steps = ((self.to.x as i32) - (self.from.x as i32))
			.abs()
			.max(((self.to.y as i32) - (self.from.y as i32)).abs());
		let mut data: Vec<S::Color> = Vec::with_capacity((steps + 1) as usize);

		for step in 0..steps + 1 {
			let t = if steps == 0 {
				0.0
			} else {
				(step as f32) / (steps as f32)
			};
			let v = Lerp::lerp_unclamped(self.from.map(|x| x as i32), self.to.map(|x| x as i32), t);
			let i = ((v.y * (size.w as i32)) + v.x) as usize;
			data.push(self.color);
			mask.set(i, true);
		}
		S::new(size.map(|x| x as u32), mask, data)
	}
}

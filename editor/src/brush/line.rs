use collections::bitvec;
use document::sprite::{Blend, Interpolation, Stencil};
use math::{Extent2, Lerp, Vec2};

use crate::brush::Brush;

struct Line<T>
where
	T: Default + Copy + Blend<Output = T> + Lerp<f32, Output = T> + std::fmt::Debug,
{
	from: Vec2<u32>,
	to: Vec2<u32>,
	width: u32,
	color: T,
	interpolation: Interpolation,
}

impl<T> Brush<T> for Line<T>
where
	T: Default + Copy + Blend<Output = T> + Lerp<f32, Output = T> + std::fmt::Debug,
{
	fn get_stencil(&self) -> Stencil<T> {
		let size = Extent2::new(
			self.from.x.max(self.to.x) + 1,
			self.from.y.max(self.to.y) + 1,
		);
		let mut mask = bitvec![0; (size.w * size.h) as usize];
		let steps = ((self.to.x as i32) - (self.from.x as i32))
			.abs()
			.max(((self.to.y as i32) - (self.from.y as i32)).abs());
		let mut data: Vec<T> = Vec::with_capacity((steps + 1) as usize);

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
		Stencil {
			size: size.map(|x| x as u32),
			mask: mask,
			data: data,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use document::sprite::{Blend, BlendMode};
	use math::Vec2;

	#[derive(Default, Copy, Clone, PartialEq, Debug)]
	struct Gray(u8);

	impl Lerp<f32> for Gray {
		type Output = Gray;
		fn lerp_unclamped(from: Gray, to: Gray, factor: f32) -> Gray {
			Gray(Lerp::lerp_unclamped(from.0, to.0, factor))
		}
	}

	impl Blend for Gray {
		type Output = Gray;
		fn blend(from: &Gray, to: &Gray, mode: &BlendMode) -> Gray {
			match mode {
				// BlendMode::Normal => Gray(to.0),
				BlendMode::Add => Gray(from.0 + to.0),
				BlendMode::Subtract => Gray(from.0 - to.0),
				BlendMode::Multiply => Gray(from.0 * to.0),
				BlendMode::Divide => Gray(from.0 / to.0),
				_ => Gray(to.0),
			}
		}
	}

	#[test]
	fn it_paints() {
		let line = Line {
			from: Vec2::new(0, 0),
			to: Vec2::new(0, 9),
			width: 1,
			color: Gray(255),
			interpolation: Interpolation::Nearest,
		};
		let stencil = line.get_stencil();

		assert_eq!(
			format!("{:?}", stencil),
			"Stencil { ⡇\n          ⡇\n          ⠃ }"
		);

		let line = Line {
			from: Vec2::new(9, 0),
			to: Vec2::new(0, 9),
			width: 1,
			color: Gray(255),
			interpolation: Interpolation::Nearest,
		};
		let stencil = line.get_stencil();

		assert_eq!(
			format!("{:?}", stencil),
			"Stencil { ⠀⠀⠀⡠⠊\n          ⠀⡠⠊⠀⠀\n          ⠊⠀⠀⠀⠀ }"
		);
	}
}

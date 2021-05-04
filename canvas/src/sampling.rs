use crate::Canvas;
use color::{ChannelError, Pixel, PixelMut};
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum Sampling {
	Nearest,
	Bilinear,
	// Bicubic,
}

pub trait Sampler {
	type Output;

	fn sample2d<'target>(&self, position: (f32, f32), sampling: Sampling) -> Self::Output;
}

impl Sampler for Canvas {
	type Output = Result<Vec<u8>, ChannelError>;

	fn sample2d<'target>(
		&self,
		position: (f32, f32),
		sampling: Sampling,
	) -> Result<Vec<u8>, ChannelError> {
		let rect = self.rect();
		match sampling {
			Sampling::Nearest => {
				let x = position.0.round().clamp(rect.x as f32, rect.w as f32 - 1.);
				let y = position.1.round().clamp(rect.y as f32, rect.h as f32 - 1.);
				Ok(self[(x as i32, y as i32)].to_vec())
			}
			Sampling::Bilinear => {
				let l = position.0.floor().clamp(rect.x as f32, rect.w as f32 - 1.);
				let r = (l + 1f32).clamp(rect.x as f32, rect.w as f32 - 1.);
				let t = position.1.floor().clamp(rect.y as f32, rect.h as f32 - 1.);
				let b = (t + 1f32).clamp(rect.y as f32, rect.h as f32 - 1.);
				let hw = position.0 - l;
				let vw = position.1 - t;

				let (tl, tr, bl, br) = (
					&self[(l as i32, t as i32)],
					&self[(r as i32, t as i32)],
					&self[(l as i32, b as i32)],
					&self[(r as i32, b as i32)],
				);

				let mut w_buf = self.channel.default_pixel();
				let mut v_buf = self.channel.default_pixel();
				let mut r_buf = self.channel.default_pixel();
				{
					let mut tmp = PixelMut::from_buffer_mut(&mut w_buf, self.channel);
					let from = Pixel::from_buffer(tl, self.channel);
					let to = Pixel::from_buffer(tr, self.channel);
					tmp.lerp(&from, &to, hw)?;
				}
				{
					let mut tmp = PixelMut::from_buffer_mut(&mut v_buf, self.channel);
					let from = Pixel::from_buffer(bl, self.channel);
					let to = Pixel::from_buffer(br, self.channel);
					tmp.lerp(&from, &to, hw)?;
				}

				{
					let mut tmp = PixelMut::from_buffer_mut(&mut r_buf, self.channel);
					let from = Pixel::from_buffer(&w_buf, self.channel);
					let to = Pixel::from_buffer(&v_buf, self.channel);
					tmp.lerp(&from, &to, vw)?;
				}
				Ok(r_buf)
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::*;
	use color::*;
	use vek::geom::repr_c::Rect;

	#[test]
	fn nearest_sample() {
		let canvas = Canvas::from_stencil(Stencil::from_buffer_mask_alpha(
			Rect::new(0, 0, 2, 2),
			Channel::Lumaa,
			vec![5, 255, 10, 255, 15, 255, 20, 255],
		));

		let buffer = canvas.sample2d((0., 0.), Sampling::Nearest).unwrap();
		assert_eq!(buffer, vec![5, 255]);
		let buffer = canvas.sample2d((0.5, 0.), Sampling::Nearest).unwrap();
		assert_eq!(buffer, vec![10, 255]);
		let buffer = canvas.sample2d((1., 0.), Sampling::Nearest).unwrap();
		assert_eq!(buffer, vec![10, 255]);
		let buffer = canvas.sample2d((0., 0.5), Sampling::Nearest).unwrap();
		assert_eq!(buffer, vec![15, 255]);
		let buffer = canvas.sample2d((0.5, 0.5), Sampling::Nearest).unwrap();
		assert_eq!(buffer, vec![20, 255]);
		let buffer = canvas.sample2d((1., 1.), Sampling::Nearest).unwrap();
		assert_eq!(buffer, vec![20, 255]);
	}

	#[test]
	fn bilinear_sample() {
		let canvas = Canvas::from_stencil(Stencil::from_buffer_mask_alpha(
			Rect::new(0, 0, 2, 2),
			Channel::Lumaa,
			vec![5, 255, 10, 255, 15, 255, 20, 255],
		));

		let buffer = canvas.sample2d((0., 0.), Sampling::Bilinear).unwrap();
		assert_eq!(buffer, vec![5, 255]);
		let buffer = canvas.sample2d((0.5, 0.), Sampling::Bilinear).unwrap();
		assert_eq!(buffer, vec![8, 255]);
		let buffer = canvas.sample2d((1., 0.), Sampling::Bilinear).unwrap();
		assert_eq!(buffer, vec![10, 255]);
		let buffer = canvas.sample2d((0., 0.5), Sampling::Bilinear).unwrap();
		assert_eq!(buffer, vec![10, 255]);
		let buffer = canvas.sample2d((0.5, 0.5), Sampling::Bilinear).unwrap();
		assert_eq!(buffer, vec![13, 255]);
		let buffer = canvas.sample2d((1., 1.), Sampling::Bilinear).unwrap();
		assert_eq!(buffer, vec![20, 255]);
	}
}

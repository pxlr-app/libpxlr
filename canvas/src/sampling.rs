use crate::Canvas;
use color::{ChannelError, Pixel, PixelMut};
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum Sampling {
	Nearest,
	Bilinear,
	// Bicubic,
}

pub trait Samplable {
	type Error: std::error::Error;

	fn sample2d<'samplable, 'out>(
		&'samplable self,
		position: (f32, f32),
		sampling: Sampling,
		out: &'out mut [u8],
	) -> Result<(), Self::Error>;
}

impl Samplable for Canvas {
	type Error = ChannelError;

	fn sample2d<'samplable, 'out>(
		&'samplable self,
		position: (f32, f32),
		sampling: Sampling,
		out: &'out mut [u8],
	) -> Result<(), Self::Error> {
		assert_eq!(out.len(), self.channel.pixel_stride());
		let bounds = self.bounds();
		match sampling {
			Sampling::Nearest => {
				let x = position
					.0
					.round()
					.clamp(bounds.x as f32, bounds.w as f32 - 1.);
				let y = position
					.1
					.round()
					.clamp(bounds.y as f32, bounds.h as f32 - 1.);
				out.copy_from_slice(&self[(x as i32, y as i32)]);
				Ok(())
			}
			Sampling::Bilinear => {
				let l = position
					.0
					.floor()
					.clamp(bounds.x as f32, bounds.w as f32 - 1.);
				let r = (l + 1f32).clamp(bounds.x as f32, bounds.w as f32 - 1.);
				let t = position
					.1
					.floor()
					.clamp(bounds.y as f32, bounds.h as f32 - 1.);
				let b = (t + 1f32).clamp(bounds.y as f32, bounds.h as f32 - 1.);
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
				out.copy_from_slice(&r_buf);
				Ok(())
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

		let mut buffer = Channel::Lumaa.default_pixel();
		canvas
			.sample2d((0., 0.), Sampling::Nearest, &mut buffer)
			.unwrap();
		assert_eq!(buffer, vec![5, 255]);
		let mut buffer = Channel::Lumaa.default_pixel();
		canvas
			.sample2d((0.5, 0.), Sampling::Nearest, &mut buffer)
			.unwrap();
		assert_eq!(buffer, vec![10, 255]);
		let mut buffer = Channel::Lumaa.default_pixel();
		canvas
			.sample2d((1., 0.), Sampling::Nearest, &mut buffer)
			.unwrap();
		assert_eq!(buffer, vec![10, 255]);
		let mut buffer = Channel::Lumaa.default_pixel();
		canvas
			.sample2d((0., 0.5), Sampling::Nearest, &mut buffer)
			.unwrap();
		assert_eq!(buffer, vec![15, 255]);
		let mut buffer = Channel::Lumaa.default_pixel();
		canvas
			.sample2d((0.5, 0.5), Sampling::Nearest, &mut buffer)
			.unwrap();
		assert_eq!(buffer, vec![20, 255]);
		let mut buffer = Channel::Lumaa.default_pixel();
		canvas
			.sample2d((1., 1.), Sampling::Nearest, &mut buffer)
			.unwrap();
		assert_eq!(buffer, vec![20, 255]);
	}

	#[test]
	fn bilinear_sample() {
		let canvas = Canvas::from_stencil(Stencil::from_buffer_mask_alpha(
			Rect::new(0, 0, 2, 2),
			Channel::Lumaa,
			vec![5, 255, 10, 255, 15, 255, 20, 255],
		));

		let mut buffer = Channel::Lumaa.default_pixel();
		canvas
			.sample2d((0., 0.), Sampling::Bilinear, &mut buffer)
			.unwrap();
		assert_eq!(buffer, vec![5, 255]);
		let mut buffer = Channel::Lumaa.default_pixel();
		canvas
			.sample2d((0.5, 0.), Sampling::Bilinear, &mut buffer)
			.unwrap();
		assert_eq!(buffer, vec![8, 255]);
		let mut buffer = Channel::Lumaa.default_pixel();
		canvas
			.sample2d((1., 0.), Sampling::Bilinear, &mut buffer)
			.unwrap();
		assert_eq!(buffer, vec![10, 255]);
		let mut buffer = Channel::Lumaa.default_pixel();
		canvas
			.sample2d((0., 0.5), Sampling::Bilinear, &mut buffer)
			.unwrap();
		assert_eq!(buffer, vec![10, 255]);
		let mut buffer = Channel::Lumaa.default_pixel();
		canvas
			.sample2d((0.5, 0.5), Sampling::Bilinear, &mut buffer)
			.unwrap();
		assert_eq!(buffer, vec![13, 255]);
		let mut buffer = Channel::Lumaa.default_pixel();
		canvas
			.sample2d((1., 1.), Sampling::Bilinear, &mut buffer)
			.unwrap();
		assert_eq!(buffer, vec![20, 255]);
	}
}

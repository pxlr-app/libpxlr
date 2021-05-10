use crate::{Canvas, Stencil};
use color::{ChannelError, Pixel, PixelMut};
use vek::vec::repr_c::vec2::Vec2;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Sampling {
	Nearest,
	Bilinear,
	// Bicubic,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SamplingError {
	ChannelError(ChannelError),
	OutOfBounds,
	Empty,
}

impl std::error::Error for SamplingError {}

impl std::fmt::Display for SamplingError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			SamplingError::ChannelError(chan) => write!(f, "{}", chan),
			SamplingError::OutOfBounds => write!(f, "Position is out of bounds"),
			SamplingError::Empty => write!(f, "Position is out of bounds"),
		}
	}
}

impl From<ChannelError> for SamplingError {
	fn from(value: ChannelError) -> SamplingError {
		SamplingError::ChannelError(value)
	}
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
	type Error = SamplingError;

	fn sample2d<'samplable, 'out>(
		&'samplable self,
		position: (f32, f32),
		sampling: Sampling,
		out: &'out mut [u8],
	) -> Result<(), Self::Error> {
		let channel = self.channel();
		assert_eq!(out.len(), channel.pixel_stride());
		let bounds = self.bounds();
		if !bounds.contains_point(Vec2::new(position.0 as i32, position.1 as i32)) {
			out.copy_from_slice(&channel.default_pixel());
			return Err(SamplingError::OutOfBounds);
		}
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

				let mut w_buf = channel.default_pixel();
				let mut v_buf = channel.default_pixel();
				let mut r_buf = channel.default_pixel();
				{
					let mut tmp = PixelMut::from_buffer_mut(&mut w_buf, channel);
					let from = Pixel::from_buffer(tl, channel);
					let to = Pixel::from_buffer(tr, channel);
					tmp.lerp(&from, &to, hw)?;
				}
				{
					let mut tmp = PixelMut::from_buffer_mut(&mut v_buf, channel);
					let from = Pixel::from_buffer(bl, channel);
					let to = Pixel::from_buffer(br, channel);
					tmp.lerp(&from, &to, hw)?;
				}

				{
					let mut tmp = PixelMut::from_buffer_mut(&mut r_buf, channel);
					let from = Pixel::from_buffer(&w_buf, channel);
					let to = Pixel::from_buffer(&v_buf, channel);
					tmp.lerp(&from, &to, vw)?;
				}
				out.copy_from_slice(&r_buf);
				Ok(())
			}
		}
	}
}

impl Samplable for Stencil {
	type Error = SamplingError;

	fn sample2d<'samplable, 'out>(
		&'samplable self,
		position: (f32, f32),
		sampling: Sampling,
		out: &'out mut [u8],
	) -> Result<(), Self::Error> {
		let channel = self.channel();
		assert_eq!(out.len(), channel.pixel_stride());
		let bounds = self.bounds();
		if !bounds.contains_point(Vec2::new(position.0 as i32, position.1 as i32)) {
			return Err(SamplingError::OutOfBounds);
		}
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
				if let Some(data) = self.try_get(x as i32, y as i32) {
					out.copy_from_slice(data);
					Ok(())
				} else {
					Err(SamplingError::Empty)
				}
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
					&self.try_get(l as i32, t as i32),
					&self.try_get(r as i32, t as i32),
					&self.try_get(l as i32, b as i32),
					&self.try_get(r as i32, b as i32),
				);

				if let (None, None, None, None) = (tl, tr, bl, br) {
					Err(SamplingError::Empty)
				} else {
					let empty = channel.default_pixel();
					let tl = tl.or_else(|| Some(&empty)).take().unwrap();
					let tr = tr.or_else(|| Some(&empty)).take().unwrap();
					let bl = bl.or_else(|| Some(&empty)).take().unwrap();
					let br = br.or_else(|| Some(&empty)).take().unwrap();
					let mut w_buf = channel.default_pixel();
					let mut v_buf = channel.default_pixel();
					let mut r_buf = channel.default_pixel();
					{
						let mut tmp = PixelMut::from_buffer_mut(&mut w_buf, channel);
						let from = Pixel::from_buffer(tl, channel);
						let to = Pixel::from_buffer(tr, channel);
						tmp.lerp(&from, &to, hw)?;
					}
					{
						let mut tmp = PixelMut::from_buffer_mut(&mut v_buf, channel);
						let from = Pixel::from_buffer(bl, channel);
						let to = Pixel::from_buffer(br, channel);
						tmp.lerp(&from, &to, hw)?;
					}

					{
						let mut tmp = PixelMut::from_buffer_mut(&mut r_buf, channel);
						let from = Pixel::from_buffer(&w_buf, channel);
						let to = Pixel::from_buffer(&v_buf, channel);
						tmp.lerp(&from, &to, vw)?;
					}
					out.copy_from_slice(&r_buf);
					Ok(())
				}
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use color::Channel;
	use crate::*;
	use vek::geom::repr_c::Rect;

	#[test]
	fn canvas_nearest_sample() {
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
	fn canvas_bilinear_sample() {
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

	#[test]
	fn stencil_nearest_sample() {
		let stencil = Stencil::from_buffer_mask_alpha(
			Rect::new(0, 0, 2, 2),
			Channel::Lumaa,
			vec![5, 255, 10, 255, 0, 0, 20, 255],
		);

		let mut buffer = Channel::Lumaa.default_pixel();
		stencil
			.sample2d((0., 0.), Sampling::Nearest, &mut buffer)
			.unwrap();
		assert_eq!(buffer, vec![5, 255]);
		let mut buffer = Channel::Lumaa.default_pixel();
		stencil
			.sample2d((0.5, 0.), Sampling::Nearest, &mut buffer)
			.unwrap();
		assert_eq!(buffer, vec![10, 255]);
		let mut buffer = Channel::Lumaa.default_pixel();
		stencil
			.sample2d((1., 0.), Sampling::Nearest, &mut buffer)
			.unwrap();
		assert_eq!(buffer, vec![10, 255]);
		let mut buffer = Channel::Lumaa.default_pixel();
		let res = stencil
			.sample2d((0., 0.5), Sampling::Nearest, &mut buffer);
		assert_eq!(res, Err(SamplingError::Empty));
		assert_eq!(buffer, vec![0, 0]);
		let mut buffer = Channel::Lumaa.default_pixel();
		stencil
			.sample2d((0.5, 0.5), Sampling::Nearest, &mut buffer)
			.unwrap();
		assert_eq!(buffer, vec![20, 255]);
		let mut buffer = Channel::Lumaa.default_pixel();
		stencil
			.sample2d((1., 1.), Sampling::Nearest, &mut buffer)
			.unwrap();
		assert_eq!(buffer, vec![20, 255]);

		let mut buffer = Channel::Lumaa.default_pixel();
		let res = stencil
			.sample2d((-1., 0.), Sampling::Nearest, &mut buffer);
		assert_eq!(res, Err(SamplingError::OutOfBounds));
		assert_eq!(buffer, vec![0, 0]);
	}

	#[test]
	fn stencil_bilinear_sample() {
		let stencil = Stencil::from_buffer_mask_alpha(
			Rect::new(0, 0, 2, 2),
			Channel::Lumaa,
			vec![5, 255, 10, 255, 0, 0, 20, 255],
		);

		let mut buffer = Channel::Lumaa.default_pixel();
		stencil
			.sample2d((0., 0.), Sampling::Bilinear, &mut buffer)
			.unwrap();
		assert_eq!(buffer, vec![5, 255]);
		let mut buffer = Channel::Lumaa.default_pixel();
		stencil
			.sample2d((0.5, 0.), Sampling::Bilinear, &mut buffer)
			.unwrap();
		assert_eq!(buffer, vec![8, 255]);
		let mut buffer = Channel::Lumaa.default_pixel();
		stencil
			.sample2d((1., 0.), Sampling::Bilinear, &mut buffer)
			.unwrap();
		assert_eq!(buffer, vec![10, 255]);
		let mut buffer = Channel::Lumaa.default_pixel();
		stencil
			.sample2d((0., 0.5), Sampling::Bilinear, &mut buffer)
			.unwrap();
		assert_eq!(buffer, vec![3, 128]);
		let mut buffer = Channel::Lumaa.default_pixel();
		stencil
			.sample2d((0.5, 0.5), Sampling::Bilinear, &mut buffer)
			.unwrap();
		assert_eq!(buffer, vec![9, 192]);
		let mut buffer = Channel::Lumaa.default_pixel();
		stencil
			.sample2d((1., 1.), Sampling::Bilinear, &mut buffer)
			.unwrap();
		assert_eq!(buffer, vec![20, 255]);
	}
}

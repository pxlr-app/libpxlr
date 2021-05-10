use crate::{Canvas, Samplable, Sampling, Stencil};
use bitvec::{bitvec, order::Lsb0};
use color::ChannelError;
#[cfg(feature = "rayon")]
use rayon::prelude::*;
use vek::{
	geom::repr_c::{Aabr, Rect},
	mat::repr_c::column_major::{Mat3, Mat4},
	vec::repr_c::vec2::Vec2,
};

pub trait Transformable {
	type Output;

	fn transform(&self, sampling: Sampling, matrix: &Mat3<f32>) -> Self::Output;
}

impl Transformable for Canvas {
	type Output = Result<Canvas, ChannelError>;

	fn transform(&self, sampling: Sampling, matrix: &Mat3<f32>) -> Result<Canvas, ChannelError> {
		let channel = self.channel();
		let stride = channel.pixel_stride();
		let old_bounds = self.bounds();

		// Calculate new bounds
		let half_size: Aabr<f32> = old_bounds.into_aabr().as_();
		let half_size = half_size.half_size();
		let tl = matrix.mul_point_2d(Vec2::new(-half_size.w, -half_size.h));
		let tr = matrix.mul_point_2d(Vec2::new(half_size.w, -half_size.h));
		let bl = matrix.mul_point_2d(Vec2::new(-half_size.w, half_size.h));
		let br = matrix.mul_point_2d(Vec2::new(half_size.w, half_size.h));
		let l = tl.x.min(tr.x).min(bl.x).min(br.x);
		let t = tl.y.min(tr.y).min(bl.y).min(br.y);
		let r = tl.x.max(tr.x).max(bl.x).max(br.x);
		let b = tl.y.max(tr.y).max(bl.y).max(br.y);
		let new_bounds = Rect::new(
			old_bounds.x,
			old_bounds.y,
			(r - l).ceil() as i32,
			(b - t).ceil() as i32,
		);

		// Center canvas
		let projection = Mat3::<f32>::translation_2d(Vec2::new(
			old_bounds.w as f32 / -2f32 + 0.5,
			old_bounds.h as f32 / -2f32 + 0.5,
		));
		// Apply transformation
		let projection = (*matrix) * projection;
		// Decenter canvas
		let projection = Mat3::<f32>::translation_2d(-Vec2::new(
			new_bounds.w as f32 / -2f32 + 0.5,
			new_bounds.h as f32 / -2f32 + 0.5,
		)) * projection;
		// Invert matrix
		let projection = Into::<Mat3<f32>>::into(Into::<Mat4<f32>>::into(projection).inverted());

		let mut data: Vec<u8> = vec![0u8; new_bounds.w as usize * new_bounds.h as usize * stride];
		let pitch = new_bounds.w as usize * stride;

		#[cfg(feature = "rayon")]
		let chunks = data.par_chunks_mut(pitch);
		#[cfg(not(feature = "rayon"))]
		let chunks = data.chunks_mut(pitch);

		chunks.enumerate().for_each(|(y, row)| {
			for (x, slice) in row.chunks_mut(stride).enumerate() {
				let pos = projection.mul_point_2d(Vec2::new(x as f32, y as f32));
				let _ = self.sample2d((pos.x, pos.y), sampling, slice);
			}
		});
		Ok(Canvas::from_stencil(Stencil::from_buffer_mask_alpha(
			new_bounds, channel, data,
		)))
	}
}

impl Transformable for Stencil {
	type Output = Result<Stencil, ChannelError>;

	fn transform(&self, sampling: Sampling, matrix: &Mat3<f32>) -> Result<Stencil, ChannelError> {
		let channel = self.channel();
		let stride = channel.pixel_stride();
		let old_bounds = self.bounds();

		// Calculate new bounds
		let half_size: Aabr<f32> = old_bounds.into_aabr().as_();
		let half_size = half_size.half_size();
		let tl = matrix.mul_point_2d(Vec2::new(-half_size.w, -half_size.h));
		let tr = matrix.mul_point_2d(Vec2::new(half_size.w, -half_size.h));
		let bl = matrix.mul_point_2d(Vec2::new(-half_size.w, half_size.h));
		let br = matrix.mul_point_2d(Vec2::new(half_size.w, half_size.h));
		let l = tl.x.min(tr.x).min(bl.x).min(br.x);
		let t = tl.y.min(tr.y).min(bl.y).min(br.y);
		let r = tl.x.max(tr.x).max(bl.x).max(br.x);
		let b = tl.y.max(tr.y).max(bl.y).max(br.y);
		let new_bounds = Rect::new(
			old_bounds.x,
			old_bounds.y,
			(r - l).ceil() as i32,
			(b - t).ceil() as i32,
		);

		// Center canvas
		let projection = Mat3::<f32>::translation_2d(Vec2::new(
			old_bounds.w as f32 / -2f32 + 0.5,
			old_bounds.h as f32 / -2f32 + 0.5,
		));
		// Apply transformation
		let projection = (*matrix) * projection;
		// Decenter canvas
		let projection = Mat3::<f32>::translation_2d(-Vec2::new(
			new_bounds.w as f32 / -2f32 + 0.5,
			new_bounds.h as f32 / -2f32 + 0.5,
		)) * projection;
		// Invert matrix
		let projection = Into::<Mat3<f32>>::into(Into::<Mat4<f32>>::into(projection).inverted());

		let len = new_bounds.w as usize * new_bounds.h as usize;
		let mut data: Vec<u8> = Vec::with_capacity(len * stride);
		let mut mask = bitvec![Lsb0, u8; 0; len];
		let mut tmp = channel.default_pixel();

		for y in 0..(new_bounds.h as usize) {
			for x in 0..(new_bounds.w as usize) {
				let pos = projection.mul_point_2d(Vec2::new(x as f32, y as f32));
				if let Ok(()) = self.sample2d((pos.x, pos.y), sampling, &mut tmp) {
					let index = ((y as i64).wrapping_sub(new_bounds.y as i64) * new_bounds.w as i64 + (x as i64).wrapping_sub(new_bounds.x as i64)) as usize;
					mask.set(index, true);
					data.extend_from_slice(&tmp);
				}
			}
		}
		
		unsafe {
			Ok(Stencil::from_raw_parts(new_bounds, mask, channel, data))
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::*;
	use color::*;
	use image::{DynamicImage, ImageBuffer};
	use std::path::Path;
	use vek::{geom::repr_c::Rect, vec::repr_c::vec3::Vec3};

	fn load_image(path: &Path) -> Result<(Channel, u32, u32, Vec<u8>), ()> {
		match image::open(path).map_err(|_| ())? {
			DynamicImage::ImageRgb8(img) => {
				let (w, h) = img.dimensions();
				let channel = Channel::Rgb;
				let stride = channel.pixel_stride();
				let mut pixels = vec![0u8; w as usize * h as usize * stride];

				for (x, y, rgb) in img.enumerate_pixels() {
					let index = (x as u32 + w * y as u32) as usize;
					let buf = &mut pixels[(index * stride)..((index + 1) * stride)];
					let mut pixel = PixelMut::from_buffer_mut(buf, channel);
					*pixel.rgb().unwrap() = Rgb::new(rgb[0], rgb[1], rgb[2]);
				}

				Ok((channel, w, h, pixels))
			}
			DynamicImage::ImageRgba8(img) => {
				let (w, h) = img.dimensions();
				let channel = Channel::Rgba;
				let stride = channel.pixel_stride();
				let mut pixels = vec![0u8; w as usize * h as usize * stride];

				for (x, y, rgba) in img.enumerate_pixels() {
					let index = (x as u32 + w * y as u32) as usize;
					let buf = &mut pixels[(index * stride)..((index + 1) * stride)];
					let mut pixel = PixelMut::from_buffer_mut(buf, channel);
					*pixel.rgba().unwrap() =
						Rgba::new(Rgb::new(rgba[0], rgba[1], rgba[2]), rgba[3]);
				}

				Ok((channel, w, h, pixels))
			}
			_ => Err(()),
		}
	}

	fn save_pixels<D: std::ops::Index<(i32, i32), Output = [u8]>>(
		path: &Path,
		channel: Channel,
		rect: Rect<i32, i32>,
		data: D,
	) -> Result<(), ()> {
		match channel {
			Channel::Rgb => {
				let mut img = ImageBuffer::new(rect.w as u32, rect.h as u32);
				for (x, y, rgb) in img.enumerate_pixels_mut() {
					let buf = &data[(x as i32 + rect.x, y as i32 + rect.y)];
					let pixel = Pixel::from_buffer(buf, channel);
					let Rgb { red, green, blue } = pixel.rgb().unwrap();
					*rgb = image::Rgb([*red, *green, *blue]);
				}
				img.save(path).map_err(|_| ())
			}
			Channel::Rgba => {
				let mut img = ImageBuffer::new(rect.w as u32, rect.h as u32);
				for (x, y, rgba) in img.enumerate_pixels_mut() {
					let buf = &data[(x as i32 + rect.x, y as i32 + rect.y)];
					let pixel = Pixel::from_buffer(buf, channel);
					let Rgba {
						color: Rgb { red, green, blue },
						alpha,
					} = pixel.rgba().unwrap();
					*rgba = image::Rgba([*red, *green, *blue, *alpha]);
				}
				img.save(path).map_err(|_| ())
			}
			_ => Err(()),
		}
	}

	fn assert_transform_image(
		in_image: &Path,
		sampling: Sampling,
		transform: Mat3<f32>,
		out_image: &Path,
	) {
		let (channel, width, height, pixels) = load_image(in_image).unwrap();

		let canvas = Canvas::from_stencil(Stencil::from_buffer(
			Rect::new(0, 0, width as i32, height as i32),
			channel,
			pixels,
		));
		let canvas = canvas.transform(sampling, &transform).unwrap();
		let bounds = canvas.bounds();
		let transformed_pixels: Vec<_> = canvas.iter().flatten().map(|b| *b).collect();

		if let Some(_) = std::option_env!("PXLR_TEST_SAVE_IMAGE") {
			save_pixels(out_image, channel, bounds, canvas).unwrap();
		}

		let (out_channel, out_width, out_height, out_pixels) = load_image(out_image).unwrap();
		assert_eq!(channel, out_channel);
		assert_eq!(out_width, bounds.w as u32);
		assert_eq!(out_height, bounds.h as u32);
		assert_eq!(transformed_pixels, out_pixels);
	}

	#[test]
	fn transform_identity() {
		assert_transform_image(
			&Path::new("tests/character.png"),
			Sampling::Nearest,
			Mat3::identity(),
			&Path::new("tests/character-nearest-identity.png"),
		);
		assert_transform_image(
			&Path::new("tests/character.png"),
			Sampling::Bilinear,
			Mat3::identity(),
			&Path::new("tests/character-bilinear-identity.png"),
		);
	}

	#[test]
	fn transform_flip() {
		assert_transform_image(
			&Path::new("tests/character.png"),
			Sampling::Nearest,
			Mat3::scaling_3d(Vec3::new(-1., 1., 1.)),
			&Path::new("tests/character-nearest-flip-x.png"),
		);
		assert_transform_image(
			&Path::new("tests/character.png"),
			Sampling::Nearest,
			Mat3::scaling_3d(Vec3::new(1., -1., 1.)),
			&Path::new("tests/character-nearest-flip-y.png"),
		);
		assert_transform_image(
			&Path::new("tests/character.png"),
			Sampling::Nearest,
			Mat3::scaling_3d(Vec3::new(-1., -1., 1.)),
			&Path::new("tests/character-nearest-flip-xy.png"),
		);
		assert_transform_image(
			&Path::new("tests/character.png"),
			Sampling::Bilinear,
			Mat3::scaling_3d(Vec3::new(-1., 1., 1.)),
			&Path::new("tests/character-bilinear-flip-x.png"),
		);
		assert_transform_image(
			&Path::new("tests/character.png"),
			Sampling::Bilinear,
			Mat3::scaling_3d(Vec3::new(1., -1., 1.)),
			&Path::new("tests/character-bilinear-flip-y.png"),
		);
		assert_transform_image(
			&Path::new("tests/character.png"),
			Sampling::Bilinear,
			Mat3::scaling_3d(Vec3::new(-1., -1., 1.)),
			&Path::new("tests/character-bilinear-flip-xy.png"),
		);
	}

	#[test]
	fn transform_scale() {
		assert_transform_image(
			&Path::new("tests/character.png"),
			Sampling::Nearest,
			Mat3::scaling_3d(Vec3::new(2., 2., 1.)),
			&Path::new("tests/character-nearest-scale-2x.png"),
		);
		assert_transform_image(
			&Path::new("tests/character.png"),
			Sampling::Nearest,
			Mat3::scaling_3d(Vec3::new(0.5, 0.5, 1.)),
			&Path::new("tests/character-nearest-scale-half.png"),
		);
		assert_transform_image(
			&Path::new("tests/character.png"),
			Sampling::Bilinear,
			Mat3::scaling_3d(Vec3::new(2., 2., 1.)),
			&Path::new("tests/character-bilinear-scale-2x.png"),
		);
		assert_transform_image(
			&Path::new("tests/character.png"),
			Sampling::Bilinear,
			Mat3::scaling_3d(Vec3::new(0.5, 0.5, 1.)),
			&Path::new("tests/character-bilinear-scale-half.png"),
		);
	}

	#[test]
	fn transform_rotate() {
		assert_transform_image(
			&Path::new("tests/character.png"),
			Sampling::Nearest,
			Mat3::rotation_z(45. * (std::f32::consts::PI / 180.)),
			&Path::new("tests/character-nearest-rotate-45deg.png"),
		);
		assert_transform_image(
			&Path::new("tests/character.png"),
			Sampling::Nearest,
			Mat3::rotation_z(90. * (std::f32::consts::PI / 180.)),
			&Path::new("tests/character-nearest-rotate-90deg.png"),
		);
		assert_transform_image(
			&Path::new("tests/character.png"),
			Sampling::Nearest,
			Mat3::rotation_z(180. * (std::f32::consts::PI / 180.)),
			&Path::new("tests/character-nearest-rotate-180deg.png"),
		);
		assert_transform_image(
			&Path::new("tests/character.png"),
			Sampling::Bilinear,
			Mat3::rotation_z(45. * (std::f32::consts::PI / 180.)),
			&Path::new("tests/character-bilinear-rotate-45deg.png"),
		);
		assert_transform_image(
			&Path::new("tests/character.png"),
			Sampling::Bilinear,
			Mat3::rotation_z(90. * (std::f32::consts::PI / 180.)),
			&Path::new("tests/character-bilinear-rotate-90deg.png"),
		);
		assert_transform_image(
			&Path::new("tests/character.png"),
			Sampling::Bilinear,
			Mat3::rotation_z(180. * (std::f32::consts::PI / 180.)),
			&Path::new("tests/character-bilinear-rotate-180deg.png"),
		);
	}
}

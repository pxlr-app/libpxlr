use crate::{Canvas, Samplable, Sampling, Stencil};
use color::ChannelError;
#[cfg(feature = "rayon")]
use rayon::prelude::*;
use vek::{
	geom::repr_c::{Aabr, Rect},
	mat::repr_c::column_major::{Mat3, Mat4},
	vec::repr_c::{extent2::Extent2, vec2::Vec2, vec3::Vec3, vec4::Vec4},
};

pub trait Transformable {
	type Output;

	fn transform(&self, sampling: Sampling, matrix: &Mat3<f32>) -> Self::Output;
}

impl Transformable for Canvas {
	type Output = Result<Stencil, ChannelError>;

	fn transform(&self, sampling: Sampling, matrix: &Mat3<f32>) -> Result<Stencil, ChannelError> {
		let stride = self.channel.pixel_stride();

		// Calculate new bounds
		let half_size: Aabr<f32> = self.bounds().into_aabr().as_();
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
			// (l + half_size.w).floor() as i32,
			// (t + half_size.h).floor() as i32,
			0,
			0,
			(r - l).ceil() as i32,
			(b - t).ceil() as i32,
		);

		let projection = Into::<Mat3<f32>>::into(Into::<Mat4<f32>>::into(*matrix).inverted());

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
				// dbg!(x, y, pos.x, pos.y, slice);
			}
		});
		Ok(Stencil::from_buffer_mask_alpha(
			new_bounds,
			self.channel,
			data,
		))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::*;
	use color::*;
	use image::{DynamicImage, ImageBuffer};
	use std::path::Path;
	use vek::{
		geom::repr_c::Rect,
		vec::repr_c::{vec2::Vec2, vec3::Vec3},
	};

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

	#[test]
	fn transform_identity() {
		let a = Canvas::from_stencil(Stencil::from_buffer_mask_alpha(
			Rect::new(0, 0, 2, 2),
			Channel::Lumaa,
			vec![5, 255, 10, 255, 15, 255, 20, 255],
		));
		// let b = a
		// 	.transform(Sampling::Nearest, &Mat3::rotation_z(0.785398))
		// 	.unwrap();
		let b = a.transform(Sampling::Nearest, &Mat3::identity()).unwrap();
		let pixels: Vec<_> = b
			.iter()
			.map(|(_, _, data)| data.to_vec())
			.flatten()
			.collect();
		assert_eq!(pixels, vec![5, 255, 10, 255, 15, 255, 20, 255]);

		let (channel, width, height, pixels) =
			load_image(&Path::new("tests/character.png")).unwrap();

		let c = Canvas::from_stencil(Stencil::from_buffer(
			Rect::new(0, 0, width as i32, height as i32),
			channel,
			pixels,
		));
		let d = c.transform(Sampling::Nearest, &Mat3::identity()).unwrap();
		let d = Canvas::from_stencil(d);
		save_pixels(
			&Path::new("tests/transform-nearest-identity.png"),
			channel,
			d.bounds(),
			d,
		)
		.unwrap();
	}

	#[test]
	fn transform_flip() {
		let a = Canvas::from_stencil(Stencil::from_buffer_mask_alpha(
			Rect::new(0, 0, 2, 2),
			Channel::Lumaa,
			vec![5, 255, 10, 255, 15, 255, 20, 255],
		));
		let b = a
			.transform(
				Sampling::Nearest,
				&Mat3::translation_2d(Vec2::new(-0.5, 0.))
					.scaled_3d(Vec3::new(-1., 1., 1.))
					.translated_2d(Vec2::new(0.5, 0.)),
			)
			.unwrap();
		let pixels: Vec<_> = b
			.iter()
			.map(|(_, _, data)| data.to_vec())
			.flatten()
			.collect();
		assert_eq!(pixels, vec![10, 255, 5, 255, 20, 255, 15, 255]);

		let c = a
			.transform(
				Sampling::Nearest,
				&Mat3::translation_2d(Vec2::new(0., -0.5))
					.scaled_3d(Vec3::new(1., -1., 1.))
					.translated_2d(Vec2::new(0., 0.5)),
			)
			.unwrap();
		let pixels: Vec<_> = c
			.iter()
			.map(|(_, _, data)| data.to_vec())
			.flatten()
			.collect();
		assert_eq!(pixels, vec![15, 255, 20, 255, 5, 255, 10, 255]);

		{
			let (channel, width, height, pixels) =
				load_image(&Path::new("tests/character.png")).unwrap();

			let c = Canvas::from_stencil(Stencil::from_buffer(
				Rect::new(0, 0, width as i32, height as i32),
				channel,
				pixels,
			));
			let d = c
				.transform(
					Sampling::Nearest,
					&Mat3::translation_2d(Vec2::new(width as f32 / -2. + 0.5, 0.))
						.scaled_3d(Vec3::new(-1., 1., 1.))
						.translated_2d(Vec2::new(width as f32 / 2. - 0.5, 0.)),
				)
				.unwrap();
			let d = Canvas::from_stencil(d);
			save_pixels(
				&Path::new("tests/transform-nearest-flip-x.png"),
				channel,
				d.bounds(),
				d,
			)
			.unwrap();
		}

		{
			let (channel, width, height, pixels) =
				load_image(&Path::new("tests/character.png")).unwrap();

			let c = Canvas::from_stencil(Stencil::from_buffer(
				Rect::new(0, 0, width as i32, height as i32),
				channel,
				pixels,
			));
			let d = c
				.transform(
					Sampling::Nearest,
					&Mat3::translation_2d(Vec2::new(0., height as f32 / -2. + 0.5))
						.scaled_3d(Vec3::new(1., -1., 1.))
						.translated_2d(Vec2::new(0., height as f32 / 2. - 0.5)),
				)
				.unwrap();
			let d = Canvas::from_stencil(d);
			save_pixels(
				&Path::new("tests/transform-nearest-flip-y.png"),
				channel,
				d.bounds(),
				d,
			)
			.unwrap();
		}

		{
			let (channel, width, height, pixels) =
				load_image(&Path::new("tests/character.png")).unwrap();

			let c = Canvas::from_stencil(Stencil::from_buffer(
				Rect::new(0, 0, width as i32, height as i32),
				channel,
				pixels,
			));
			let d = c
				.transform(
					Sampling::Nearest,
					&Mat3::translation_2d(Vec2::new(
						width as f32 / -2. + 0.5,
						height as f32 / -2. + 0.5,
					))
					.scaled_3d(Vec3::new(-1., -1., 1.))
					.translated_2d(Vec2::new(width as f32 / 2. - 0.5, height as f32 / 2. - 0.5)),
				)
				.unwrap();
			let d = Canvas::from_stencil(d);
			save_pixels(
				&Path::new("tests/transform-nearest-flip-xy.png"),
				channel,
				d.bounds(),
				d,
			)
			.unwrap();
		}
	}

	#[test]
	fn transform_scale() {
		let a = Canvas::from_stencil(Stencil::from_buffer_mask_alpha(
			Rect::new(0, 0, 2, 2),
			Channel::Lumaa,
			vec![5, 255, 10, 255, 15, 255, 20, 255],
		));
		let b = a
			.transform(
				Sampling::Nearest,
				&Mat3::translation_2d(Vec2::new(-0.5, -0.5))
					.scaled_3d(Vec3::new(2., 2., 1.))
					.translated_2d(Vec2::new(0.5, 0.5)),
			)
			.unwrap();
		let pixels: Vec<_> = b
			.iter()
			.map(|(_, _, data)| data.to_vec())
			.flatten()
			.collect();
		assert_eq!(
			pixels,
			vec![
				5, 255, 10, 255, 10, 255, 10, 255, 15, 255, 20, 255, 20, 255, 20, 255, 15, 255, 20,
				255, 20, 255, 20, 255, 15, 255, 20, 255, 20, 255, 20, 255
			]
		);

		{
			let (channel, width, height, pixels) =
				load_image(&Path::new("tests/character.png")).unwrap();

			let c = Canvas::from_stencil(Stencil::from_buffer(
				Rect::new(0, 0, width as i32, height as i32),
				channel,
				pixels,
			));
			let d = c
				.transform(
					Sampling::Nearest,
					&Mat3::translation_2d(Vec2::new(
						width as f32 / -2. + 0.5,
						height as f32 / -2. + 0.5,
					))
					.scaled_3d(Vec3::new(2., 2., 1.))
					.translated_2d(Vec2::new(width as f32 - 0.5, height as f32 - 0.5)),
				)
				.unwrap();
			let d = Canvas::from_stencil(d);
			save_pixels(
				&Path::new("tests/transform-nearest-scale2x.png"),
				channel,
				d.bounds(),
				d,
			)
			.unwrap();
		}
	}
}

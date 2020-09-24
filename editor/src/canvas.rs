use crate::{blend::Blend, interpolation::Interpolation};
use document::prelude::*;
#[cfg(feature = "rayon")]
use rayon::prelude::*;

// https://github.com/image-rs/imageproc/blob/master/src/geometric_transformations.rs

pub fn transform_into<'src, 'dst>(
	transform: &Mat3<f32>,
	interpolation: &Interpolation,
	size: &Extent2<u32>,
	channels: Channel,
	src: &'src Pixels,
	dst: &'dst mut Pixels,
) {
	use math::{Mat4, Vec4};

	let stride = channels.size();
	assert!(src.len() == size.w as usize * size.h as usize * stride);
	assert!(dst.len() == size.w as usize * size.h as usize * stride);

	let pitch = size.w as usize * stride;
	let mut projection: Mat4<f32> = (*transform).into();
	projection.invert();

	#[cfg(feature = "rayon")]
	let chunks = dst.par_chunks_mut(pitch);
	#[cfg(not(feature = "rayon"))]
	let chunks = dst.chunks_mut(pitch);

	chunks.enumerate().for_each(|(y, row)| {
		for (x, slice) in row.chunks_mut(stride).enumerate() {
			let pos = projection * Vec4::new(x as f32, y as f32, 1., 1.);
			interpolation.interpolate_into(&pos.xy(), channels, &size, src, slice);
		}
	});
}

// pub fn blend_into<
// 	'srca,
// 	'srcb,
// 	'dest,
// >(
// 	_size: Extent2<u32>,
// 	_blend_mode: &Blend,
// 	_channels: Channel,
// 	_source_a: &'srca Pixels,
// 	_source_b: &'srcb Pixels,
// 	_dest: &'dst mut Pixel,
// ) {
// }

#[cfg(test)]
mod tests {
	use super::*;
	use image::{DynamicImage, ImageBuffer};
	use math::Vec3;
	use std::path::Path;

	#[test]
	fn test_transform() {
		test_interpolation("nearest".into(), Interpolation::Nearest);
		test_interpolation("bilinear".into(), Interpolation::Bilinear);
		// test_interpolation("bicubic".into(), Interpolation::Bicubic);
	}

	fn test_interpolation(suffix: String, inter: Interpolation) {
		let (channel, width, height, pixels) =
			load_pixels(&Path::new("tests/character.png")).unwrap();
		let size = Extent2::new(width, height);

		let mut out = vec![0u8; pixels.len()];
		let matrix = Mat3::translation_2d(Vec2::new(width as f32 / 2f32, height as f32 / 2f32));
		transform_into(&matrix, &inter, &size, channel, &pixels[..], &mut out[..]);
		#[rustfmt::skip]
		save_pixels(&Path::new(&format!("tests/transform-{}-translated.png", suffix)), channel, width, height, &out[..]).unwrap();

		let mut out = vec![0u8; pixels.len()];
		let matrix = Mat3::translation_2d(Vec2::new(width as f32 / -2f32 + 0.5, 0.))
			.scaled_3d(Vec3::new(-1., 1., 1.))
			.translated_2d(Vec2::new(width as f32 / 2f32 - 0.5, 0.));
		transform_into(&matrix, &inter, &size, channel, &pixels[..], &mut out[..]);
		#[rustfmt::skip]
		save_pixels(&Path::new(&format!("tests/transform-{}-fliped.png", suffix)), channel, width, height, &out[..]).unwrap();

		let mut out = vec![0u8; pixels.len()];
		let matrix = Mat3::translation_2d(Vec2::new(width as f32 / -2f32, height as f32 / -2f32))
			.scaled_3d(Vec3::new(0.5, 0.5, 1.))
			.translated_2d(Vec2::new(width as f32 / 2f32, height as f32 / 2f32));
		transform_into(&matrix, &inter, &size, channel, &pixels[..], &mut out[..]);
		#[rustfmt::skip]
		save_pixels(&Path::new(&format!("tests/transform-{}-0.5x.png", suffix)), channel, width, height, &out[..]).unwrap();
	}

	fn load_pixels(path: &Path) -> Result<(Channel, u32, u32, Vec<u8>), ()> {
		match image::open(path).map_err(|_| ())? {
			DynamicImage::ImageRgb8(img) => {
				let (w, h) = img.dimensions();
				let channel = Channel::RGB;
				let len = channel.size();
				let mut pixels = vec![0u8; w as usize * h as usize * len];

				for (x, y, pixel) in img.enumerate_pixels() {
					let index = (x as u32 + w * y as u32) as usize;
					let buf = &mut pixels[(index * len)..((index + 1) * len)];
					unsafe {
						*channel.unsafe_rgb_mut(buf) = RGB::new(pixel[0], pixel[1], pixel[2]);
					}
				}

				Ok((channel, w, h, pixels))
			}
			DynamicImage::ImageRgba8(img) => {
				let (w, h) = img.dimensions();
				let channel = Channel::RGB | Channel::A;
				let len = channel.size();
				let mut pixels = vec![0u8; w as usize * h as usize * len];

				for (x, y, pixel) in img.enumerate_pixels() {
					let index = (x as u32 + w * y as u32) as usize;
					let buf = &mut pixels[(index * len)..((index + 1) * len)];
					unsafe {
						*channel.unsafe_rgb_mut(buf) = RGB::new(pixel[0], pixel[1], pixel[2]);
						*channel.unsafe_a_mut(buf) = A::new(pixel[3]);
					}
				}

				Ok((channel, w, h, pixels))
			}
			_ => Err(()),
		}
	}

	fn save_pixels(
		path: &Path,
		channel: Channel,
		width: u32,
		height: u32,
		data: &[u8],
	) -> Result<(), ()> {
		let len = channel.size();
		if channel & (Channel::RGB | Channel::A) == Channel::RGB | Channel::A {
			let mut img = ImageBuffer::new(width, height);
			for (x, y, pixel) in img.enumerate_pixels_mut() {
				let index = (x as u32 + width * y as u32) as usize;
				let buf = &data[(index * len)..((index + 1) * len)];
				let RGB { r, g, b } = channel.rgb(buf).expect("No RGB channel");
				let A { a } = channel.a(buf).expect("No alpha channel");
				*pixel = image::Rgba([*r, *g, *b, *a]);
			}
			img.save(path).map_err(|_| ())
		} else if channel & Channel::RGB == Channel::RGB {
			let mut img = ImageBuffer::new(width, height);
			for (x, y, pixel) in img.enumerate_pixels_mut() {
				let index = (x as u32 + width * y as u32) as usize;
				let buf = &data[(index * len)..((index + 1) * len)];
				let RGB { r, g, b } = channel.rgb(buf).expect("No RGB channel");
				*pixel = image::Rgb([*r, *g, *b]);
			}
			img.save(path).map_err(|_| ())
		} else {
			Err(())
		}
	}
}

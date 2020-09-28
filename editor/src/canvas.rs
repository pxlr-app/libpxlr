use crate::{
	blend::{Blend, Compose},
	interpolation::Interpolation,
};
use document::prelude::*;
#[cfg(feature = "rayon")]
use rayon::prelude::*;

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

fn blend_into_inner<'frt, 'bck, 'dst>(
	blend_mode: Blend,
	compose_op: Compose,
	channels: Channel,
	frt: &'frt Pixel,
	bck: &'bck Pixel,
	dst: &'dst mut Pixel,
) {
	let stride = channels.size();
	assert!(frt.len() == stride);
	assert!(bck.len() == stride);
	assert!(dst.len() == stride);

	// Without alpha channel, no blend occur
	if channels & Channel::A != Channel::A {
		dst.copy_from_slice(frt);
	} else {
		if channels & Channel::I == Channel::I {
			unsafe {
				*channels.unsafe_i_mut(dst) = *channels.unsafe_i(frt);
			}
		}
		if channels & Channel::UV == Channel::UV {
			unsafe {
				*channels.unsafe_uv_mut(dst) = *channels.unsafe_uv(frt);
			}
		}
		if channels & Channel::XYZ == Channel::XYZ {
			unsafe {
				*channels.unsafe_xyz_mut(dst) = *channels.unsafe_xyz(frt);
			}
		}

		let fa = unsafe { channels.unsafe_a(frt).a as f32 / 255f32 };
		let ba = unsafe { channels.unsafe_a(bck).a as f32 / 255f32 };
		let oa = fa + ba * (1. - fa);

		unsafe {
			*channels.unsafe_a_mut(dst) = A::new((oa * 255_f32).round() as u8);
		}

		if channels & Channel::RGB == Channel::RGB {
			let (fr, fg, fb) = unsafe {
				let RGB { r, g, b } = channels.unsafe_rgb(frt);
				(*r as f32 / 255f32, *g as f32 / 255f32, *b as f32 / 255f32)
			};
			let (br, bg, bb) = unsafe {
				let RGB { r, g, b } = channels.unsafe_rgb(bck);
				(*r as f32 / 255f32, *g as f32 / 255f32, *b as f32 / 255f32)
			};

			#[allow(non_snake_case)]
			let (Fa, Fb) = compose_op.compose(fa, ba);

			// Apply blend
			let or = (1. - ba) * fr + ba * blend_mode.blend(br, fr);
			let og = (1. - ba) * fg + ba * blend_mode.blend(bg, fg);
			let ob = (1. - ba) * fb + ba * blend_mode.blend(bb, fb);
			// Compose
			let or = fa * Fa * or + ba * Fb * br;
			let og = fa * Fa * og + ba * Fb * bg;
			let ob = fa * Fa * ob + ba * Fb * bb;

			unsafe {
				*channels.unsafe_rgb_mut(dst) = RGB::new(
					(or * 255_f32).round() as u8,
					(og * 255_f32).round() as u8,
					(ob * 255_f32).round() as u8,
				);
			}
		}
	}
}

pub fn blend_into<'frt, 'bck, 'dst>(
	blend_mode: Blend,
	compose_op: Compose,
	size: &Extent2<u32>,
	channels: Channel,
	frt: &'frt Pixels,
	bck: &'bck Pixels,
	dst: &'dst mut Pixels,
) {
	let stride = channels.size();
	assert!(frt.len() == size.w as usize * size.h as usize * stride);
	assert!(bck.len() == size.w as usize * size.h as usize * stride);
	assert!(dst.len() == size.w as usize * size.h as usize * stride);

	let pitch = size.w as usize * stride;

	#[cfg(feature = "rayon")]
	let chunks = dst.par_chunks_mut(pitch);
	#[cfg(not(feature = "rayon"))]
	let chunks = dst.chunks_mut(pitch);

	chunks.enumerate().for_each(|(y, row)| {
		for (x, slice) in row.chunks_mut(stride).enumerate() {
			let index = x + y * size.w as usize;
			blend_into_inner(
				blend_mode,
				compose_op,
				channels,
				&frt[(index * stride)..((index + 1) * stride)],
				&bck[(index * stride)..((index + 1) * stride)],
				slice,
			);
		}
	});
}

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

	#[test]
	fn test_normal() {
		let (channel, width, height, red) = load_pixels(&Path::new("tests/red.png")).unwrap();
		let (_, _, _, green) = load_pixels(&Path::new("tests/green.png")).unwrap();
		let (_, _, _, blue) = load_pixels(&Path::new("tests/blue.png")).unwrap();
		let size = Extent2::new(width, height);
		let mut rg = vec![0u8; red.len()];
		let mut out = vec![0u8; red.len()];
		let blend_mode = Blend::Normal;
		let compose_op = Compose::DestinationOver;
		#[rustfmt::skip]
		blend_into(blend_mode, compose_op, &size, channel, &red[..], &green[..], &mut rg[..]);
		#[rustfmt::skip]
		blend_into(blend_mode, compose_op, &size, channel, &rg[..], &blue[..], &mut out[..]);
		save_pixels(
			&Path::new("tests/blended-normal.png"),
			channel,
			width,
			height,
			&out[..],
		)
		.unwrap();
	}

	#[test]
	fn test_screen() {
		let (channel, width, height, red) = load_pixels(&Path::new("tests/red.png")).unwrap();
		let (_, _, _, green) = load_pixels(&Path::new("tests/green.png")).unwrap();
		let (_, _, _, blue) = load_pixels(&Path::new("tests/blue.png")).unwrap();
		let size = Extent2::new(width, height);
		let mut rg = vec![0u8; red.len()];
		let mut out = vec![0u8; red.len()];
		let blend_mode = Blend::Screen;
		let compose_op = Compose::Lighter;
		#[rustfmt::skip]
		blend_into(blend_mode, compose_op, &size, channel, &red[..], &green[..], &mut rg[..]);
		#[rustfmt::skip]
		blend_into(blend_mode, compose_op, &size, channel, &rg[..], &blue[..], &mut out[..]);
		save_pixels(
			&Path::new("tests/blended-screen.png"),
			channel,
			width,
			height,
			&out[..],
		)
		.unwrap();
	}
}

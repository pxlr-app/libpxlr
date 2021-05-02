mod braille;
mod stencil;
use color::*;

#[cfg(test)]
mod tests {
	use super::*;
	use image::{DynamicImage, ImageBuffer};
	use std::path::Path;

	#[allow(dead_code)]
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

	#[allow(dead_code)]
	fn save_pixels(
		path: &Path,
		channel: Channel,
		width: u32,
		height: u32,
		data: &[u8],
	) -> Result<(), ()> {
		let len = channel.pixel_stride();
		match channel {
			Channel::Rgb => {
				let mut img = ImageBuffer::new(width, height);
				for (x, y, rgb) in img.enumerate_pixels_mut() {
					let index = (x as u32 + width * y as u32) as usize;
					let buf = &data[(index * len)..((index + 1) * len)];
					let pixel = Pixel::from_buffer(buf, channel);
					let Rgb { red, green, blue } = pixel.rgb().unwrap();
					*rgb = image::Rgb([*red, *green, *blue]);
				}
				img.save(path).map_err(|_| ())
			}
			Channel::Rgba => {
				let mut img = ImageBuffer::new(width, height);
				for (x, y, rgba) in img.enumerate_pixels_mut() {
					let index = (x as u32 + width * y as u32) as usize;
					let buf = &data[(index * len)..((index + 1) * len)];
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
}

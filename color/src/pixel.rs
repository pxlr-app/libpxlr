use crate::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Pixel<'data> {
	pub data: &'data [u8],
	pub channel: Channel,
}

#[derive(Debug, Eq, PartialEq)]
pub struct PixelMut<'data> {
	pub data: &'data mut [u8],
	pub channel: Channel,
}

impl<'data> Pixel<'data> {
	/// From buffer data and Channel
	pub fn from_buffer(data: &'data [u8], channel: Channel) -> Self {
		assert_eq!(data.len(), channel.pixel_stride());
		Self { data, channel }
	}

	/// Retrieve Luma color from Channel
	///
	/// ```
	/// use color::*;
	/// let color = Luma::new(1);
	/// let pixel = Pixel::from_buffer(color.to_slice(), Channel::Luma);
	/// assert!(pixel.luma().is_ok());
	/// assert_eq!(pixel.luma().unwrap(), &color);
	/// ```
	pub fn luma(&self) -> Result<&Luma, ChannelError> {
		let offset = self.channel.offset_of(Channel::Luma)?;
		Ok(Luma::from_slice(&self.data[offset..offset + Luma::SIZE]))
	}

	/// Retrieve Lumaa color from Channel
	///
	/// ```
	/// use color::*;
	/// let color = Lumaa::new(Luma::new(1), 128);
	/// let pixel = Pixel::from_buffer(color.to_slice(), Channel::Lumaa);
	/// assert!(pixel.lumaa().is_ok());
	/// assert_eq!(pixel.lumaa().unwrap(), &color);
	/// ```
	pub fn lumaa(&self) -> Result<&Lumaa, ChannelError> {
		let offset = self.channel.offset_of(Channel::Lumaa)?;
		Ok(Lumaa::from_slice(&self.data[offset..offset + Lumaa::SIZE]))
	}

	/// Retrieve Rgb color from Channel
	///
	/// ```
	/// use color::*;
	/// let color = Rgb::new(32, 64, 96);
	/// let pixel = Pixel::from_buffer(color.to_slice(), Channel::Rgb);
	/// assert!(pixel.rgb().is_ok());
	/// assert_eq!(pixel.rgb().unwrap(), &color);
	/// ```
	pub fn rgb(&self) -> Result<&Rgb, ChannelError> {
		let offset = self.channel.offset_of(Channel::Rgb)?;
		Ok(Rgb::from_slice(&self.data[offset..offset + Rgb::SIZE]))
	}

	/// Retrieve Rgba color from Channel
	///
	/// ```
	/// use color::*;
	/// let color = Rgba::new(Rgb::new(32, 64, 96), 128);
	/// let pixel = Pixel::from_buffer(color.to_slice(), Channel::Rgba);
	/// assert!(pixel.rgba().is_ok());
	/// assert_eq!(pixel.rgba().unwrap(), &color);
	/// ```
	pub fn rgba(&self) -> Result<&Rgba, ChannelError> {
		let offset = self.channel.offset_of(Channel::Rgba)?;
		Ok(Rgba::from_slice(&self.data[offset..offset + Rgba::SIZE]))
	}

	/// Retrieve Uv color from Channel
	///
	/// ```
	/// use color::*;
	/// let color = Uv::new(0.25, 0.75);
	/// let pixel = Pixel::from_buffer(color.to_slice(), Channel::Uv);
	/// assert!(pixel.uv().is_ok());
	/// assert_eq!(pixel.uv().unwrap(), &color);
	/// ```
	pub fn uv(&self) -> Result<&Uv, ChannelError> {
		let offset = self.channel.offset_of(Channel::Uv)?;
		Ok(Uv::from_slice(&self.data[offset..offset + Uv::SIZE]))
	}

	/// Retrieve Normal color from Channel
	///
	/// ```
	/// use color::*;
	/// let color = Normal::new(0.25, 0.75, 0.33);
	/// let pixel = Pixel::from_buffer(color.to_slice(), Channel::Normal);
	/// assert!(pixel.normal().is_ok());
	/// assert_eq!(pixel.normal().unwrap(), &color);
	/// ```
	pub fn normal(&self) -> Result<&Normal, ChannelError> {
		let offset = self.channel.offset_of(Channel::Normal)?;
		Ok(Normal::from_slice(
			&self.data[offset..offset + Normal::SIZE],
		))
	}
}

impl<'data> PixelMut<'data> {
	/// From buffer data and Channel
	pub fn from_buffer_mut(data: &'data mut [u8], channel: Channel) -> Self {
		assert_eq!(data.len(), channel.pixel_stride());
		Self { data, channel }
	}

	/// Retrieve Luma color from Channel
	///
	/// ```
	/// use color::*;
	/// let mut color = Luma::new(1);
	/// let mut pixel = PixelMut::from_buffer_mut(color.to_slice_mut(), Channel::Luma);
	/// assert!(pixel.luma().is_ok());
	/// assert_eq!(pixel.luma().unwrap(), &Luma::new(1));
	/// ```
	pub fn luma(&mut self) -> Result<&mut Luma, ChannelError> {
		let offset = self.channel.offset_of(Channel::Luma)?;
		Ok(Luma::from_slice_mut(
			&mut self.data[offset..offset + Luma::SIZE],
		))
	}

	/// Retrieve Lumaa color from Channel
	///
	/// ```
	/// use color::*;
	/// let mut color = Lumaa::new(Luma::new(1), 128);
	/// let mut pixel = PixelMut::from_buffer_mut(color.to_slice_mut(), Channel::Lumaa);
	/// assert!(pixel.lumaa().is_ok());
	/// assert_eq!(pixel.lumaa().unwrap(), &Lumaa::new(Luma::new(1), 128));
	/// ```
	pub fn lumaa(&mut self) -> Result<&mut Lumaa, ChannelError> {
		let offset = self.channel.offset_of(Channel::Lumaa)?;
		Ok(Lumaa::from_slice_mut(
			&mut self.data[offset..offset + Lumaa::SIZE],
		))
	}

	/// Retrieve Rgb color from Channel
	///
	/// ```
	/// use color::*;
	/// let mut color = Rgb::new(32, 64, 96);
	/// let mut pixel = PixelMut::from_buffer_mut(color.to_slice_mut(), Channel::Rgb);
	/// assert!(pixel.rgb().is_ok());
	/// assert_eq!(pixel.rgb().unwrap(), &Rgb::new(32, 64, 96));
	/// ```
	pub fn rgb(&mut self) -> Result<&mut Rgb, ChannelError> {
		let offset = self.channel.offset_of(Channel::Rgb)?;
		Ok(Rgb::from_slice_mut(
			&mut self.data[offset..offset + Rgb::SIZE],
		))
	}

	/// Retrieve Rgba color from Channel
	///
	/// ```
	/// use color::*;
	/// let mut color = Rgba::new(Rgb::new(32, 64, 96), 128);
	/// let mut pixel = PixelMut::from_buffer_mut(color.to_slice_mut(), Channel::Rgba);
	/// assert!(pixel.rgba().is_ok());
	/// assert_eq!(pixel.rgba().unwrap(), &Rgba::new(Rgb::new(32, 64, 96), 128));
	/// ```
	pub fn rgba(&mut self) -> Result<&mut Rgba, ChannelError> {
		let offset = self.channel.offset_of(Channel::Rgba)?;
		Ok(Rgba::from_slice_mut(
			&mut self.data[offset..offset + Rgba::SIZE],
		))
	}

	/// Retrieve Uv color from Channel
	///
	/// ```
	/// use color::*;
	/// let mut color = Uv::new(0.25, 0.75);
	/// let mut pixel = PixelMut::from_buffer_mut(color.to_slice_mut(), Channel::Uv);
	/// assert!(pixel.uv().is_ok());
	/// assert_eq!(pixel.uv().unwrap(), &Uv::new(0.25, 0.75));
	/// ```
	pub fn uv(&mut self) -> Result<&mut Uv, ChannelError> {
		let offset = self.channel.offset_of(Channel::Uv)?;
		Ok(Uv::from_slice_mut(
			&mut self.data[offset..offset + Uv::SIZE],
		))
	}

	/// Retrieve Normal color from Channel
	///
	/// ```
	/// use color::*;
	/// let mut color = Normal::new(0.25, 0.75, 0.33);
	/// let mut pixel = PixelMut::from_buffer_mut(color.to_slice_mut(), Channel::Normal);
	/// assert!(pixel.normal().is_ok());
	/// assert_eq!(pixel.normal().unwrap(), &Normal::new(0.25, 0.75, 0.33));
	/// ```
	pub fn normal(&mut self) -> Result<&mut Normal, ChannelError> {
		let offset = self.channel.offset_of(Channel::Normal)?;
		Ok(Normal::from_slice_mut(
			&mut self.data[offset..offset + Normal::SIZE],
		))
	}

	/// Blend pixel together
	pub fn blend<'frt, 'bck>(
		&mut self,
		blend_mode: Blending,
		compose_op: Compositing,
		frt: &'frt Pixel,
		bck: &'bck Pixel,
	) -> Result<(), ChannelError> {
		// TODO allow blending between color with conversion?
		if self.channel != frt.channel {
			return Err(ChannelError::Mismatch(self.channel, frt.channel));
		}
		if frt.channel != bck.channel {
			return Err(ChannelError::Mismatch(frt.channel, bck.channel));
		}

		fn blend_normal<'frt, 'bck>(
			compose_op: Compositing,
			frt: &'frt Pixel,
			bck: &'bck Pixel,
			out: &mut PixelMut,
		) {
			let f = frt.normal().unwrap();
			let b = bck.normal().unwrap();

			#[allow(non_snake_case)]
			let (Fa, Fb) = compose_op.compose(1., 1.);

			// Compose
			let rx = f.x * Fa + b.x * Fb;
			let ry = f.y * Fa + b.y * Fb;
			let rz = f.z * Fa + b.z * Fb;

			*out.normal().unwrap() = Normal::new(rx, ry, rz);
		}

		match self.channel {
			Channel::Luma | Channel::LumaNormal => {
				let f = frt.luma().unwrap();
				let b = bck.luma().unwrap();

				let fl = f.luma as f32 / 255.;
				let bl = b.luma as f32 / 255.;

				#[allow(non_snake_case)]
				let (Fa, Fb) = compose_op.compose(1., 1.);

				// Compose
				let rl = fl * Fa + bl * Fb;

				*self.luma().unwrap() = Luma::new((rl * 255.).round() as u8);

				if let Channel::LumaNormal = self.channel {
					blend_normal(compose_op, frt, bck, self);
				}
			}
			Channel::Lumaa | Channel::LumaaNormal => {
				let f = frt.lumaa().unwrap();
				let b = bck.lumaa().unwrap();

				let fl = f.color.luma as f32 / 255.;
				let fa = f.alpha as f32 / 255.;
				let bl = b.color.luma as f32 / 255.;
				let ba = b.alpha as f32 / 255.;

				#[allow(non_snake_case)]
				let (Fa, Fb) = compose_op.compose(fa, ba);

				// Compose
				let rl = fl * Fa + bl * Fb;
				let ra = fa + ba * (1. - fa);

				*self.lumaa().unwrap() = Lumaa::new(
					Luma::new((rl * 255.).round() as u8),
					(ra * 255.).round() as u8,
				);

				if let Channel::LumaaNormal = self.channel {
					blend_normal(compose_op, frt, bck, self);
				}
			}
			Channel::Rgb | Channel::RgbNormal => {
				let f = frt.rgb().unwrap();
				let b = bck.rgb().unwrap();

				let fr = f.red as f32 / 255.;
				let fg = f.green as f32 / 255.;
				let fb = f.blue as f32 / 255.;
				let br = b.red as f32 / 255.;
				let bg = b.green as f32 / 255.;
				let bb = b.blue as f32 / 255.;

				#[allow(non_snake_case)]
				let (Fa, Fb) = compose_op.compose(1., 1.);

				// Compose
				let rr = fr * Fa + br * Fb;
				let rg = fg * Fa + bg * Fb;
				let rb = fb * Fa + bb * Fb;

				*self.rgb().unwrap() = Rgb::new(
					(rr * 255.).round() as u8,
					(rg * 255.).round() as u8,
					(rb * 255.).round() as u8,
				);

				if let Channel::RgbNormal = self.channel {
					blend_normal(compose_op, frt, bck, self);
				}
			}
			Channel::Rgba | Channel::RgbaNormal => {
				let f = frt.rgba().unwrap();
				let b = bck.rgba().unwrap();

				let fr = f.color.red as f32 / 255.;
				let fg = f.color.green as f32 / 255.;
				let fb = f.color.blue as f32 / 255.;
				let fa = f.alpha as f32 / 255.;
				let br = b.color.red as f32 / 255.;
				let bg = b.color.green as f32 / 255.;
				let bb = b.color.blue as f32 / 255.;
				let ba = b.alpha as f32 / 255.;

				#[allow(non_snake_case)]
				let (Fa, Fb) = compose_op.compose(fa, ba);

				// Apply blend
				let or = (1. - ba) * fr + ba * blend_mode.blend(br, fr);
				let og = (1. - ba) * fg + ba * blend_mode.blend(bg, fg);
				let ob = (1. - ba) * fb + ba * blend_mode.blend(bb, fb);
				// Compose
				let rr = fa * Fa * or + ba * Fb * br;
				let rg = fa * Fa * og + ba * Fb * bg;
				let rb = fa * Fa * ob + ba * Fb * bb;
				let ra = fa + ba * (1. - fa);

				*self.rgba().unwrap() = Rgba::new(
					Rgb::new(
						(rr * 255.).round() as u8,
						(rg * 255.).round() as u8,
						(rb * 255.).round() as u8,
					),
					(ra * 255.).round() as u8,
				);

				if let Channel::RgbaNormal = self.channel {
					blend_normal(compose_op, frt, bck, self);
				}
			}
			Channel::Uv | Channel::UvNormal => {
				let f = frt.uv().unwrap();
				let b = bck.uv().unwrap();

				#[allow(non_snake_case)]
				let (Fa, Fb) = compose_op.compose(1., 1.);

				// Compose
				let ru = f.u * Fa + b.u * Fb;
				let rv = f.v * Fa + b.v * Fb;

				*self.uv().unwrap() = Uv::new(ru, rv);

				if let Channel::Uv = self.channel {
					blend_normal(compose_op, frt, bck, self);
				}
			}
			Channel::Normal => {
				blend_normal(compose_op, frt, bck, self);
			}
		};

		Ok(())
	}

	/// As immutable
	pub fn as_immutable(&self) -> Pixel {
		Pixel {
			data: self.data,
			channel: self.channel,
		}
	}

	/// Lerp from Pixel to Pixel
	pub fn lerp<'from, 'to>(
		&mut self,
		from: &'from Pixel,
		to: &'to Pixel,
		factor: f32,
	) -> Result<(), ChannelError> {
		if self.channel != from.channel {
			return Err(ChannelError::Mismatch(self.channel, from.channel));
		}
		if from.channel != to.channel {
			return Err(ChannelError::Mismatch(from.channel, to.channel));
		}

		use vek::ops::Lerp;
		if let (Ok(dst), Ok(from), Ok(to)) = (self.luma(), from.luma(), to.luma()) {
			*dst = Lerp::lerp(*from, *to, factor);
		} else if let (Ok(dst), Ok(from), Ok(to)) = (self.lumaa(), from.lumaa(), to.lumaa()) {
			*dst = Lerp::lerp(*from, *to, factor);
		} else if let (Ok(dst), Ok(from), Ok(to)) = (self.rgb(), from.rgb(), to.rgb()) {
			*dst = Lerp::lerp(*from, *to, factor);
		} else if let (Ok(dst), Ok(from), Ok(to)) = (self.rgba(), from.rgba(), to.rgba()) {
			*dst = Lerp::lerp(*from, *to, factor);
		} else if let (Ok(dst), Ok(from), Ok(to)) = (self.uv(), from.uv(), to.uv()) {
			*dst = Lerp::lerp(*from, *to, factor);
		} else if let (Ok(dst), Ok(from), Ok(to)) = (self.normal(), from.normal(), to.normal()) {
			*dst = Lerp::lerp(*from, *to, factor);
		}

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn pixel_get_color_within_channel() {
		let buffer = Channel::LumaNormal.default_pixel();
		let pixel = Pixel::from_buffer(&buffer, Channel::LumaNormal);
		assert!(pixel.luma().is_ok());
		assert!(pixel.normal().is_ok());
		assert_eq!(pixel.luma().unwrap(), &Luma::default());
		assert_eq!(pixel.normal().unwrap(), &Normal::default());

		let buffer = Channel::LumaaNormal.default_pixel();
		let pixel = Pixel::from_buffer(&buffer, Channel::LumaaNormal);
		assert!(pixel.lumaa().is_ok());
		assert!(pixel.normal().is_ok());
		assert_eq!(pixel.lumaa().unwrap(), &Lumaa::default());
		assert_eq!(pixel.normal().unwrap(), &Normal::default());

		let buffer = Channel::RgbNormal.default_pixel();
		let pixel = Pixel::from_buffer(&buffer, Channel::RgbNormal);
		assert!(pixel.rgb().is_ok());
		assert!(pixel.normal().is_ok());
		assert_eq!(pixel.rgb().unwrap(), &Rgb::default());
		assert_eq!(pixel.normal().unwrap(), &Normal::default());

		let buffer = Channel::RgbaNormal.default_pixel();
		let pixel = Pixel::from_buffer(&buffer, Channel::RgbaNormal);
		assert!(pixel.rgba().is_ok());
		assert!(pixel.normal().is_ok());
		assert_eq!(pixel.rgba().unwrap(), &Rgba::default());
		assert_eq!(pixel.normal().unwrap(), &Normal::default());

		let buffer = Channel::UvNormal.default_pixel();
		let pixel = Pixel::from_buffer(&buffer, Channel::UvNormal);
		assert!(pixel.uv().is_ok());
		assert!(pixel.normal().is_ok());
		assert_eq!(pixel.uv().unwrap(), &Uv::default());
		assert_eq!(pixel.normal().unwrap(), &Normal::default());
	}

	#[test]
	fn pixel_set_color_within_channel() {
		let mut buffer = Channel::LumaNormal.default_pixel();
		let mut pixel = PixelMut::from_buffer_mut(&mut buffer, Channel::LumaNormal);
		assert!(pixel.luma().is_ok());
		assert!(pixel.normal().is_ok());
		*pixel.luma().unwrap() = Luma::new(128);
		*pixel.normal().unwrap() = Normal::new(0.2, 0.5, 0.8);
		assert_eq!(pixel.luma().unwrap(), &Luma::new(128));
		assert_eq!(pixel.normal().unwrap(), &Normal::new(0.2, 0.5, 0.8));
		assert_eq!(
			buffer,
			vec![128, 205, 204, 76, 62, 0, 0, 0, 63, 205, 204, 76, 63]
		);

		let mut buffer = Channel::LumaaNormal.default_pixel();
		let mut pixel = PixelMut::from_buffer_mut(&mut buffer, Channel::LumaaNormal);
		assert!(pixel.lumaa().is_ok());
		assert!(pixel.normal().is_ok());
		*pixel.lumaa().unwrap() = Lumaa::new(Luma::new(64), 128);
		*pixel.normal().unwrap() = Normal::new(0.2, 0.5, 0.8);
		assert_eq!(pixel.lumaa().unwrap(), &Lumaa::new(Luma::new(64), 128));
		assert_eq!(pixel.normal().unwrap(), &Normal::new(0.2, 0.5, 0.8));
		assert_eq!(
			buffer,
			vec![64, 128, 205, 204, 76, 62, 0, 0, 0, 63, 205, 204, 76, 63]
		);

		let mut buffer = Channel::RgbNormal.default_pixel();
		let mut pixel = PixelMut::from_buffer_mut(&mut buffer, Channel::RgbNormal);
		assert!(pixel.rgb().is_ok());
		assert!(pixel.normal().is_ok());
		*pixel.rgb().unwrap() = Rgb::new(32, 64, 96);
		*pixel.normal().unwrap() = Normal::new(0.2, 0.5, 0.8);
		assert_eq!(pixel.rgb().unwrap(), &Rgb::new(32, 64, 96));
		assert_eq!(pixel.normal().unwrap(), &Normal::new(0.2, 0.5, 0.8));
		assert_eq!(
			buffer,
			vec![32, 64, 96, 205, 204, 76, 62, 0, 0, 0, 63, 205, 204, 76, 63]
		);

		let mut buffer = Channel::RgbaNormal.default_pixel();
		let mut pixel = PixelMut::from_buffer_mut(&mut buffer, Channel::RgbaNormal);
		assert!(pixel.rgba().is_ok());
		assert!(pixel.normal().is_ok());
		*pixel.rgba().unwrap() = Rgba::new(Rgb::new(32, 64, 96), 128);
		*pixel.normal().unwrap() = Normal::new(0.2, 0.5, 0.8);
		assert_eq!(pixel.rgba().unwrap(), &Rgba::new(Rgb::new(32, 64, 96), 128));
		assert_eq!(pixel.normal().unwrap(), &Normal::new(0.2, 0.5, 0.8));
		assert_eq!(
			buffer,
			vec![32, 64, 96, 128, 205, 204, 76, 62, 0, 0, 0, 63, 205, 204, 76, 63]
		);

		let mut buffer = Channel::UvNormal.default_pixel();
		let mut pixel = PixelMut::from_buffer_mut(&mut buffer, Channel::UvNormal);
		assert!(pixel.uv().is_ok());
		assert!(pixel.normal().is_ok());
		*pixel.uv().unwrap() = Uv::new(0.2, 0.8);
		*pixel.normal().unwrap() = Normal::new(0.2, 0.5, 0.8);
		assert_eq!(pixel.uv().unwrap(), &Uv::new(0.2, 0.8));
		assert_eq!(pixel.normal().unwrap(), &Normal::new(0.2, 0.5, 0.8));
		assert_eq!(
			buffer,
			vec![
				205, 204, 76, 62, 205, 204, 76, 63, 205, 204, 76, 62, 0, 0, 0, 63, 205, 204, 76, 63
			]
		);
	}

	#[test]
	fn pixel_lerp() {
		let from_buf = Rgba::new(Rgb::new(255, 0, 0), 255).to_slice().to_vec();
		let from_px = Pixel::from_buffer(&from_buf, Channel::Rgba);
		let to_buf = Rgba::new(Rgb::new(0, 0, 255), 255).to_slice().to_vec();
		let to_px = Pixel::from_buffer(&to_buf, Channel::Rgba);
		let mut dst_buf = Channel::Rgba.default_pixel();
		let mut dst_px = PixelMut::from_buffer_mut(&mut dst_buf, Channel::Rgba);
		dst_px.lerp(&from_px, &to_px, 0.5).unwrap();
		assert_eq!(dst_buf, vec![127, 0, 127, 255]);
	}
}

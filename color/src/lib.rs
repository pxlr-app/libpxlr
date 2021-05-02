mod blending;
use serde::{Deserialize, Serialize};
use vek::ops::Lerp;

pub use blending::*;

pub trait Color: Default {
	const SIZE: usize;

	fn to_slice(&self) -> &[u8];
	fn to_slice_mut(&mut self) -> &mut [u8];
	fn from_slice(slice: &[u8]) -> &Self;
	fn from_slice_mut(slice: &mut [u8]) -> &mut Self;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Alpha<C: Color> {
	pub color: C,
	pub alpha: u8,
}

impl<C: Color> Alpha<C> {
	pub fn new(color: C, alpha: u8) -> Self {
		Self { color, alpha }
	}
}

impl<C: Color> Color for Alpha<C> {
	const SIZE: usize = 1 + std::mem::size_of::<C>();

	fn to_slice(&self) -> &[u8] {
		unsafe {
			std::slice::from_raw_parts(
				self as *const Self as *const u8,
				std::mem::size_of::<Self>(),
			)
		}
	}

	fn to_slice_mut(&mut self) -> &mut [u8] {
		unsafe {
			std::slice::from_raw_parts_mut(
				self as *mut Self as *mut u8,
				std::mem::size_of::<Self>(),
			)
		}
	}

	fn from_slice(slice: &[u8]) -> &Self {
		assert_eq!(slice.len(), std::mem::size_of::<Self>());
		unsafe { &*(slice.as_ptr() as *const Self) }
	}

	fn from_slice_mut(slice: &mut [u8]) -> &mut Self {
		assert_eq!(slice.len(), std::mem::size_of::<Self>());
		unsafe { &mut *(slice.as_ptr() as *mut Self) }
	}
}

impl<C: Color> Default for Alpha<C> {
	fn default() -> Self {
		Self {
			color: Default::default(),
			alpha: 255,
		}
	}
}

impl<C: Color> std::ops::Deref for Alpha<C> {
	type Target = C;

	fn deref(&self) -> &C {
		&self.color
	}
}

impl<C: Color> std::ops::DerefMut for Alpha<C> {
	fn deref_mut(&mut self) -> &mut C {
		&mut self.color
	}
}

macro_rules! define_color {
	($name:ident, ($($comp:ident),+), $type:ty) => {
		#[repr(C)]
		#[derive(Debug, Default, Clone, Copy, PartialEq)]
		pub struct $name {
			$(pub $comp: $type),+
		}

		impl $name {
			pub fn new($($comp: $type),+) -> Self {
				Self { $($comp: $comp,)+ }
			}
		}

		impl Color for $name {
			const SIZE: usize = std::mem::size_of::<Self>();

			fn to_slice(&self) -> &[u8] {
				unsafe { std::slice::from_raw_parts(self as *const Self as *const u8, std::mem::size_of::<Self>()) }
			}

			fn to_slice_mut(&mut self) -> &mut [u8] {
				unsafe { std::slice::from_raw_parts_mut(self as *mut Self as *mut u8, std::mem::size_of::<Self>()) }
			}

			fn from_slice(slice: &[u8]) -> &Self {
				assert_eq!(slice.len(), std::mem::size_of::<Self>());
				unsafe { &*(slice.as_ptr() as *const Self) }
			}

			fn from_slice_mut(slice: &mut [u8]) -> &mut Self {
				assert_eq!(slice.len(), std::mem::size_of::<Self>());
				unsafe { &mut *(slice.as_ptr() as *mut Self) }
			}
		}
	};
}

define_color!(Luma, (luma), u8);
pub type Lumaa = Alpha<Luma>;
define_color!(Rgb, (red, green, blue), u8);
pub type Rgba = Alpha<Rgb>;
define_color!(Uv, (u, v), f32);
define_color!(Normal, (x, y, z), f32);

impl From<Luma> for Rgb {
	fn from(value: Luma) -> Self {
		Rgb::new(value.luma, value.luma, value.luma)
	}
}
impl From<Rgb> for Luma {
	fn from(value: Rgb) -> Self {
		use palette::IntoColor;
		let luma: palette::LinLuma<palette::white_point::D65, f32> = palette::rgb::LinSrgb::new(
			(value.red as f32) / 255f32,
			(value.green as f32) / 255f32,
			(value.blue as f32) / 255f32,
		)
		.into_luma();
		Luma::new((luma.luma * 255f32) as u8)
	}
}
impl From<Uv> for Rgb {
	fn from(value: Uv) -> Self {
		Rgb::new((value.u * 255f32) as u8, (value.v * 255f32) as u8, 0)
	}
}
impl From<Rgb> for Uv {
	fn from(value: Rgb) -> Self {
		Uv::new((value.red as f32) / 255f32, (value.green as f32) / 255f32)
	}
}
impl From<Normal> for Rgb {
	fn from(value: Normal) -> Self {
		Rgb::new(
			(value.x * 255f32) as u8,
			(value.y * 255f32) as u8,
			(value.z * 255f32) as u8,
		)
	}
}
impl From<Rgb> for Normal {
	fn from(value: Rgb) -> Self {
		Normal::new(
			(value.red as f32) / 255f32,
			(value.green as f32) / 255f32,
			(value.blue as f32) / 255f32,
		)
	}
}
impl<C: Color> From<C> for Alpha<C> {
	fn from(value: C) -> Self {
		Alpha::new(value, 255)
	}
}
impl From<Alpha<Luma>> for Alpha<Rgb> {
	fn from(value: Alpha<Luma>) -> Self {
		Alpha::new(value.color.into(), value.alpha)
	}
}
impl From<Alpha<Rgb>> for Alpha<Luma> {
	fn from(value: Alpha<Rgb>) -> Self {
		Alpha::new(value.color.into(), value.alpha)
	}
}
impl From<Alpha<Uv>> for Alpha<Rgb> {
	fn from(value: Alpha<Uv>) -> Self {
		Alpha::new(value.color.into(), value.alpha)
	}
}
impl From<Alpha<Normal>> for Alpha<Rgb> {
	fn from(value: Alpha<Normal>) -> Self {
		Alpha::new(value.color.into(), value.alpha)
	}
}
impl Lerp<f32> for Luma {
	type Output = Luma;

	fn lerp_unclamped(from: Self, to: Self, factor: f32) -> Self::Output {
		Luma {
			luma: Lerp::lerp_unclamped(&from.luma, &to.luma, factor),
		}
	}
}
impl Lerp<f32> for Rgb {
	type Output = Rgb;

	fn lerp_unclamped(from: Self, to: Self, factor: f32) -> Self::Output {
		use palette::{LinSrgb, Mix};
		let a = LinSrgb::new(
			from.red as f32 / 255f32,
			from.green as f32 / 255f32,
			from.blue as f32 / 255f32,
		);
		let b = LinSrgb::new(
			to.red as f32 / 255f32,
			to.green as f32 / 255f32,
			to.blue as f32 / 255f32,
		);
		let c = a.mix(&b, factor);
		Rgb::new(
			(c.red * 255f32) as u8,
			(c.green * 255f32) as u8,
			(c.blue * 255f32) as u8,
		)
	}
}
impl Lerp<f32> for Uv {
	type Output = Uv;

	fn lerp_unclamped(from: Self, to: Self, factor: f32) -> Self::Output {
		use vek::vec::repr_c::vec2::Vec2;
		let vr = Vec2::lerp(Vec2::new(from.u, from.v), Vec2::new(to.u, to.v), factor);
		Uv::new(vr.x, vr.y)
	}
}
impl Lerp<f32> for Normal {
	type Output = Normal;

	fn lerp_unclamped(from: Self, to: Self, factor: f32) -> Self::Output {
		use vek::vec::repr_c::vec3::Vec3;
		let vr = Vec3::slerp(
			Vec3::new(from.x, from.y, from.z),
			Vec3::new(to.x, to.y, to.z),
			factor,
		);
		Normal::new(vr.x, vr.y, vr.z)
	}
}
impl<C: Lerp<f32, Output = C> + Copy + Color> Lerp<f32> for Alpha<C> {
	type Output = Alpha<<C as Lerp>::Output>;

	fn lerp_unclamped(from: Self, to: Self, factor: f32) -> Self::Output {
		Alpha {
			color: Lerp::lerp_unclamped(from.color, to.color, factor),
			alpha: Lerp::lerp_unclamped(&from.alpha, &to.alpha, factor),
		}
	}
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Channel {
	Luma,
	Lumaa,
	Rgb,
	Rgba,
	Uv,
	Normal,
	LumaNormal,
	LumaaNormal,
	RgbNormal,
	RgbaNormal,
	UvNormal,
}

impl std::fmt::Display for Channel {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			Channel::Luma => write!(f, "Channel::Luma"),
			Channel::Lumaa => write!(f, "Channel::Lumaa"),
			Channel::Rgb => write!(f, "Channel::Rgb"),
			Channel::Rgba => write!(f, "Channel::Rgba"),
			Channel::Uv => write!(f, "Channel::Uv"),
			Channel::Normal => write!(f, "Channel::Normal"),
			Channel::LumaNormal => write!(f, "Channel::LumaNormal"),
			Channel::LumaaNormal => write!(f, "Channel::LumaaNormal"),
			Channel::RgbNormal => write!(f, "Channel::RgbNormal"),
			Channel::RgbaNormal => write!(f, "Channel::RgbaNormal"),
			Channel::UvNormal => write!(f, "Channel::UvNormal"),
		}
	}
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ChannelError {
	NotFound(Channel),
}

impl std::error::Error for ChannelError {}

impl std::fmt::Display for ChannelError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			ChannelError::NotFound(chan) => write!(f, "Channel {} not found", chan),
		}
	}
}

impl Channel {
	/// Stride of a pixel in this Channel
	pub fn pixel_stride(&self) -> usize {
		match self {
			Channel::Luma => 1,
			Channel::Lumaa => 2,
			Channel::Rgb => 3,
			Channel::Rgba => 4,
			Channel::Uv => 8,
			Channel::Normal => 12,
			Channel::LumaNormal => 13,
			Channel::LumaaNormal => 14,
			Channel::RgbNormal => 15,
			Channel::RgbaNormal => 16,
			Channel::UvNormal => 20,
		}
	}

	/// Return an empty pixel for Channel
	pub fn default_pixel(&self) -> Vec<u8> {
		let mut data = Vec::with_capacity(self.pixel_stride());
		match self {
			Channel::Luma | Channel::LumaNormal => {
				data.extend_from_slice(Luma::default().to_slice())
			}
			Channel::Lumaa | Channel::LumaaNormal => {
				data.extend_from_slice(Lumaa::default().to_slice())
			}
			Channel::Rgb | Channel::RgbNormal => data.extend_from_slice(Rgb::default().to_slice()),
			Channel::Rgba | Channel::RgbaNormal => {
				data.extend_from_slice(Rgba::default().to_slice())
			}
			Channel::Uv | Channel::UvNormal => data.extend_from_slice(Uv::default().to_slice()),
			_ => {}
		}
		match self {
			Channel::LumaNormal
			| Channel::LumaaNormal
			| Channel::RgbNormal
			| Channel::RgbaNormal
			| Channel::UvNormal
			| Channel::Normal => data.extend_from_slice(Normal::default().to_slice()),
			_ => {}
		}
		data
	}

	/// Offset of channel in a Channel
	///
	/// Given a Channel of RgbaNormal, retrieve the offset of Normal.
	/// Offset is equal to the size of Rgba.
	///
	/// ```
	/// use color::*;
	/// let channel = Channel::RgbaNormal;
	/// assert_eq!(channel.offset_of(Channel::Rgba), Ok(0));
	/// assert_eq!(channel.offset_of(Channel::Normal), Ok(std::mem::size_of::<Rgba>()));
	/// ```
	pub fn offset_of(&self, channel: Channel) -> Result<usize, ChannelError> {
		match (self, channel) {
			(Channel::Luma, Channel::Luma)
			| (Channel::Lumaa, Channel::Lumaa)
			| (Channel::Rgb, Channel::Rgb)
			| (Channel::Rgba, Channel::Rgba)
			| (Channel::Uv, Channel::Uv)
			| (Channel::Normal, Channel::Normal)
			| (Channel::LumaNormal, Channel::Luma)
			| (Channel::LumaaNormal, Channel::Lumaa)
			| (Channel::RgbNormal, Channel::Rgb)
			| (Channel::RgbaNormal, Channel::Rgba)
			| (Channel::UvNormal, Channel::Uv) => Ok(0),
			(Channel::LumaNormal, Channel::Normal) => Ok(std::mem::size_of::<Luma>()),
			(Channel::LumaaNormal, Channel::Normal) => Ok(std::mem::size_of::<Lumaa>()),
			(Channel::RgbNormal, Channel::Normal) => Ok(std::mem::size_of::<Rgb>()),
			(Channel::RgbaNormal, Channel::Normal) => Ok(std::mem::size_of::<Rgba>()),
			(Channel::UvNormal, Channel::Normal) => Ok(std::mem::size_of::<Uv>()),
			_ => Err(ChannelError::NotFound(channel)),
		}
	}
}

impl Default for Channel {
	fn default() -> Self {
		Channel::Luma
	}
}

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
		blend_mode: Blend,
		compose_op: Compose,
		frt: &'frt Pixel,
		bck: &'bck Pixel,
	) -> Result<(), ChannelError> {
		assert_eq!(self.channel, frt.channel);
		assert_eq!(self.channel, bck.channel);

		match self.channel {
			Channel::Lumaa | Channel::LumaaNormal | Channel::Rgba | Channel::RgbaNormal => {
				if let Channel::LumaaNormal | Channel::RgbaNormal = self.channel {
					*self.normal().unwrap() = *frt.normal().unwrap();
				}
				match self.channel {
					Channel::Lumaa | Channel::LumaaNormal => {
						*self.luma().unwrap() = *frt.luma().unwrap();
					}
					Channel::Rgba | Channel::RgbaNormal => {
						let f = frt.rgba().unwrap();
						let b = bck.rgba().unwrap();

						let fr = f.color.red as f32 / 255f32;
						let fg = f.color.green as f32 / 255f32;
						let fb = f.color.blue as f32 / 255f32;
						let fa = f.alpha as f32 / 255f32;
						let ba = b.alpha as f32 / 255f32;
						let br = b.color.red as f32 / 255f32;
						let bg = b.color.green as f32 / 255f32;
						let bb = b.color.blue as f32 / 255f32;

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
								(rr * 255_f32).round() as u8,
								(rg * 255_f32).round() as u8,
								(rb * 255_f32).round() as u8,
							),
							(ra * 255_f32).round() as u8,
						)
					}
					_ => unreachable!(),
				};
			}
			// Without alpha, no blending
			_ => {
				self.data.copy_from_slice(frt.data);
			}
		};

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use image::{DynamicImage, ImageBuffer};
	use std::path::Path;

	#[test]
	fn color_sizes() {
		assert_eq!(std::mem::size_of::<Luma>(), 1);
		assert_eq!(std::mem::size_of::<Lumaa>(), 2);
		assert_eq!(std::mem::size_of::<Rgb>(), 3);
		assert_eq!(std::mem::size_of::<Rgba>(), 4);
		assert_eq!(std::mem::size_of::<Uv>(), 8);
		assert_eq!(std::mem::size_of::<Normal>(), 12);
	}

	#[test]
	fn color_conversion() {
		let luma_to_rgb: Rgb = Luma::new(128).into();
		assert_eq!(luma_to_rgb, Rgb::new(128, 128, 128));
		let luma_to_lumaa: Lumaa = Luma::new(128).into();
		assert_eq!(luma_to_lumaa, Alpha::new(Luma::new(128), 255));
		let lumaa_to_luma: Luma = *Alpha::new(Luma::new(128), 255);
		assert_eq!(lumaa_to_luma, Luma::new(128));
		let rgb_to_luma: Luma = Rgb::new(128, 128, 128).into();
		assert_eq!(rgb_to_luma, Luma::new(128));
		let rgb_to_rgba: Rgba = Rgb::new(1, 2, 3).into();
		assert_eq!(rgb_to_rgba, Alpha::new(Rgb::new(1, 2, 3), 255));
		let rgba_to_rgb: Rgb = *Alpha::new(Rgb::new(1, 2, 3), 255);
		assert_eq!(rgba_to_rgb, Rgb::new(1, 2, 3));
		let uv_to_rgb: Rgb = Uv::new(0.5, 0.5).into();
		assert_eq!(uv_to_rgb, Rgb::new(127, 127, 0));
		let rgb_to_uv: Uv = Rgb::new(127, 127, 0).into();
		assert_eq!(rgb_to_uv, Uv::new(0.49803922, 0.49803922));
		let nor_to_rgb: Rgb = Normal::new(0.5, 0.5, 0.5).into();
		assert_eq!(nor_to_rgb, Rgb::new(127, 127, 127));
		let rgb_to_nor: Normal = Rgb::new(120, 240, 90).into();
		assert_eq!(rgb_to_nor, Normal::new(0.47058824, 0.9411765, 0.3529412));
		let lumaa_to_rgba: Rgba = Alpha::new(Luma::new(128), 128).into();
		assert_eq!(lumaa_to_rgba, Alpha::new(Rgb::new(128, 128, 128), 128));
	}

	#[test]
	fn channel_strides() {
		assert_eq!(Channel::Luma.pixel_stride(), 1);
		assert_eq!(Channel::Lumaa.pixel_stride(), 2);
		assert_eq!(Channel::Rgb.pixel_stride(), 3);
		assert_eq!(Channel::Rgba.pixel_stride(), 4);
		assert_eq!(Channel::Uv.pixel_stride(), 8);
		assert_eq!(Channel::Normal.pixel_stride(), 12);
		assert_eq!(Channel::LumaNormal.pixel_stride(), 13);
		assert_eq!(Channel::LumaaNormal.pixel_stride(), 14);
		assert_eq!(Channel::RgbNormal.pixel_stride(), 15);
		assert_eq!(Channel::RgbaNormal.pixel_stride(), 16);
		assert_eq!(Channel::UvNormal.pixel_stride(), 20);
	}

	#[test]
	fn channel_default_pixel() {
		assert_eq!(Channel::Luma.default_pixel(), vec![0]);
		assert_eq!(Channel::Lumaa.default_pixel(), vec![0, 255]);
		assert_eq!(Channel::Rgb.default_pixel(), vec![0, 0, 0]);
		assert_eq!(Channel::Rgba.default_pixel(), vec![0, 0, 0, 255]);
		assert_eq!(Channel::Uv.default_pixel(), vec![0, 0, 0, 0, 0, 0, 0, 0]);
		assert_eq!(
			Channel::Normal.default_pixel(),
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
		);
		assert_eq!(
			Channel::LumaNormal.default_pixel(),
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
		);
		assert_eq!(
			Channel::LumaaNormal.default_pixel(),
			vec![0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
		);
		assert_eq!(
			Channel::RgbNormal.default_pixel(),
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
		);
		assert_eq!(
			Channel::RgbaNormal.default_pixel(),
			vec![0, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
		);
		assert_eq!(
			Channel::UvNormal.default_pixel(),
			vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
		);
	}

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

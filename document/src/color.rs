use crate::prelude::*;
use bitflags::bitflags;
use math::{Lerp, Vec2, Vec3};
use nom::number::complete::{le_f32, le_u8};
use std::fmt::Debug;

pub type Pixel = [u8];
pub type Pixels = [u8];

pub trait Color {
	const COMPONENTS: u8;
	const SIZE: usize;

	fn to_slice(&self) -> &[u8];
	fn to_slice_mut(&mut self) -> &mut [u8];
	fn from_slice(slice: &[u8]) -> &Self;
	fn from_slice_mut(slice: &mut [u8]) -> &mut Self;
}

macro_rules! define_color {
	($name:ident, $fn:ident, $channels:expr, $type:ty, $reader:expr, ($($comp:ident),+)) => {
		#[repr(C)]
		#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize)]
		pub struct $name {
			$(pub $comp: $type),+
		}

		pub fn $fn($($comp: $type),+) -> $name {
			$name::new($($comp,)+)
		}

		impl $name {
			pub fn new($($comp: $type),+) -> Self {
				Self { $($comp: $comp,)+ }
			}
		}

		impl Color for $name {
			const COMPONENTS: u8 = $channels;
			const SIZE: usize = $channels * std::mem::size_of::<$type>();

			fn to_slice(&self) -> &[u8] {
				unsafe { std::slice::from_raw_parts(self as *const Self as *const u8, std::mem::size_of::<Self>()) }
			}

			fn to_slice_mut(&mut self) -> &mut [u8] {
				unsafe { std::slice::from_raw_parts_mut(self as *mut Self as *mut u8, std::mem::size_of::<Self>()) }
			}

			fn from_slice(slice: &[u8]) -> &Self {
				assert_eq!(slice.len(), Self::SIZE);
				unsafe { &*(slice.as_ptr() as *const Self) }
			}

			fn from_slice_mut(slice: &mut [u8]) -> &mut Self {
				assert_eq!(slice.len(), Self::SIZE);
				unsafe { &mut *(slice.as_ptr() as *mut Self) }
			}
		}

		impl parser::Parse for $name {
			fn parse(bytes: &[u8]) -> nom::IResult<&[u8], $name> {
				$(let (bytes, $comp) = $reader(bytes)?;)+
				Ok((bytes, $name::new($($comp,)+)))
			}
		}

		impl parser::Write for $name {
			fn write(&self, writer: &mut dyn io::Write) -> io::Result<usize> {
				$(writer.write_all(&self.$comp.to_le_bytes())?;)+
				Ok($name::SIZE)
			}
		}

		impl std::convert::AsRef<[u8]> for $name {
			fn as_ref(&self) -> &[u8] {
				self.to_slice()
			}
		}
	};
}

define_color!(I, i, 1, u8, le_u8, (i));
define_color!(RGB, rgb, 3, u8, le_u8, (r, g, b));
define_color!(A, a, 1, u8, le_u8, (a));
define_color!(UV, uv, 2, f32, le_f32, (u, v));
define_color!(XYZ, xyz, 3, f32, le_f32, (x, y, z));

impl Lerp<f32> for &I {
	type Output = I;

	fn lerp_unclamped(from: Self, to: Self, factor: f32) -> Self::Output {
		if factor.round() == 0f32 {
			*from
		} else {
			*to
		}
	}
}

impl Lerp<f32> for &RGB {
	type Output = RGB;

	fn lerp_unclamped(from: Self, to: Self, factor: f32) -> Self::Output {
		// TODO : Lerp over HSL color space instead
		let vf = Vec3::new(from.r as f32, from.g as f32, from.b as f32);
		let vt = Vec3::new(to.r as f32, to.g as f32, to.b as f32);
		let vr = Vec3::lerp(vf, vt, factor);
		RGB::new(vr.x as u8, vr.y as u8, vr.z as u8)
	}
}

impl Lerp<f32> for &A {
	type Output = A;

	fn lerp_unclamped(from: Self, to: Self, factor: f32) -> Self::Output {
		A::new(f32::lerp(from.a as f32, to.a as f32, factor).round() as u8)
	}
}

impl Lerp<f32> for &UV {
	type Output = UV;

	fn lerp_unclamped(from: Self, to: Self, factor: f32) -> Self::Output {
		let vf = Vec2::new(from.u, from.v);
		let vt = Vec2::new(to.u, to.v);
		let vr = Vec2::lerp(vf, vt, factor);
		UV::new(vr.x, vr.y)
	}
}

impl Lerp<f32> for &XYZ {
	type Output = XYZ;

	fn lerp_unclamped(from: Self, to: Self, factor: f32) -> Self::Output {
		let vf = Vec3::new(from.x, from.y, from.z);
		let vt = Vec3::new(to.x, to.y, to.z);
		let vr = Vec3::slerp(vf, vt, factor);
		XYZ::new(vr.x, vr.y, vr.z)
	}
}

bitflags! {
	#[derive(Serialize, Deserialize)]
	pub struct Channel: u8 {
		const I = 0b00000001;
		const RGB = 0b00000010;
		const A = 0b00000100;
		const UV = 0b00001000;
		const XYZ = 0b00010000;
	}
}

impl Channel {
	/// Size of all channels of a Channel
	pub fn size(&self) -> usize {
		let channels = *self;
		let mut size = 0usize;
		if channels & Channel::I == Channel::I {
			size += I::SIZE;
		}
		if channels & Channel::RGB == Channel::RGB {
			size += RGB::SIZE;
		}
		if channels & Channel::A == Channel::A {
			size += A::SIZE;
		}
		if channels & Channel::UV == Channel::UV {
			size += UV::SIZE;
		}
		if channels & Channel::XYZ == Channel::XYZ {
			size += XYZ::SIZE;
		}
		size
	}

	/// Return an empty pixel for channels
	pub fn default_pixel(&self) -> Vec<u8> {
		vec![0u8; self.size()]
	}

	/// Offset of channel in a Channel
	///
	/// Given a Channel with RGBA channels, retrive the offset of channel A.
	/// Offset is equal to the size of RGB channels.
	///
	/// Given a Channel with RGBAXYZ channels, retrieve the offset of channel XYZ.
	/// Offset is equal to the size of RGB + A channels.
	///
	/// ```
	/// use document::color::*;
	/// let channels = Channel::RGB | Channel::A | Channel::XYZ;
	/// assert_eq!(channels.offset_of(Channel::XYZ), RGB::SIZE + A::SIZE);
	/// ```
	pub fn offset_of(&self, channel: Channel) -> usize {
		let channels = *self;
		let mut offset = 0usize;
		if channel == Channel::I {
			return offset;
		}
		if channels & Channel::I == Channel::I {
			offset += I::SIZE;
		}
		if channel == Channel::RGB {
			return offset;
		}
		if channels & Channel::RGB == Channel::RGB {
			offset += RGB::SIZE;
		}
		if channel == Channel::A {
			return offset;
		}
		if channels & Channel::A == Channel::A {
			offset += A::SIZE;
		}
		if channel == Channel::UV {
			return offset;
		}
		if channels & Channel::UV == Channel::UV {
			offset += UV::SIZE;
		}
		if channel == Channel::XYZ {
			return offset;
		}
		if channels & Channel::XYZ == Channel::XYZ {
			offset += XYZ::SIZE;
		}
		offset
	}

	/// Retrieve I channel as I color
	///
	/// ```
	/// use document::color::*;
	/// let color = I::new(1);
	/// let buffer = color.to_slice();
	/// assert!(Channel::I.i(buffer).is_some());
	/// assert_eq!(Channel::I.i(buffer).unwrap(), &color);
	/// ```
	pub fn i<'p>(&self, pixel: &'p Pixel) -> Option<&'p I> {
		if *self & Channel::I != Channel::I {
			None
		} else {
			Some(unsafe { self.unsafe_i(&pixel) })
		}
	}

	/// Retrieve I channel as I color
	///
	/// ```
	/// use document::color::*;
	/// let color = I::new(1);
	/// let buffer = color.to_slice();
	/// unsafe { assert_eq!(Channel::I.unsafe_i(buffer), &color); }
	/// ```
	pub unsafe fn unsafe_i<'p>(&self, pixel: &'p Pixel) -> &'p I {
		let offset = self.offset_of(Channel::I);
		let size = I::SIZE;
		I::from_slice(&pixel[offset..offset + size])
	}

	/// Retrieve mutable I channel as I color
	///
	/// ```
	/// use document::color::*;
	/// let mut color = I::new(1);
	/// let mut buffer = color.to_slice_mut();
	/// assert!(Channel::I.i_mut(buffer).is_some());
	/// assert_eq!(Channel::I.i_mut(buffer).unwrap(), &I::new(1));
	/// ```
	pub fn i_mut<'p>(&self, pixel: &'p mut Pixel) -> Option<&'p mut I> {
		if *self & Channel::I != Channel::I {
			None
		} else {
			Some(unsafe { self.unsafe_i_mut(pixel) })
		}
	}

	/// Retrieve I channel as I color
	///
	/// ```
	/// use document::color::*;
	/// let mut color = I::new(1);
	/// let mut buffer = color.to_slice_mut();
	/// unsafe { assert_eq!(Channel::I.unsafe_i_mut(buffer), &I::new(1)); }
	/// ```
	pub unsafe fn unsafe_i_mut<'p>(&self, pixel: &'p mut Pixel) -> &'p mut I {
		let offset = self.offset_of(Channel::I);
		let size = I::SIZE;
		I::from_slice_mut(&mut pixel[offset..offset + size])
	}

	/// Retrieve RGB channels as RGB color
	///
	/// ```
	/// use document::color::*;
	/// let color = RGB::new(1, 2, 3);
	/// let buffer = color.to_slice();
	/// assert!(Channel::RGB.rgb(buffer).is_some());
	/// assert_eq!(Channel::RGB.rgb(buffer).unwrap(), &color);
	/// ```
	pub fn rgb<'p>(&self, pixel: &'p Pixel) -> Option<&'p RGB> {
		if *self & Channel::RGB != Channel::RGB {
			None
		} else {
			Some(unsafe { self.unsafe_rgb(pixel) })
		}
	}

	/// Retrieve RGB channel as RGB color
	///
	/// ```
	/// use document::color::*;
	/// let color = RGB::new(1, 2, 3);
	/// let buffer = color.to_slice();
	/// unsafe { assert_eq!(Channel::RGB.unsafe_rgb(buffer), &color); }
	/// ```
	pub unsafe fn unsafe_rgb<'p>(&self, pixel: &'p Pixel) -> &'p RGB {
		let offset = self.offset_of(Channel::RGB);
		let size = RGB::SIZE;
		RGB::from_slice(&pixel[offset..offset + size])
	}

	/// Retrieve mutable RGB channels as RGB color
	///
	/// ```
	/// use document::color::*;
	/// let mut color = RGB::new(1, 2, 3);
	/// let mut buffer = color.to_slice_mut();
	/// assert!(Channel::RGB.rgb_mut(buffer).is_some());
	/// assert_eq!(Channel::RGB.rgb_mut(buffer).unwrap(), &RGB::new(1, 2, 3));
	/// ```
	pub fn rgb_mut<'p>(&self, pixel: &'p mut Pixel) -> Option<&'p mut RGB> {
		if *self & Channel::RGB != Channel::RGB {
			None
		} else {
			Some(unsafe { self.unsafe_rgb_mut(pixel) })
		}
	}

	/// Retrieve RGB channel as RGB color
	///
	/// ```
	/// use document::color::*;
	/// let mut color = RGB::new(1, 2, 3);
	/// let mut buffer = color.to_slice_mut();
	/// unsafe { assert_eq!(Channel::RGB.unsafe_rgb_mut(buffer), &RGB::new(1, 2, 3)); }
	/// ```
	pub unsafe fn unsafe_rgb_mut<'p>(&self, pixel: &'p mut Pixel) -> &'p mut RGB {
		let offset = self.offset_of(Channel::RGB);
		let size = RGB::SIZE;
		RGB::from_slice_mut(&mut pixel[offset..offset + size])
	}

	/// Retrieve A channel as A color
	///
	/// ```
	/// use document::color::*;
	/// let color = A::new(1);
	/// let buffer = color.to_slice();
	/// assert!(Channel::A.a(buffer).is_some());
	/// assert_eq!(Channel::A.a(buffer).unwrap(), &color);
	/// ```
	pub fn a<'p>(&self, pixel: &'p Pixel) -> Option<&'p A> {
		if *self & Channel::A != Channel::A {
			None
		} else {
			Some(unsafe { self.unsafe_a(pixel) })
		}
	}

	/// Retrieve A channel as A color
	///
	/// ```
	/// use document::color::*;
	/// let color = A::new(1);
	/// let buffer = color.to_slice();
	/// unsafe { assert_eq!(Channel::A.unsafe_a(buffer), &color); }
	/// ```
	pub unsafe fn unsafe_a<'p>(&self, pixel: &'p Pixel) -> &'p A {
		let offset = self.offset_of(Channel::A);
		let size = A::SIZE;
		A::from_slice(&pixel[offset..offset + size])
	}

	/// Retrieve mutable A channel as A color
	///
	/// ```
	/// use document::color::*;
	/// let mut color = A::new(1);
	/// let mut buffer = color.to_slice_mut();
	/// assert!(Channel::A.a_mut(buffer).is_some());
	/// assert_eq!(Channel::A.a_mut(buffer).unwrap(), &A::new(1));
	/// ```
	pub fn a_mut<'p>(&self, pixel: &'p mut Pixel) -> Option<&'p mut A> {
		if *self & Channel::A != Channel::A {
			None
		} else {
			Some(unsafe { self.unsafe_a_mut(pixel) })
		}
	}

	/// Retrieve A channel as A color
	///
	/// ```
	/// use document::color::*;
	/// let mut color = A::new(1);
	/// let mut buffer = color.to_slice_mut();
	/// unsafe { assert_eq!(Channel::A.unsafe_a_mut(buffer), &A::new(1)); }
	/// ```
	pub unsafe fn unsafe_a_mut<'p>(&self, pixel: &'p mut Pixel) -> &'p mut A {
		let offset = self.offset_of(Channel::A);
		let size = A::SIZE;
		A::from_slice_mut(&mut pixel[offset..offset + size])
	}

	/// Retrieve UV channels as UV color
	///
	/// ```
	/// use document::color::*;
	/// let color = UV::new(1., 2.);
	/// let buffer = color.to_slice();
	/// assert!(Channel::UV.uv(buffer).is_some());
	/// assert_eq!(Channel::UV.uv(buffer).unwrap(), &color);
	/// ```
	pub fn uv<'p>(&self, pixel: &'p Pixel) -> Option<&'p UV> {
		if *self & Channel::UV != Channel::UV {
			None
		} else {
			Some(unsafe { self.unsafe_uv(pixel) })
		}
	}

	/// Retrieve UV channel as UV color
	///
	/// ```
	/// use document::color::*;
	/// let color = UV::new(1., 2.);
	/// let buffer = color.to_slice();
	/// unsafe { assert_eq!(Channel::UV.unsafe_uv(buffer), &color); }
	/// ```
	pub unsafe fn unsafe_uv<'p>(&self, pixel: &'p Pixel) -> &'p UV {
		let offset = self.offset_of(Channel::UV);
		let size = UV::SIZE;
		UV::from_slice(&pixel[offset..offset + size])
	}

	/// Retrieve mutable UV channels as UV color
	///
	/// ```
	/// use document::color::*;
	/// let mut color = UV::new(1., 2.);
	/// let mut buffer = color.to_slice_mut();
	/// assert!(Channel::UV.uv_mut(buffer).is_some());
	/// assert_eq!(Channel::UV.uv_mut(buffer).unwrap(), &UV::new(1., 2.));
	/// ```
	pub fn uv_mut<'p>(&self, pixel: &'p mut Pixel) -> Option<&'p mut UV> {
		if *self & Channel::UV != Channel::UV {
			None
		} else {
			Some(unsafe { self.unsafe_uv_mut(pixel) })
		}
	}

	/// Retrieve UV channel as UV color
	///
	/// ```
	/// use document::color::*;
	/// let mut color = UV::new(1., 2.);
	/// let mut buffer = color.to_slice_mut();
	/// unsafe { assert_eq!(Channel::UV.unsafe_uv_mut(buffer), &UV::new(1., 2.)); }
	/// ```
	pub unsafe fn unsafe_uv_mut<'p>(&self, pixel: &'p mut Pixel) -> &'p mut UV {
		let offset = self.offset_of(Channel::UV);
		let size = UV::SIZE;
		UV::from_slice_mut(&mut pixel[offset..offset + size])
	}

	/// Retrieve XYZ channels as XYZ color
	///
	/// ```
	/// use document::color::*;
	/// let color = XYZ::new(1., 2., 3.);
	/// let buffer = color.to_slice();
	/// assert!(Channel::XYZ.xyz(buffer).is_some());
	/// assert_eq!(Channel::XYZ.xyz(buffer).unwrap(), &color);
	/// ```
	pub fn xyz<'p>(&self, pixel: &'p Pixel) -> Option<&'p XYZ> {
		if *self & Channel::XYZ != Channel::XYZ {
			None
		} else {
			Some(unsafe { self.unsafe_xyz(pixel) })
		}
	}

	/// Retrieve XYZ channel as XYZ color
	///
	/// ```
	/// use document::color::*;
	/// let color = XYZ::new(1., 2., 3.);
	/// let buffer = color.to_slice();
	/// unsafe { assert_eq!(Channel::XYZ.unsafe_xyz(buffer), &color); }
	/// ```
	pub unsafe fn unsafe_xyz<'p>(&self, pixel: &'p Pixel) -> &'p XYZ {
		let offset = self.offset_of(Channel::XYZ);
		let size = XYZ::SIZE;
		XYZ::from_slice(&pixel[offset..offset + size])
	}

	/// Retrieve mutable XYZ channels as XYZ color
	///
	/// ```
	/// use document::color::*;
	/// let mut color = XYZ::new(1., 2., 3.);
	/// let mut buffer = color.to_slice_mut();
	/// assert!(Channel::XYZ.xyz_mut(buffer).is_some());
	/// assert_eq!(Channel::XYZ.xyz_mut(buffer).unwrap(), &XYZ::new(1., 2., 3.));
	/// ```
	pub fn xyz_mut<'p>(&self, pixel: &'p mut Pixel) -> Option<&'p mut XYZ> {
		if *self & Channel::XYZ != Channel::XYZ {
			None
		} else {
			Some(unsafe { self.unsafe_xyz_mut(pixel) })
		}
	}

	/// Retrieve XYZ channel as XYZ color
	///
	/// ```
	/// use document::color::*;
	/// let mut color = XYZ::new(1., 2., 3.);
	/// let mut buffer = color.to_slice_mut();
	/// unsafe { assert_eq!(Channel::XYZ.unsafe_xyz_mut(buffer), &XYZ::new(1., 2., 3.)); }
	/// ```
	pub unsafe fn unsafe_xyz_mut<'p>(&self, pixel: &'p mut Pixel) -> &'p mut XYZ {
		let offset = self.offset_of(Channel::XYZ);
		let size = XYZ::SIZE;
		XYZ::from_slice_mut(&mut pixel[offset..offset + size])
	}
}

#[cfg(test)]
mod tests {
	use crate::prelude::*;

	#[test]
	fn test_channels() {
		let channels = Channel::I | Channel::RGB | Channel::A | Channel::UV;
		let buffer: &Pixel = &[1, 1, 2, 3, 2, 0, 0, 128, 63, 0, 0, 0, 64];
		assert_eq!(channels.size(), 13);
		assert_eq!(channels.i(&buffer).unwrap(), &I::new(1));
		assert_eq!(channels.rgb(&buffer).unwrap(), &RGB::new(1, 2, 3));
		assert_eq!(channels.a(&buffer).unwrap(), &A::new(2));
		assert_eq!(channels.uv(&buffer).unwrap(), &UV::new(1., 2.));
		assert!(channels.xyz(&buffer).is_none());

		let mut buffer: &mut Pixel = &mut [1, 1, 2, 3, 2, 0, 0, 128, 63, 0, 0, 0, 64];
		assert_eq!(channels.size(), 13);
		assert_eq!(channels.i_mut(&mut buffer).unwrap(), &I::new(1));
		assert_eq!(channels.rgb_mut(&mut buffer).unwrap(), &RGB::new(1, 2, 3));
		assert_eq!(channels.a_mut(&mut buffer).unwrap(), &A::new(2));
		assert_eq!(channels.uv_mut(&mut buffer).unwrap(), &UV::new(1., 2.));
		assert!(channels.xyz_mut(&mut buffer).is_none());
	}

	#[test]
	fn test_channels_mut() {
		let channels = Channel::I | Channel::RGB | Channel::A | Channel::UV;

		let mut buffer: Vec<u8> = vec![0u8; channels.size()];
		*channels.i_mut(&mut buffer[..]).unwrap() = I::new(1);
		*channels.rgb_mut(&mut buffer[..]).unwrap() = RGB::new(1, 2, 3);
		*channels.a_mut(&mut buffer[..]).unwrap() = A::new(2);
		*channels.uv_mut(&mut buffer[..]).unwrap() = UV::new(1., 2.);
		assert_eq!(buffer, &[1, 1, 2, 3, 2, 0, 0, 128, 63, 0, 0, 0, 64][..]);

		let buffer: &mut Pixel = &mut [1, 1, 2, 3, 2, 0, 0, 128, 63, 0, 0, 0, 64];
		{
			let mut rgb = channels.rgb_mut(buffer).unwrap();
			rgb.r = 4;
			assert_eq!(rgb, &RGB::new(4, 2, 3));
		}
		{
			let mut uv = channels.uv_mut(buffer).unwrap();
			uv.u = 3.;
			assert_eq!(uv, &UV::new(3., 2.));
		}
		assert_eq!(buffer, &[1, 4, 2, 3, 2, 0, 0, 64, 64, 0, 0, 0, 64]);
	}
}

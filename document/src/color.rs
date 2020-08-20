use crate::prelude::*;
use nom::number::complete::{le_f32, le_u8};
use std::fmt::Debug;

pub trait Color {
	const COMPONENTS: u8;
	const SIZE: usize;

	fn to_slice(&self) -> &[u8];
	fn to_slice_mut(&mut self) -> &mut [u8];
	fn from_slice(slice: &[u8]) -> &Self;
	fn from_slice_mut(slice: &mut [u8]) -> &mut Self;
}

macro_rules! define_color {
	($name:ident, $components:expr, $type:ty, $reader:expr, ($($comp:ident),+)) => {
		#[repr(C)]
		#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize)]
		pub struct $name {
			$(pub $comp: $type),+
		}

		impl $name {
			pub fn new($($comp: $type),+) -> Self {
				Self { $($comp: $comp,)+ }
			}
		}

		impl Color for $name {
			const COMPONENTS: u8 = $components;
			const SIZE: usize = $components * std::mem::size_of::<$type>();

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
	};
}

define_color!(I, 1, u8, le_u8, (i));
define_color!(RGB, 3, u8, le_u8, (r, g, b));
define_color!(A, 1, u8, le_u8, (a));
define_color!(UV, 2, f32, le_f32, (u, v));
define_color!(XYZ, 3, f32, le_f32, (x, y, z));

#[derive(Serialize, Deserialize)]
pub struct Pixel;

impl Pixel {
	pub const I: u8 = 0b00000001;
	pub const RGB: u8 = 0b00000010;
	pub const A: u8 = 0b00000100;
	pub const UV: u8 = 0b00001000;
	pub const XYZ: u8 = 0b00010000;

	/// Size of all components of a Pixel
	pub fn size_of(components: u8) -> usize {
		let mut size = 0usize;
		if components & Pixel::I > 0 {
			size += I::SIZE;
		}
		if components & Pixel::RGB > 0 {
			size += RGB::SIZE;
		}
		if components & Pixel::A > 0 {
			size += A::SIZE;
		}
		if components & Pixel::UV > 0 {
			size += UV::SIZE;
		}
		if components & Pixel::XYZ > 0 {
			size += XYZ::SIZE;
		}
		size
	}

	/// Offset of component in a Pixel
	///
	/// Given a Pixel with RGBA components, retrive the offset of component A.
	/// Offset is equal to the size of RGB components.
	///
	/// Given a Pixel with RGBAXYZ components, retrieve the offset of component XYZ.
	/// Offset is equal to the size of RGB + A components.
	///
	/// ```
	/// use document::color::*;
	/// let components = Pixel::RGB | Pixel::A | Pixel::XYZ;
	/// assert_eq!(Pixel::offset_of(components, Pixel::XYZ), RGB::SIZE + A::SIZE);
	/// ```
	pub fn offset_of(components: u8, component: u8) -> usize {
		let mut offset = 0usize;
		if component == Pixel::I {
			return offset;
		}
		if components & Pixel::I > 0 {
			offset += I::SIZE;
		}
		if component == Pixel::RGB {
			return offset;
		}
		if components & Pixel::RGB > 0 {
			offset += RGB::SIZE;
		}
		if component == Pixel::A {
			return offset;
		}
		if components & Pixel::A > 0 {
			offset += A::SIZE;
		}
		if component == Pixel::UV {
			return offset;
		}
		if components & Pixel::UV > 0 {
			offset += UV::SIZE;
		}
		if component == Pixel::XYZ {
			return offset;
		}
		if components & Pixel::XYZ > 0 {
			offset += XYZ::SIZE;
		}
		offset
	}

	/// Retrieve I component as I color
	///
	/// ```
	/// use document::color::*;
	/// let color = I::new(1);
	/// let buffer = color.to_slice();
	/// assert!(Pixel::i(Pixel::I, buffer).is_some());
	/// assert_eq!(Pixel::i(Pixel::I, buffer).unwrap(), &color);
	/// ```
	pub fn i(components: u8, data: &[u8]) -> Option<&I> {
		if components & Pixel::I == 0 {
			None
		} else {
			let offset = Pixel::offset_of(components, Pixel::I);
			let size = I::SIZE;
			Some(I::from_slice(&data[offset..offset + size]))
		}
	}

	/// Retrieve mutable I component as I color
	///
	/// ```
	/// use document::color::*;
	/// let mut color = I::new(1);
	/// let mut buffer = color.to_slice_mut();
	/// assert!(Pixel::i_mut(Pixel::I, buffer).is_some());
	/// assert_eq!(Pixel::i_mut(Pixel::I, buffer).unwrap(), &I::new(1));
	/// ```
	pub fn i_mut(components: u8, data: &mut [u8]) -> Option<&mut I> {
		if components & Pixel::I == 0 {
			None
		} else {
			let offset = Pixel::offset_of(components, Pixel::I);
			let size = I::SIZE;
			Some(I::from_slice_mut(&mut data[offset..offset + size]))
		}
	}

	/// Retrieve RGB components as RGB color
	///
	/// ```
	/// use document::color::*;
	/// let color = RGB::new(1, 2, 3);
	/// let buffer = color.to_slice();
	/// assert!(Pixel::rgb(Pixel::RGB, buffer).is_some());
	/// assert_eq!(Pixel::rgb(Pixel::RGB, buffer).unwrap(), &color);
	/// ```
	pub fn rgb(components: u8, data: &[u8]) -> Option<&RGB> {
		if components & Pixel::RGB == 0 {
			None
		} else {
			let offset = Pixel::offset_of(components, Pixel::RGB);
			let size = RGB::SIZE;
			Some(RGB::from_slice(&data[offset..offset + size]))
		}
	}

	/// Retrieve mutable RGB components as RGB color
	///
	/// ```
	/// use document::color::*;
	/// let mut color = RGB::new(1, 2, 3);
	/// let mut buffer = color.to_slice_mut();
	/// assert!(Pixel::rgb_mut(Pixel::RGB, buffer).is_some());
	/// assert_eq!(Pixel::rgb_mut(Pixel::RGB, buffer).unwrap(), &RGB::new(1, 2, 3));
	/// ```
	pub fn rgb_mut(components: u8, data: &mut [u8]) -> Option<&mut RGB> {
		if components & Pixel::RGB == 0 {
			None
		} else {
			let offset = Pixel::offset_of(components, Pixel::RGB);
			let size = RGB::SIZE;
			Some(RGB::from_slice_mut(&mut data[offset..offset + size]))
		}
	}

	/// Retrieve A component as A color
	///
	/// ```
	/// use document::color::*;
	/// let color = A::new(1);
	/// let buffer = color.to_slice();
	/// assert!(Pixel::a(Pixel::A, buffer).is_some());
	/// assert_eq!(Pixel::a(Pixel::A, buffer).unwrap(), &color);
	/// ```
	pub fn a(components: u8, data: &[u8]) -> Option<&A> {
		if components & Pixel::A == 0 {
			None
		} else {
			let offset = Pixel::offset_of(components, Pixel::A);
			let size = A::SIZE;
			Some(A::from_slice(&data[offset..offset + size]))
		}
	}

	/// Retrieve mutable A component as A color
	///
	/// ```
	/// use document::color::*;
	/// let mut color = A::new(1);
	/// let mut buffer = color.to_slice_mut();
	/// assert!(Pixel::a_mut(Pixel::A, buffer).is_some());
	/// assert_eq!(Pixel::a_mut(Pixel::A, buffer).unwrap(), &A::new(1));
	/// ```
	pub fn a_mut(components: u8, data: &mut [u8]) -> Option<&mut A> {
		if components & Pixel::A == 0 {
			None
		} else {
			let offset = Pixel::offset_of(components, Pixel::A);
			let size = A::SIZE;
			Some(A::from_slice_mut(&mut data[offset..offset + size]))
		}
	}

	/// Retrieve UV components as UV color
	///
	/// ```
	/// use document::color::*;
	/// let color = UV::new(1., 2.);
	/// let buffer = color.to_slice();
	/// assert!(Pixel::uv(Pixel::UV, buffer).is_some());
	/// assert_eq!(Pixel::uv(Pixel::UV, buffer).unwrap(), &color);
	/// ```
	pub fn uv(components: u8, data: &[u8]) -> Option<&UV> {
		if components & Pixel::UV == 0 {
			None
		} else {
			let offset = Pixel::offset_of(components, Pixel::UV);
			let size = UV::SIZE;
			Some(UV::from_slice(&data[offset..offset + size]))
		}
	}

	/// Retrieve mutable UV components as UV color
	///
	/// ```
	/// use document::color::*;
	/// let mut color = UV::new(1., 2.);
	/// let mut buffer = color.to_slice_mut();
	/// assert!(Pixel::uv_mut(Pixel::UV, buffer).is_some());
	/// assert_eq!(Pixel::uv_mut(Pixel::UV, buffer).unwrap(), &UV::new(1., 2.));
	/// ```
	pub fn uv_mut(components: u8, data: &mut [u8]) -> Option<&mut UV> {
		if components & Pixel::UV == 0 {
			None
		} else {
			let offset = Pixel::offset_of(components, Pixel::UV);
			let size = UV::SIZE;
			Some(UV::from_slice_mut(&mut data[offset..offset + size]))
		}
	}

	/// Retrieve XYZ components as XYZ color
	///
	/// ```
	/// use document::color::*;
	/// let color = XYZ::new(1., 2., 3.);
	/// let buffer = color.to_slice();
	/// assert!(Pixel::xyz(Pixel::XYZ, buffer).is_some());
	/// assert_eq!(Pixel::xyz(Pixel::XYZ, buffer).unwrap(), &color);
	/// ```
	pub fn xyz(components: u8, data: &[u8]) -> Option<&XYZ> {
		if components & Pixel::XYZ == 0 {
			None
		} else {
			let offset = Pixel::offset_of(components, Pixel::XYZ);
			let size = XYZ::SIZE;
			Some(XYZ::from_slice(&data[offset..offset + size]))
		}
	}

	/// Retrieve mutable XYZ components as XYZ color
	///
	/// ```
	/// use document::color::*;
	/// let mut color = XYZ::new(1., 2., 3.);
	/// let mut buffer = color.to_slice_mut();
	/// assert!(Pixel::xyz_mut(Pixel::XYZ, buffer).is_some());
	/// assert_eq!(Pixel::xyz_mut(Pixel::XYZ, buffer).unwrap(), &XYZ::new(1., 2., 3.));
	/// ```
	pub fn xyz_mut(components: u8, data: &mut [u8]) -> Option<&mut XYZ> {
		if components & Pixel::XYZ == 0 {
			None
		} else {
			let offset = Pixel::offset_of(components, Pixel::XYZ);
			let size = XYZ::SIZE;
			Some(XYZ::from_slice_mut(&mut data[offset..offset + size]))
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::prelude::*;

	#[test]
	fn test_components() {
		let channels = Pixel::I | Pixel::RGB | Pixel::A | Pixel::UV;
		let buffer: &[u8] = &[1, 1, 2, 3, 2, 0, 0, 128, 63, 0, 0, 0, 64];
		assert_eq!(Pixel::size_of(channels), 13);
		assert_eq!(Pixel::i(channels, &buffer).unwrap(), &I::new(1));
		assert_eq!(Pixel::rgb(channels, &buffer).unwrap(), &RGB::new(1, 2, 3));
		assert_eq!(Pixel::a(channels, &buffer).unwrap(), &A::new(2));
		assert_eq!(Pixel::uv(channels, &buffer).unwrap(), &UV::new(1., 2.));
		assert!(Pixel::xyz(channels, &buffer).is_none());

		let mut buffer: &mut [u8] = &mut [1, 1, 2, 3, 2, 0, 0, 128, 63, 0, 0, 0, 64];
		assert_eq!(Pixel::size_of(channels), 13);
		assert_eq!(Pixel::i_mut(channels, &mut buffer).unwrap(), &I::new(1));
		assert_eq!(
			Pixel::rgb_mut(channels, &mut buffer).unwrap(),
			&RGB::new(1, 2, 3)
		);
		assert_eq!(Pixel::a_mut(channels, &mut buffer).unwrap(), &A::new(2));
		assert_eq!(
			Pixel::uv_mut(channels, &mut buffer).unwrap(),
			&UV::new(1., 2.)
		);
		assert!(Pixel::xyz_mut(channels, &mut buffer).is_none());
	}

	#[test]
	fn test_components_mut() {
		let channels = Pixel::I | Pixel::RGB | Pixel::A | Pixel::UV;
		let buffer: &mut [u8] = &mut [1, 1, 2, 3, 2, 0, 0, 128, 63, 0, 0, 0, 64];
		{
			let mut rgb = Pixel::rgb_mut(channels, buffer).unwrap();
			rgb.r = 4;
			assert_eq!(rgb, &RGB::new(4, 2, 3));
		}
		{
			let mut uv = Pixel::uv_mut(channels, buffer).unwrap();
			uv.u = 3.;
			assert_eq!(uv, &UV::new(3., 2.));
		}
		assert_eq!(buffer, &[1, 4, 2, 3, 2, 0, 0, 64, 64, 0, 0, 0, 64]);
	}
}

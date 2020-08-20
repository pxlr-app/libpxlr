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
pub struct Pixel<'data> {
	components: u8,
	data: &'data [u8],
}

impl<'data> Pixel<'data> {
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

	pub fn from_slice(components: u8, data: &'data [u8]) -> Self {
		Self { components, data }
	}

	pub fn size(&self) -> usize {
		Self::size_of(self.components)
	}

	/// Retrieve I component as I color
	///
	/// ```
	/// use document::color::*;
	/// let color = I::new(1);
	/// let pixel = Pixel::from_slice(Pixel::I, &color.to_slice());
	/// assert!(pixel.i().is_some());
	/// assert_eq!(pixel.i().unwrap(), &color);
	/// ```
	pub fn i(&self) -> Option<&I> {
		if self.components & Pixel::I == 0 {
			None
		} else {
			let offset = Pixel::offset_of(self.components, Pixel::I);
			let size = I::SIZE;
			Some(I::from_slice(&self.data[offset..offset + size]))
		}
	}

	/// Retrieve RGB components as RGB color
	///
	/// ```
	/// use document::color::*;
	/// let color = RGB::new(1, 2, 3);
	/// let pixel = Pixel::from_slice(Pixel::RGB, &color.to_slice());
	/// assert!(pixel.rgb().is_some());
	/// assert_eq!(pixel.rgb().unwrap(), &color);
	/// ```
	pub fn rgb(&self) -> Option<&RGB> {
		if self.components & Pixel::RGB == 0 {
			None
		} else {
			let offset = Pixel::offset_of(self.components, Pixel::RGB);
			let size = RGB::SIZE;
			Some(RGB::from_slice(&self.data[offset..offset + size]))
		}
	}

	/// Retrieve A component as A color
	///
	/// ```
	/// use document::color::*;
	/// let color = A::new(1);
	/// let pixel = Pixel::from_slice(Pixel::A, &color.to_slice());
	/// assert!(pixel.a().is_some());
	/// assert_eq!(pixel.a().unwrap(), &color);
	/// ```
	pub fn a(&self) -> Option<&A> {
		if self.components & Pixel::A == 0 {
			None
		} else {
			let offset = Pixel::offset_of(self.components, Pixel::A);
			let size = A::SIZE;
			Some(A::from_slice(&self.data[offset..offset + size]))
		}
	}

	/// Retrieve UV components as UV color
	///
	/// ```
	/// use document::color::*;
	/// let color = UV::new(1., 2.);
	/// let pixel = Pixel::from_slice(Pixel::UV, &color.to_slice());
	/// assert!(pixel.uv().is_some());
	/// assert_eq!(pixel.uv().unwrap(), &color);
	/// ```
	pub fn uv(&self) -> Option<&UV> {
		if self.components & Pixel::UV == 0 {
			None
		} else {
			let offset = Pixel::offset_of(self.components, Pixel::UV);
			let size = UV::SIZE;
			Some(UV::from_slice(&self.data[offset..offset + size]))
		}
	}

	/// Retrieve XYZ components as XYZ color
	///
	/// ```
	/// use document::color::*;
	/// let color = XYZ::new(1., 2., 3.);
	/// let pixel = Pixel::from_slice(Pixel::XYZ, &color.to_slice());
	/// assert!(pixel.xyz().is_some());
	/// assert_eq!(pixel.xyz().unwrap(), &color);
	/// ```
	pub fn xyz(&self) -> Option<&XYZ> {
		if self.components & Pixel::XYZ == 0 {
			None
		} else {
			let offset = Pixel::offset_of(self.components, Pixel::XYZ);
			let size = XYZ::SIZE;
			Some(XYZ::from_slice(&self.data[offset..offset + size]))
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::prelude::*;

	#[test]
	fn test_components() {
		let pixel = Pixel::from_slice(
			Pixel::I | Pixel::RGB | Pixel::A | Pixel::UV,
			&[1, 1, 2, 3, 2, 0, 0, 128, 63, 0, 0, 0, 64],
		);
		assert_eq!(pixel.size(), 13);
		assert_eq!(pixel.i().unwrap(), &I::new(1));
		assert_eq!(pixel.rgb().unwrap(), &RGB::new(1, 2, 3));
		assert_eq!(pixel.a().unwrap(), &A::new(2));
		assert_eq!(pixel.uv().unwrap(), &UV::new(1., 2.));
		assert!(pixel.xyz().is_none());
	}
}

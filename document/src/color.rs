use crate::prelude::*;
use bitflags::bitflags;
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
	($name:ident, $channels:expr, $type:ty, $reader:expr, ($($comp:ident),+)) => {
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

define_color!(I, 1, u8, le_u8, (i));
define_color!(RGB, 3, u8, le_u8, (r, g, b));
define_color!(A, 1, u8, le_u8, (a));
define_color!(UV, 2, f32, le_f32, (u, v));
define_color!(XYZ, 3, f32, le_f32, (x, y, z));

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
	pub fn size_of(channels: Channel) -> usize {
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
	/// assert_eq!(Channel::offset_of(channels, Channel::XYZ), RGB::SIZE + A::SIZE);
	/// ```
	pub fn offset_of(channels: Channel, channel: Channel) -> usize {
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
	/// assert!(Channel::i(Channel::I, buffer).is_some());
	/// assert_eq!(Channel::i(Channel::I, buffer).unwrap(), &color);
	/// ```
	pub fn i(channels: Channel, data: &[u8]) -> Option<&I> {
		if channels & Channel::I != Channel::I {
			None
		} else {
			let offset = Channel::offset_of(channels, Channel::I);
			let size = I::SIZE;
			Some(I::from_slice(&data[offset..offset + size]))
		}
	}

	/// Retrieve mutable I channel as I color
	///
	/// ```
	/// use document::color::*;
	/// let mut color = I::new(1);
	/// let mut buffer = color.to_slice_mut();
	/// assert!(Channel::i_mut(Channel::I, buffer).is_some());
	/// assert_eq!(Channel::i_mut(Channel::I, buffer).unwrap(), &I::new(1));
	/// ```
	pub fn i_mut(channels: Channel, data: &mut [u8]) -> Option<&mut I> {
		if channels & Channel::I != Channel::I {
			None
		} else {
			let offset = Channel::offset_of(channels, Channel::I);
			let size = I::SIZE;
			Some(I::from_slice_mut(&mut data[offset..offset + size]))
		}
	}

	/// Retrieve RGB channels as RGB color
	///
	/// ```
	/// use document::color::*;
	/// let color = RGB::new(1, 2, 3);
	/// let buffer = color.to_slice();
	/// assert!(Channel::rgb(Channel::RGB, buffer).is_some());
	/// assert_eq!(Channel::rgb(Channel::RGB, buffer).unwrap(), &color);
	/// ```
	pub fn rgb(channels: Channel, data: &[u8]) -> Option<&RGB> {
		if channels & Channel::RGB != Channel::RGB {
			None
		} else {
			let offset = Channel::offset_of(channels, Channel::RGB);
			let size = RGB::SIZE;
			Some(RGB::from_slice(&data[offset..offset + size]))
		}
	}

	/// Retrieve mutable RGB channels as RGB color
	///
	/// ```
	/// use document::color::*;
	/// let mut color = RGB::new(1, 2, 3);
	/// let mut buffer = color.to_slice_mut();
	/// assert!(Channel::rgb_mut(Channel::RGB, buffer).is_some());
	/// assert_eq!(Channel::rgb_mut(Channel::RGB, buffer).unwrap(), &RGB::new(1, 2, 3));
	/// ```
	pub fn rgb_mut(channels: Channel, data: &mut [u8]) -> Option<&mut RGB> {
		if channels & Channel::RGB != Channel::RGB {
			None
		} else {
			let offset = Channel::offset_of(channels, Channel::RGB);
			let size = RGB::SIZE;
			Some(RGB::from_slice_mut(&mut data[offset..offset + size]))
		}
	}

	/// Retrieve A channel as A color
	///
	/// ```
	/// use document::color::*;
	/// let color = A::new(1);
	/// let buffer = color.to_slice();
	/// assert!(Channel::a(Channel::A, buffer).is_some());
	/// assert_eq!(Channel::a(Channel::A, buffer).unwrap(), &color);
	/// ```
	pub fn a(channels: Channel, data: &[u8]) -> Option<&A> {
		if channels & Channel::A != Channel::A {
			None
		} else {
			let offset = Channel::offset_of(channels, Channel::A);
			let size = A::SIZE;
			Some(A::from_slice(&data[offset..offset + size]))
		}
	}

	/// Retrieve mutable A channel as A color
	///
	/// ```
	/// use document::color::*;
	/// let mut color = A::new(1);
	/// let mut buffer = color.to_slice_mut();
	/// assert!(Channel::a_mut(Channel::A, buffer).is_some());
	/// assert_eq!(Channel::a_mut(Channel::A, buffer).unwrap(), &A::new(1));
	/// ```
	pub fn a_mut(channels: Channel, data: &mut [u8]) -> Option<&mut A> {
		if channels & Channel::A != Channel::A {
			None
		} else {
			let offset = Channel::offset_of(channels, Channel::A);
			let size = A::SIZE;
			Some(A::from_slice_mut(&mut data[offset..offset + size]))
		}
	}

	/// Retrieve UV channels as UV color
	///
	/// ```
	/// use document::color::*;
	/// let color = UV::new(1., 2.);
	/// let buffer = color.to_slice();
	/// assert!(Channel::uv(Channel::UV, buffer).is_some());
	/// assert_eq!(Channel::uv(Channel::UV, buffer).unwrap(), &color);
	/// ```
	pub fn uv(channels: Channel, data: &[u8]) -> Option<&UV> {
		if channels & Channel::UV != Channel::UV {
			None
		} else {
			let offset = Channel::offset_of(channels, Channel::UV);
			let size = UV::SIZE;
			Some(UV::from_slice(&data[offset..offset + size]))
		}
	}

	/// Retrieve mutable UV channels as UV color
	///
	/// ```
	/// use document::color::*;
	/// let mut color = UV::new(1., 2.);
	/// let mut buffer = color.to_slice_mut();
	/// assert!(Channel::uv_mut(Channel::UV, buffer).is_some());
	/// assert_eq!(Channel::uv_mut(Channel::UV, buffer).unwrap(), &UV::new(1., 2.));
	/// ```
	pub fn uv_mut(channels: Channel, data: &mut [u8]) -> Option<&mut UV> {
		if channels & Channel::UV != Channel::UV {
			None
		} else {
			let offset = Channel::offset_of(channels, Channel::UV);
			let size = UV::SIZE;
			Some(UV::from_slice_mut(&mut data[offset..offset + size]))
		}
	}

	/// Retrieve XYZ channels as XYZ color
	///
	/// ```
	/// use document::color::*;
	/// let color = XYZ::new(1., 2., 3.);
	/// let buffer = color.to_slice();
	/// assert!(Channel::xyz(Channel::XYZ, buffer).is_some());
	/// assert_eq!(Channel::xyz(Channel::XYZ, buffer).unwrap(), &color);
	/// ```
	pub fn xyz(channels: Channel, data: &[u8]) -> Option<&XYZ> {
		if channels & Channel::XYZ != Channel::XYZ {
			None
		} else {
			let offset = Channel::offset_of(channels, Channel::XYZ);
			let size = XYZ::SIZE;
			Some(XYZ::from_slice(&data[offset..offset + size]))
		}
	}

	/// Retrieve mutable XYZ channels as XYZ color
	///
	/// ```
	/// use document::color::*;
	/// let mut color = XYZ::new(1., 2., 3.);
	/// let mut buffer = color.to_slice_mut();
	/// assert!(Channel::xyz_mut(Channel::XYZ, buffer).is_some());
	/// assert_eq!(Channel::xyz_mut(Channel::XYZ, buffer).unwrap(), &XYZ::new(1., 2., 3.));
	/// ```
	pub fn xyz_mut(channels: Channel, data: &mut [u8]) -> Option<&mut XYZ> {
		if channels & Channel::XYZ != Channel::XYZ {
			None
		} else {
			let offset = Channel::offset_of(channels, Channel::XYZ);
			let size = XYZ::SIZE;
			Some(XYZ::from_slice_mut(&mut data[offset..offset + size]))
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::prelude::*;

	#[test]
	fn test_channels() {
		let channels = Channel::I | Channel::RGB | Channel::A | Channel::UV;
		let buffer: &[u8] = &[1, 1, 2, 3, 2, 0, 0, 128, 63, 0, 0, 0, 64];
		assert_eq!(Channel::size_of(channels), 13);
		assert_eq!(Channel::i(channels, &buffer).unwrap(), &I::new(1));
		assert_eq!(Channel::rgb(channels, &buffer).unwrap(), &RGB::new(1, 2, 3));
		assert_eq!(Channel::a(channels, &buffer).unwrap(), &A::new(2));
		assert_eq!(Channel::uv(channels, &buffer).unwrap(), &UV::new(1., 2.));
		assert!(Channel::xyz(channels, &buffer).is_none());

		let mut buffer: &mut [u8] = &mut [1, 1, 2, 3, 2, 0, 0, 128, 63, 0, 0, 0, 64];
		assert_eq!(Channel::size_of(channels), 13);
		assert_eq!(Channel::i_mut(channels, &mut buffer).unwrap(), &I::new(1));
		assert_eq!(
			Channel::rgb_mut(channels, &mut buffer).unwrap(),
			&RGB::new(1, 2, 3)
		);
		assert_eq!(Channel::a_mut(channels, &mut buffer).unwrap(), &A::new(2));
		assert_eq!(
			Channel::uv_mut(channels, &mut buffer).unwrap(),
			&UV::new(1., 2.)
		);
		assert!(Channel::xyz_mut(channels, &mut buffer).is_none());
	}

	#[test]
	fn test_channels_mut() {
		let channels = Channel::I | Channel::RGB | Channel::A | Channel::UV;
		let buffer: &mut [u8] = &mut [1, 1, 2, 3, 2, 0, 0, 128, 63, 0, 0, 0, 64];
		{
			let mut rgb = Channel::rgb_mut(channels, buffer).unwrap();
			rgb.r = 4;
			assert_eq!(rgb, &RGB::new(4, 2, 3));
		}
		{
			let mut uv = Channel::uv_mut(channels, buffer).unwrap();
			uv.u = 3.;
			assert_eq!(uv, &UV::new(3., 2.));
		}
		assert_eq!(buffer, &[1, 4, 2, 3, 2, 0, 0, 64, 64, 0, 0, 0, 64]);
	}
}

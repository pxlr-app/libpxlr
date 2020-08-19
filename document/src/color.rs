use crate::prelude::*;
use bitflags::bitflags;
use nom::number::complete::{le_f32, le_u8};
use std::fmt::Debug;

trait Color {
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
				unsafe { std::slice::from_raw_parts(self as *const $name as *const u8, std::mem::size_of::<$name>()) }
			}

			fn to_slice_mut(&mut self) -> &mut [u8] {
				unsafe { std::slice::from_raw_parts_mut(self as *mut $name as *mut u8, std::mem::size_of::<$name>()) }
			}

			fn from_slice(slice: &[u8]) -> &Self {
				assert_eq!(slice.len(), $name::SIZE);
				unsafe { &*(slice.as_ptr() as *const $name) }
			}

			fn from_slice_mut(slice: &mut [u8]) -> &mut Self {
				assert_eq!(slice.len(), $name::SIZE);
				unsafe { &mut *(slice.as_ptr() as *mut $name) }
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

bitflags! {
	#[derive(Serialize, Deserialize)]
	pub struct Channel: u8 {
		const I     = 0b00000001;
		const RGB 	= 0b00000010;
		const A		= 0b00000100;
		const UV 	= 0b00001000;
		const XYZ 	= 0b00010000;
	}
}

impl Channel {
	/// Length of the entire channel
	pub fn len(&self) -> usize {
		let mut len = 0usize;
		let bits = self.bits();
		if bits & Channel::I.bits > 0 {
			len += I::SIZE;
		}
		if bits & Channel::RGB.bits > 0 {
			len += RGB::SIZE;
		}
		if bits & Channel::A.bits > 0 {
			len += A::SIZE;
		}
		if bits & Channel::UV.bits > 0 {
			len += UV::SIZE;
		}
		if bits & Channel::XYZ.bits > 0 {
			len += XYZ::SIZE;
		}
		len
	}

	/// Retrive the offset to iterate from channel to channel
	///
	/// Given a composite channel RGBA, retrive the offset to A.
	/// Offset is equal to the length of RGB.
	///
	/// In a composite channel of RGBAXYZ, retrieve the offset to XYZ.
	/// Offset is equal to the length of RGB + A.
	///
	/// ```
	/// use document::color::Channel;
	/// let composite = Channel::RGB | Channel::A | Channel::XYZ;
	/// assert_eq!(composite.offset(Channel::XYZ), Channel::RGB.len() + Channel::A.len());
	/// ```
	///
	pub fn offset(&self, channel: Channel) -> usize {
		let mut offset = 0usize;
		let bits = self.bits();
		for chan in [
			Channel::I,
			Channel::RGB,
			Channel::A,
			Channel::UV,
			Channel::XYZ,
		]
		.iter()
		{
			if &channel == chan {
				return offset;
			}
			if bits & chan.bits > 0 {
				offset += chan.len();
			}
		}
		0
	}
}

impl parser::Parse for Channel {
	fn parse(bytes: &[u8]) -> nom::IResult<&[u8], Channel> {
		let (bytes, bits) = le_u8(bytes)?;
		Ok((bytes, Channel::from_bits(bits).unwrap()))
	}
}

impl parser::Write for Channel {
	fn write(&self, writer: &mut dyn io::Write) -> io::Result<usize> {
		writer.write_all(&self.bits().to_le_bytes())?;
		Ok(1)
	}
}

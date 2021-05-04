use crate::*;
use serde::{Deserialize, Serialize};

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
	Mismatch(Channel, Channel),
}

impl std::error::Error for ChannelError {}

impl std::fmt::Display for ChannelError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			ChannelError::NotFound(chan) => write!(f, "Channel {} not found", chan),
			ChannelError::Mismatch(a, b) => write!(f, "Channel mismatch {} != {}", a, b),
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

#[cfg(test)]
mod tests {
	use super::*;

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
}

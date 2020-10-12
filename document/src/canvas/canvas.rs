use crate::prelude::*;
use bytes::Bytes;
use rstar::{Envelope, Point, PointDistance, RTree, RTreeObject, AABB};
use std::ops::Index;

#[derive(Debug, Clone)]
struct CanvasBrush {
	pub position: Vec2<i32>,
	pub stencil: Stencil,
}

#[derive(Debug)]
struct CanvasNode {
	pub brush: Arc<CanvasBrush>,
}

impl RTreeObject for CanvasNode {
	type Envelope = AABB<[i32; 2]>;

	fn envelope(&self) -> Self::Envelope {
		AABB::from_corners(
			[self.brush.position.x, self.brush.position.y],
			[
				self.brush.position.x + self.brush.stencil.size.w as i32,
				self.brush.position.y + self.brush.stencil.size.h as i32,
			],
		)
	}
}

impl PointDistance for CanvasNode {
	fn distance_2(
		&self,
		point: &<Self::Envelope as Envelope>::Point,
	) -> <<Self::Envelope as Envelope>::Point as Point>::Scalar {
		let min = (self as &dyn RTreeObject<Envelope = AABB<[i32; 2]>>)
			.envelope()
			.min_point(point);
		let sub = [min[0] - point[0], min[1] - point[1]];
		sub[0] * sub[0] + sub[1] * sub[1]
	}

	fn contains_point(&self, point: &<Self::Envelope as Envelope>::Point) -> bool {
		(self as &dyn RTreeObject<Envelope = AABB<[i32; 2]>>)
			.envelope()
			.contains_point(point)
	}

	fn distance_2_if_less_or_equal(
		&self,
		point: &<Self::Envelope as Envelope>::Point,
		max_distance_2: <<Self::Envelope as Envelope>::Point as Point>::Scalar,
	) -> Option<<<Self::Envelope as Envelope>::Point as Point>::Scalar> {
		let distance_2 = self.distance_2(point);
		if distance_2 <= max_distance_2 {
			Some(distance_2)
		} else {
			None
		}
	}
}

#[derive(Debug)]
pub enum CanvasError {
	ChannelMismatch,
	RegionNotContained,
}

impl std::error::Error for CanvasError {}

impl std::fmt::Display for CanvasError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match *self {
			CanvasError::ChannelMismatch => write!(f, "Channel mismatch."),
			CanvasError::RegionNotContained => write!(f, "Region not contained in this Canvas"),
		}
	}
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum FlipAxis {
	Horizontal,
	Vertical,
	Both,
}

#[derive(Debug, Clone)]
pub struct Canvas {
	pub size: Extent2<u32>,
	pub channels: Channel,
	empty_pixel: Vec<u8>,
	rtree: Arc<RTree<CanvasNode>>,
	brushes: Vec<Arc<CanvasBrush>>,
}

impl Canvas {
	/// Create an empty canvas of specific size
	pub fn new(size: Extent2<u32>, channels: Channel) -> Self {
		Canvas {
			size,
			channels,
			empty_pixel: channels.default_pixel(),
			brushes: Vec::new(),
			rtree: Arc::new(RTree::new()),
		}
	}

	/// Create a canvas from pixel buffer
	pub fn from_buffer(size: Extent2<u32>, channels: Channel, data: Vec<u8>) -> Self {
		let stencil = Stencil::from_buffer(size, channels, data);
		Self::from_stencil(stencil)
	}

	/// Create a canvas from a stencil
	pub fn from_stencil(stencil: Stencil) -> Self {
		let size = stencil.size.clone();
		Self::new(size, stencil.channels)
			.apply_stencil(Vec2::new(0, 0), stencil)
			.unwrap()
	}

	/// Apply a stencil on this canvas
	pub fn apply_stencil(
		&self,
		position: Vec2<i32>,
		stencil: Stencil,
	) -> Result<Canvas, CanvasError> {
		self.apply_stencil_with_blend(position, stencil, Blend::Normal, Compose::Lighter)
	}

	/// Apply a stencil on this canvas by blending the stencil on top
	/// of previous stencils.
	pub fn apply_stencil_with_blend(
		&self,
		position: Vec2<i32>,
		mut stencil: Stencil,
		blend_mode: Blend,
		compose_op: Compose,
	) -> Result<Canvas, CanvasError> {
		if self.channels != stencil.channels {
			return Err(CanvasError::ChannelMismatch);
		}
		let mut cloned = self.clone();
		for (x, y, dst) in stencil.iter_mut() {
			for node in self.rtree.locate_all_at_point(&[x as i32, y as i32]) {
				let x = x as i32 - node.brush.position.x;
				let y = y as i32 - node.brush.position.y;
				if x >= 0 && y >= 0 {
					if let Some(bck) = node.brush.stencil.try_get(x as u32, y as u32) {
						let frt = unsafe { std::mem::transmute::<&mut [u8], &[u8]>(dst) };
						blend_pixel_into(blend_mode, compose_op, self.channels, frt, bck, dst);
					}
				}
			}
		}
		cloned
			.brushes
			.push(Arc::new(CanvasBrush { position, stencil }));
		cloned.rebuild_rtree_from_brushes();
		Ok(cloned)
	}

	/// Rebuild RTree from brushes
	fn rebuild_rtree_from_brushes(&mut self) {
		self.rtree = Arc::new(RTree::bulk_load(
			self.brushes
				.iter()
				.map(|brush| CanvasNode {
					brush: brush.clone(),
				})
				.collect::<Vec<_>>(),
		));
	}

	/// Iterate over each pixel of this canvas
	pub fn iter(&self) -> CanvasIterator {
		CanvasIterator {
			canvas: self,
			region: Rect::new(0, 0, self.size.w, self.size.h),
			index: 0,
		}
	}

	/// Iterate part of the canvas
	pub fn iter_region(&self, region: Rect<u32, u32>) -> Result<CanvasIterator, CanvasError> {
		if region.x + region.w > self.size.w || region.y + region.h > self.size.h {
			Err(CanvasError::RegionNotContained)
		} else {
			Ok(CanvasIterator {
				canvas: self,
				region,
				index: 0,
			})
		}
	}

	/// Copy canvas to new Bytes buffer
	///
	/// ```
	/// use document::prelude::*;
	/// let canvas = Canvas::from_buffer(Extent2::new(2, 2), Channel::A, vec![1u8, 2, 3, 4]);
	/// let bytes = canvas.copy_to_bytes();
	/// assert_eq!(&bytes[..], &[1u8, 2, 3, 4][..]);
	/// ```
	pub fn copy_to_bytes(&self) -> Bytes {
		Bytes::from(self.iter().flatten().map(|b| *b).collect::<Vec<u8>>())
	}

	/// Copy region to new Bytes buffer
	///
	/// ```
	/// use document::prelude::*;
	/// let canvas = Canvas::from_buffer(Extent2::new(2, 2), Channel::A, vec![1u8, 2, 3, 4]);
	/// let bytes = canvas.copy_region_to_bytes(Rect::new(1, 0, 1, 2)).unwrap();
	/// assert_eq!(&bytes[..], &[2u8, 4][..]);
	/// ```
	pub fn copy_region_to_bytes(&self, region: Rect<u32, u32>) -> Result<Bytes, CanvasError> {
		Ok(Bytes::from(
			self.iter_region(region)?
				.flatten()
				.map(|b| *b)
				.collect::<Vec<u8>>(),
		))
	}

	/// Allocate a copy of this canvas
	pub fn copy(&self) -> Self {
		Self::from_buffer(self.size, self.channels, self.copy_to_bytes().to_vec())
	}

	/// Resize canvas
	pub fn resize(&self, size: Extent2<u32>, interpolation: Interpolation) -> Self {
		use math::Vec3;
		let mut resized = vec![0u8; size.w as usize * size.h as usize * self.channels.size()];
		let transform = Mat3::scaling_3d(Vec3::new(
			size.w as f32 / self.size.w as f32,
			size.h as f32 / self.size.h as f32,
			1.,
		));
		transform_into(
			&transform,
			&interpolation,
			&size,
			self.channels,
			self,
			&mut resized[..],
		);
		Self::from_buffer(size, self.channels, resized)
	}

	/// Crop canvas
	///
	/// ```
	/// use document::prelude::*;
	/// let canvas = Canvas::from_buffer(Extent2::new(2, 2), Channel::A, vec![1u8, 2, 3, 4]);
	/// let canvas = canvas.crop(Rect::new(1, 0, 1, 2));
	/// let bytes = canvas.copy_to_bytes();
	/// assert_eq!(&bytes[..], &[2u8, 4][..]);
	/// ```
	pub fn crop(&self, region: Rect<i32, u32>) -> Self {
		let mut canvas = self.clone();
		let outer: Rect<i32, i32> = Rect::new(region.x, region.y, region.w as i32, region.h as i32);

		canvas.size = region.extent();
		canvas.brushes = canvas
			.brushes
			.drain(..)
			.filter_map(|stencil| {
				let inner = Rect::new(
					stencil.position.x as i32,
					stencil.position.y as i32,
					stencil.stencil.size.w as i32,
					stencil.stencil.size.h as i32,
				);
				if outer.contains_rect(inner) || outer.collides_with_rect(inner) {
					let mut cloned = (*stencil).clone();
					cloned.position.x -= outer.x;
					cloned.position.y -= outer.y;
					Some(Arc::new(cloned))
				} else {
					None
				}
			})
			.collect();
		canvas.rebuild_rtree_from_brushes();
		canvas
	}

	/// Flip canvas
	///
	/// ```
	/// use document::prelude::*;
	/// let canvas = Canvas::from_buffer(Extent2::new(2, 2), Channel::A, vec![1u8, 2, 3, 4]);
	/// let canvas = canvas.flip(FlipAxis::Vertical);
	/// let bytes = canvas.copy_to_bytes();
	/// assert_eq!(&bytes[..], &[3u8, 4, 1, 2][..]);
	///
	/// let canvas = Canvas::from_buffer(Extent2::new(2, 2), Channel::A, vec![1u8, 2, 3, 4]);
	/// let canvas = canvas.flip(FlipAxis::Horizontal);
	/// let bytes = canvas.copy_to_bytes();
	/// assert_eq!(&bytes[..], &[2u8, 1, 4, 3][..]);
	///
	/// let canvas = Canvas::from_buffer(Extent2::new(2, 2), Channel::A, vec![1u8, 2, 3, 4]);
	/// let canvas = canvas.flip(FlipAxis::Both);
	/// let bytes = canvas.copy_to_bytes();
	/// assert_eq!(&bytes[..], &[4u8, 3, 2, 1][..]);
	/// ```
	pub fn flip(&self, axis: FlipAxis) -> Self {
		use math::Vec3;
		let mut resized =
			vec![0u8; self.size.w as usize * self.size.h as usize * self.channels.size()];
		let (centered, scaling) = match axis {
			FlipAxis::Horizontal => (
				Vec2::new(self.size.w as f32 / -2f32 + 0.5, 0.),
				Vec3::new(-1., 1., 1.),
			),
			FlipAxis::Vertical => (
				Vec2::new(0., self.size.h as f32 / -2f32 + 0.5),
				Vec3::new(1., -1., 1.),
			),
			FlipAxis::Both => (
				Vec2::new(
					self.size.w as f32 / -2f32 + 0.5,
					self.size.h as f32 / -2f32 + 0.5,
				),
				Vec3::new(-1., -1., 1.),
			),
		};
		let transform = Mat3::translation_2d(centered)
			.scaled_3d(scaling)
			.translated_2d(-centered);
		transform_into(
			&transform,
			&Interpolation::Nearest,
			&self.size,
			self.channels,
			self,
			&mut resized[..],
		);
		Self::from_buffer(self.size, self.channels, resized)
	}

	/// Rotate canvas
	///
	/// ```
	/// use document::prelude::*;
	/// let canvas = Canvas::from_buffer(Extent2::new(2, 2), Channel::A, vec![1u8, 2, 3, 4]);
	/// let canvas = canvas.rotate(90. * std::f32::consts::PI / 180., Interpolation::Nearest);
	/// let bytes = canvas.copy_to_bytes();
	/// assert_eq!(&bytes[..], &[3u8, 1, 4, 2][..]);
	/// ```
	pub fn rotate(&self, radians: f32, interpolation: Interpolation) -> Self {
		use math::Vec3;
		let size =
			Mat3::rotation_z(radians) * Vec3::new(self.size.w as f32, self.size.h as f32, 1.);
		let size = Extent2::new(size.x.round().abs() as u32, size.y.round().abs() as u32);
		let mut resized = vec![0u8; size.w as usize * size.h as usize * self.channels.size()];
		let centered = Vec2::new(
			self.size.w as f32 / -2f32 + 0.5,
			self.size.h as f32 / -2f32 + 0.5,
		);
		let transform = Mat3::translation_2d(centered)
			.rotated_z(radians)
			.translated_2d(-centered);
		transform_into(
			&transform,
			&interpolation,
			&size,
			self.channels,
			self,
			&mut resized[..],
		);
		Self::from_buffer(self.size, self.channels, resized)
	}
}

pub struct CanvasIterator<'a> {
	canvas: &'a Canvas,
	region: Rect<u32, u32>,
	index: usize,
}

impl<'a> Iterator for CanvasIterator<'a> {
	type Item = &'a Pixel;

	fn next(&mut self) -> Option<&'a Pixel> {
		if self.index < (self.region.w * self.region.h) as usize {
			let x = self.index % self.region.w as usize + self.region.x as usize;
			let y = self.index / self.region.w as usize + self.region.y as usize;
			self.index += 1;
			return Some(&self.canvas[(x as u32, y as u32)]);
		}
		return None;
	}
}

impl<'a> IntoIterator for &'a Canvas {
	type Item = &'a Pixel;
	type IntoIter = CanvasIterator<'a>;

	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

impl Index<(u32, u32)> for Canvas {
	type Output = Pixel;

	fn index(&self, index: (u32, u32)) -> &Self::Output {
		let index = (index.0 + self.size.w * index.1) as usize;
		self.index(index)
	}
}

impl Index<usize> for Canvas {
	type Output = Pixel;

	fn index(&self, index: usize) -> &Self::Output {
		let x = index as u32 % self.size.w;
		let y = index as u32 / self.size.w;
		if let Some(node) = self.rtree.locate_at_point(&[x as i32, y as i32]) {
			let x = x as i32 - node.brush.position.x;
			let y = y as i32 - node.brush.position.y;
			if x >= 0 && y >= 0 {
				if let Some(pixel) = node.brush.stencil.try_get(x as u32, y as u32) {
					return pixel;
				}
			}
		}
		&self.empty_pixel
	}
}

impl serde::Serialize for Canvas {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		let stencil = Stencil::from_buffer_mask_alpha(
			self.size,
			self.channels,
			self.copy_to_bytes().to_vec(),
		);
		stencil.serialize(serializer)
	}
}

impl<'de> serde::Deserialize<'de> for Canvas {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		let stencil = Stencil::deserialize(deserializer)?;
		Ok(Canvas::from_stencil(stencil))
	}
}

impl parser::Parse for Canvas {
	fn parse(bytes: &[u8]) -> nom::IResult<&[u8], Canvas> {
		let (bytes, stencil) = Stencil::parse(bytes)?;
		Ok((bytes, Canvas::from_stencil(stencil)))
	}
}

impl parser::Write for Canvas {
	fn write(&self, writer: &mut dyn io::Write) -> io::Result<usize> {
		let stencil = Stencil::from_buffer_mask_alpha(
			self.size,
			self.channels,
			self.copy_to_bytes().to_vec(),
		);
		let size = stencil.write(writer)?;
		Ok(size)
	}
}

#[cfg(test)]
mod tests {
	use crate::prelude::*;
	use collections::bitvec;

	#[test]
	fn test_apply_blending() {
		let a = Canvas::new(Extent2::new(2, 2), Channel::RGB | Channel::A);

		let stencil = Stencil {
			size: Extent2::new(2, 2),
			mask: bitvec![Lsb0, u8; 1, 0, 0, 0],
			channels: Channel::RGB | Channel::A,
			data: vec![255u8, 128, 128, 255],
		};

		let b = a.apply_stencil(Vec2::new(0, 0), stencil).unwrap();
		assert_eq!(b[0], [255u8, 128, 128, 255][..]);
		assert_eq!(b[1], [0u8, 0, 0, 0][..]);
		assert_eq!(b[2], [0u8, 0, 0, 0][..]);
		assert_eq!(b[3], [0u8, 0, 0, 0][..]);

		let stencil = Stencil {
			size: Extent2::new(2, 2),
			mask: bitvec![Lsb0, u8; 1, 0, 0, 0],
			channels: Channel::RGB | Channel::A,
			data: vec![0u8, 128, 128, 128],
		};

		let c = b
			.apply_stencil_with_blend(Vec2::new(0, 0), stencil, Blend::Normal, Compose::Lighter)
			.unwrap();
		assert_eq!(c[0], [255u8, 192, 192, 255][..]);
		assert_eq!(c[1], [0u8, 0, 0, 0][..]);
		assert_eq!(c[2], [0u8, 0, 0, 0][..]);
		assert_eq!(c[3], [0u8, 0, 0, 0][..]);
	}

	#[test]
	fn test_index() {
		let a = Canvas::new(Extent2::new(2, 2), Channel::A);

		let stencil = Stencil {
			size: Extent2::new(2, 2),
			mask: bitvec![Lsb0, u8; 1, 0, 0, 1],
			channels: Channel::A,
			data: vec![8u8, 9],
		};

		let b = a.apply_stencil(Vec2::new(0, 0), stencil).unwrap();
		assert_eq!(b[0], [8u8][..]);
		assert_eq!(b[1], [0u8][..]);
		assert_eq!(b[2], [0u8][..]);
		assert_eq!(b[3], [9u8][..]);

		let stencil = Stencil {
			size: Extent2::new(2, 2),
			mask: bitvec![Lsb0, u8; 0, 1, 1, 0],
			channels: Channel::A,
			data: vec![11u8, 12],
		};

		let c = a.apply_stencil(Vec2::new(0, 0), stencil).unwrap();
		assert_eq!(c[0], [0u8][..]);
		assert_eq!(c[1], [11u8][..]);
		assert_eq!(c[2], [12u8][..]);
		assert_eq!(c[3], [0u8][..]);
	}

	#[test]
	fn test_iter() {
		let a = Canvas::new(Extent2::new(2, 2), Channel::A);
		let stencil = Stencil {
			size: Extent2::new(2, 2),
			mask: bitvec![Lsb0, u8; 1, 1, 1, 1],
			channels: Channel::A,
			data: vec![1u8, 2, 3, 4],
		};
		let b = a.apply_stencil(Vec2::new(0, 0), stencil).unwrap();

		let mut i = b.iter();
		assert_eq!(i.next(), Some(&[1u8][..]));
		assert_eq!(i.next(), Some(&[2u8][..]));
		assert_eq!(i.next(), Some(&[3u8][..]));
		assert_eq!(i.next(), Some(&[4u8][..]));
		assert_eq!(i.next(), None);

		let mut i = b.iter_region(Rect::new(1, 0, 1, 2)).unwrap();
		assert_eq!(i.next(), Some(&[2u8][..]));
		assert_eq!(i.next(), Some(&[4u8][..]));
		assert_eq!(i.next(), None);
	}
}

use crate::Stencil;
use color::{Blending, Channel, ChannelError, Compositing};
use rstar::{Envelope, Point, PointDistance, RTree, RTreeObject, AABB};
use std::sync::Arc;
use vek::geom::repr_c::Rect;

#[derive(Debug, Clone)]
pub struct Canvas {
	channel: Channel,
	empty_pixel: Vec<u8>,
	rtree: Arc<RTree<StencilObject>>,
	stencils: Vec<Arc<Stencil>>,
}

#[derive(Debug)]
struct StencilObject {
	stencil: Arc<Stencil>,
}

impl RTreeObject for StencilObject {
	type Envelope = AABB<[i32; 2]>;

	fn envelope(&self) -> Self::Envelope {
		let Rect { x, y, w, h } = self.stencil.bounds();
		AABB::from_corners([x, y], [x + w, y + h])
	}
}

impl PointDistance for StencilObject {
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
	ChannelError(ChannelError),
}

impl std::error::Error for CanvasError {}

impl std::fmt::Display for CanvasError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match *self {
			CanvasError::ChannelError(err) => write!(f, "{}", err),
		}
	}
}

impl From<ChannelError> for CanvasError {
	fn from(error: ChannelError) -> Self {
		CanvasError::ChannelError(error)
	}
}

impl Canvas {
	/// Retrieve channel
	pub fn channel(&self) -> Channel {
		self.channel
	}

	/// Retrieve stencils
	pub fn stencils(&self) -> &Vec<Arc<Stencil>> {
		&self.stencils
	}

	/// Create a canvas from raw part
	pub unsafe fn from_raw_parts(channel: Channel, stencils: Vec<Arc<Stencil>>) -> Self {
		let mut canvas = Canvas {
			channel,
			empty_pixel: channel.default_pixel(),
			rtree: Arc::new(RTree::new()),
			stencils,
		};
		canvas.rebuild_rtree_from_stencils();
		canvas
	}

	/// Create an empty canvas of specific size
	pub fn new(channel: Channel) -> Self {
		unsafe { Canvas::from_raw_parts(channel, vec![]) }
	}

	/// Create a canvas from a stencil
	pub fn from_stencil(stencil: Stencil) -> Self {
		Self::new(stencil.channel()).apply_stencil(stencil).unwrap()
	}

	/// Apply a stencil on this canvas
	pub fn apply_stencil(&self, stencil: Stencil) -> Result<Canvas, CanvasError> {
		self.apply_stencil_with_blend(stencil, Blending::Normal, Compositing::Lighter)
	}

	/// Apply a stencil on this canvas by blending the stencil on top
	/// of previous stencils.
	pub fn apply_stencil_with_blend(
		&self,
		stencil: Stencil,
		blend_mode: Blending,
		compose_op: Compositing,
	) -> Result<Canvas, CanvasError> {
		if self.channel != stencil.channel() {
			return Err(CanvasError::ChannelError(ChannelError::Mismatch(
				self.channel,
				stencil.channel(),
			)));
		}
		let channel = self.channel;
		let mut stencils: Vec<Arc<Stencil>> = Vec::with_capacity(self.stencils.len());
		let bounds = stencil.bounds();
		let mut merged = false;
		for old_stencil in self.stencils.iter() {
			let old_bounds = old_stencil.bounds();
			if bounds.contains_rect(old_bounds) || bounds.collides_with_rect(old_bounds) {
				let new_stencil = Stencil::merge(old_stencil, &stencil, blend_mode, compose_op);
				stencils.push(Arc::new(new_stencil));
				merged = true;
			} else {
				stencils.push(old_stencil.clone());
			}
		}
		if !merged {
			stencils.push(Arc::new(stencil));
		}

		let mut canvas = Canvas {
			channel,
			empty_pixel: channel.default_pixel(),
			rtree: Arc::new(RTree::new()),
			stencils,
		};
		canvas.rebuild_rtree_from_stencils();
		Ok(canvas)
	}

	/// Rebuild RTree from brushes
	fn rebuild_rtree_from_stencils(&mut self) {
		self.rtree = Arc::new(RTree::bulk_load(
			self.stencils
				.iter()
				.map(|stencil| StencilObject {
					stencil: stencil.clone(),
				})
				.collect::<Vec<_>>(),
		));
	}

	/// Try to retrieve a pixel at coordinate
	pub fn try_get(&self, x: i32, y: i32) -> Option<&[u8]> {
		if let Some(node) = self.rtree.locate_at_point(&[x, y]) {
			node.stencil.try_get(x, y)
		} else {
			None
		}
	}

	/// Retrieve canvas bounds
	pub fn bounds(&self) -> Rect<i32, i32> {
		if self.stencils.len() == 0 {
			Rect::new(0, 0, 0, 0)
		} else {
			let aabb = self.rtree.root().envelope();
			let lower = aabb.lower();
			let upper = aabb.upper();
			Rect::new(lower[0], lower[1], upper[0] - lower[0], upper[1] - lower[1])
		}
	}

	/// Iterate over each pixel of this canvas
	pub fn iter(&self) -> CanvasIterator {
		CanvasIterator {
			canvas: self,
			region: self.bounds(),
			index: 0,
		}
	}

	/// Iterate part of the canvas
	pub fn iter_region(&self, region: Rect<i32, i32>) -> CanvasIterator {
		CanvasIterator {
			canvas: self,
			region,
			index: 0,
		}
	}

	/// Allocate a copy of this canvas
	pub fn copy_to_stencil(&self) -> Stencil {
		Stencil::from_buffer(
			self.bounds(),
			self.channel,
			self.iter().flatten().map(|b| *b).collect::<Vec<u8>>(),
		)
	}

	/// Crop canvas
	pub fn crop(&self, region: Rect<i32, i32>) -> Self {
		let mut canvas = self.clone();
		canvas.stencils = canvas
			.stencils
			.drain(..)
			.filter_map(|stencil| {
				if region.contains_rect(stencil.bounds())
					|| region.collides_with_rect(stencil.bounds())
				{
					Some(stencil.clone())
				} else {
					None
				}
			})
			.collect();
		canvas.rebuild_rtree_from_stencils();
		canvas
	}
}

impl std::ops::Index<(i32, i32)> for Canvas {
	type Output = [u8];

	fn index(&self, index: (i32, i32)) -> &Self::Output {
		let (x, y) = index;
		if let Some(pixel) = self.try_get(x, y) {
			pixel
		} else {
			&self.empty_pixel
		}
	}
}

pub struct CanvasIterator<'canvas> {
	canvas: &'canvas Canvas,
	region: Rect<i32, i32>,
	index: usize,
}

impl<'canvas> Iterator for CanvasIterator<'canvas> {
	type Item = &'canvas [u8];

	fn next(&mut self) -> Option<&'canvas [u8]> {
		if self.index < (self.region.w * self.region.h) as usize {
			let x = self.index as i32 % self.region.w + self.region.x;
			let y = self.index as i32 / self.region.w + self.region.y;
			self.index += 1;
			return Some(&self.canvas[(x, y)]);
		}
		return None;
	}
}

impl<'canvas> IntoIterator for &'canvas Canvas {
	type Item = &'canvas [u8];
	type IntoIter = CanvasIterator<'canvas>;

	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn default_canvas() {
		let a = Canvas::new(Channel::default());
		assert_eq!(a.bounds(), Rect::new(0, 0, 0, 0));
	}

	#[test]
	fn apply_stencil() {
		let a = Canvas::new(Channel::Lumaa);
		let stencil = Stencil::from_buffer_mask_alpha(
			Rect::new(0, 0, 2, 2),
			Channel::Lumaa,
			vec![1, 255, 0, 0, 0, 0, 4, 1],
		);
		let b = a.apply_stencil(stencil).unwrap();
		assert_eq!(b.bounds(), Rect::new(0, 0, 2, 2));

		let a = Canvas::from_stencil(Stencil::from_buffer(
			Rect::new(0, 0, 4, 4),
			Channel::Lumaa,
			vec![
				1, 255, 2, 255, 3, 255, 4, 255, 5, 255, 6, 255, 7, 255, 8, 255, 9, 255, 10, 255,
				11, 255, 12, 255, 13, 255, 14, 255, 15, 255, 16, 255,
			],
		));
		let b = a
			.apply_stencil_with_blend(
				Stencil::from_buffer(
					Rect::new(1, 1, 2, 2),
					Channel::Lumaa,
					vec![1, 255, 1, 255, 1, 255, 1, 255],
				),
				Blending::Normal,
				Compositing::SourceOut,
			)
			.unwrap();
		assert_eq!(b.bounds(), Rect::new(0, 0, 4, 4));
		let pixels: Vec<_> = b.iter().flatten().map(|b| *b).collect();
		assert_eq!(
			pixels,
			vec![
				1, 255, 2, 255, 3, 255, 4, 255, 5, 255, 0, 0, 0, 0, 8, 255, 9, 255, 0, 0, 0, 0, 12,
				255, 13, 255, 14, 255, 15, 255, 16, 255
			]
		);
	}

	#[test]
	fn iter() {
		let a = Canvas::new(Channel::Lumaa);
		let stencil = Stencil::from_buffer_mask_alpha(
			Rect::new(0, 0, 2, 2),
			Channel::Lumaa,
			vec![1, 255, 0, 0, 0, 0, 4, 1],
		);
		let b = a.apply_stencil(stencil).unwrap();
		let pixels: Vec<_> = b.iter().flatten().map(|b| *b).collect();
		assert_eq!(pixels, vec![1, 255, 0, 0, 0, 0, 4, 1]);
		let pixels: Vec<_> = b
			.iter_region(Rect::new(1, 0, 1, 2))
			.flatten()
			.map(|b| *b)
			.collect();
		assert_eq!(pixels, vec![0, 0, 4, 1]);
		let pixels: Vec<_> = b
			.iter_region(Rect::new(-10, -10, 2, 2))
			.flatten()
			.map(|b| *b)
			.collect();
		assert_eq!(pixels, vec![0, 0, 0, 0, 0, 0, 0, 0]);
	}

	#[test]
	fn crop() {
		let a = Canvas::from_stencil(Stencil::from_buffer_mask_alpha(
			Rect::new(0, 0, 2, 2),
			Channel::Lumaa,
			vec![1, 255, 0, 0, 0, 0, 4, 1],
		));
		let pixels: Vec<_> = a.iter().flatten().map(|b| *b).collect();
		assert_eq!(pixels, vec![1, 255, 0, 0, 0, 0, 4, 1]);
		let b = a
			.apply_stencil(Stencil::from_buffer_mask_alpha(
				Rect::new(10, 10, 2, 2),
				Channel::Lumaa,
				vec![1, 255, 0, 0, 0, 0, 4, 1],
			))
			.unwrap();
		let c = b.crop(Rect::new(1, 0, 1, 2));
		let pixels: Vec<_> = c.iter().flatten().map(|b| *b).collect();
		assert_eq!(pixels, vec![1, 255, 0, 0, 0, 0, 4, 1]);
	}
}

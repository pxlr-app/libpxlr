use crate::Stencil;
use color::{Blending, Channel, ChannelError, Compositing, Pixel, PixelMut};
use rstar::{Envelope, Point, PointDistance, RTree, RTreeObject, AABB};
use std::sync::Arc;
use vek::geom::repr_c::Rect;

#[derive(Debug, Clone)]
pub struct Canvas {
	pub channel: Channel,
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
		AABB::from_corners([x, y], [w, h])
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
	/// Create an empty canvas of specific size
	pub fn new(channel: Channel) -> Self {
		Canvas {
			channel,
			empty_pixel: channel.default_pixel(),
			rtree: Arc::new(RTree::new()),
			stencils: Vec::new(),
		}
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
		mut stencil: Stencil,
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
		let mut cloned = self.clone();
		for (x, y, dst_buf) in stencil.iter_mut() {
			let mut dst_px = PixelMut::from_buffer_mut(dst_buf, channel);
			for node in self.rtree.locate_all_at_point(&[x, y]) {
				if let Some(bck_buf) = node.stencil.try_get(x, y) {
					let frt_px = Pixel::from_buffer(bck_buf, channel);
					let bck_px = Pixel::from_buffer(bck_buf, channel);
					dst_px.blend(blend_mode, compose_op, &frt_px, &bck_px)?;
				}
			}
		}
		cloned.stencils.push(Arc::new(stencil));
		cloned.rebuild_rtree_from_stencils();
		Ok(cloned)
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
			Rect::new(lower[0], lower[1], upper[0], upper[1])
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
		let pixels: Vec<_> = b.iter().flatten().collect();
		assert_eq!(pixels, vec![&1, &255, &0, &0, &0, &0, &4, &1]);
		let pixels: Vec<_> = b.iter_region(Rect::new(1, 0, 1, 2)).flatten().collect();
		assert_eq!(pixels, vec![&0, &0, &4, &1]);
		let pixels: Vec<_> = b.iter_region(Rect::new(-10, -10, 2, 2)).flatten().collect();
		assert_eq!(pixels, vec![&0, &0, &0, &0, &0, &0, &0, &0]);
	}

	#[test]
	fn crop() {
		let a = Canvas::from_stencil(Stencil::from_buffer_mask_alpha(
			Rect::new(0, 0, 2, 2),
			Channel::Lumaa,
			vec![1, 255, 0, 0, 0, 0, 4, 1],
		));
		let pixels: Vec<_> = a.iter().flatten().collect();
		assert_eq!(pixels, vec![&1, &255, &0, &0, &0, &0, &4, &1]);
		let b = a
			.apply_stencil(Stencil::from_buffer_mask_alpha(
				Rect::new(10, 10, 2, 2),
				Channel::Lumaa,
				vec![1, 255, 0, 0, 0, 0, 4, 1],
			))
			.unwrap();
		let c = b.crop(Rect::new(1, 0, 1, 2));
		let pixels: Vec<_> = c.iter().flatten().collect();
		assert_eq!(pixels, vec![&1, &255, &0, &0, &0, &0, &4, &1]);
	}
}

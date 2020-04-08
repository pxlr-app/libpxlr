use math::{Extent2, Vec2, Lerp};
use std::rc::Rc;
use uuid::Uuid;

use crate::node::*;
use crate::patch::*;
use crate::sprite::*;

pub struct Canvas<T>
where
	T: Default + Clone + Lerp<f32>,
{
	pub id: Uuid,
	pub name: Rc<String>,
	pub size: Rc<Extent2<u32>>,
	pub data: Rc<Vec<T>>,
}

impl<T> Canvas<T>
where
	T: Default + Clone + Lerp<f32>,
{
	pub fn new(id: Option<Uuid>, name: &str, size: Extent2<u32>, data: Vec<T>) -> Canvas<T> {
		Canvas::<T> {
			id: id.or(Some(Uuid::new_v4())).unwrap(),
			name: Rc::new(name.to_owned()),
			size: Rc::new(size),
			data: Rc::new(data),
		}
	}
}

impl<T> std::ops::Index<(u32, u32)> for Canvas<T>
where
	T: Default + Clone + Lerp<f32>,
{
	type Output = T;

	fn index(&self, (x, y): (u32, u32)) -> &T {
		let i = (x * self.size.w + y) as usize;
		&self.data[i]
	}
}

impl<T> Node for Canvas<T>
where
	T: Default + Clone + Lerp<f32>,
{
	fn id(&self) -> Uuid {
		self.id
	}
}

impl<T> Layer for Canvas<T>
where
	T: Default + Clone + Lerp<f32> + 'static,
{
	fn crop(&self, offset: Vec2<u32>, size: Extent2<u32>) -> (CropPatch, Box<dyn PatchImpl>) {
		assert_eq!(size.w + offset.x <= self.size.w, true);
		assert_eq!(size.h + offset.y <= self.size.h, true);
		(
			CropPatch {
				target: self.id,
				offset: offset,
				size: size,
			},
			Box::new(RestoreCanvasPatch::<T> {
				target: self.id,
				name: (*self.name).to_owned(),
				size: (*self.size).clone(),
				data: (*self.data).to_owned(),
			}),
		)
	}
}

impl<'a, T> Renamable<'a> for Canvas<T>
where
	T: Default + Clone + Lerp<f32>,
{
	fn rename(&self, new_name: &'a str) -> (RenamePatch, RenamePatch) {
		(
			RenamePatch {
				target: self.id,
				name: new_name.to_owned(),
			},
			RenamePatch {
				target: self.id,
				name: (*self.name).to_owned(),
			},
		)
	}
}

pub struct RestoreCanvasPatch<T>
where
	T: Clone,
{
	pub target: Uuid,
	pub name: String,
	pub size: Extent2<u32>,
	pub data: Vec<T>,
}

impl<T> Patch for RestoreCanvasPatch<T>
where
	T: Clone,
{
	fn target(&self) -> Uuid {
		self.target
	}
}

impl<T> Patchable for Canvas<T>
where
	T: Default + Clone + Lerp<f32> + 'static,
{
	fn patch(&self, patch: &dyn PatchImpl) -> Option<Box<Self>> {
		if patch.target() == self.id {
			if let Some(patch) = patch.as_any().downcast_ref::<RenamePatch>() {
				return Some(Box::new(Canvas::<T> {
					id: self.id,
					name: Rc::new(patch.name.clone()),
					size: self.size.clone(),
					data: self.data.clone(),
				}));
			} else if let Some(patch) = patch.as_any().downcast_ref::<RestoreCanvasPatch<T>>() {
				return Some(Box::new(Canvas::<T> {
					id: self.id,
					name: Rc::new(patch.name.to_owned()),
					size: Rc::new(patch.size),
					data: Rc::new(patch.data.to_owned()),
				}));
			} else if let Some(patch) = patch.as_any().downcast_ref::<CropPatch>() {
				let data: Vec<T> = vec![Default::default(); (patch.size.w * patch.size.h) as usize];
				return Some(Box::new(Canvas::<T> {
					id: self.id,
					name: self.name.clone(),
					size: Rc::new(patch.size),
					data: Rc::new(data),
				}));
			}
		}
		return None;
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use math::Lerp;
	use image::{ImageBuffer};

	#[derive(Clone)]
	struct RGB(u8, u8, u8);

	impl Default for RGB {
		fn default() -> Self { RGB(0u8, 0u8, 0u8) }
	}

	impl Lerp<f32> for RGB {
		type Output = RGB;
		fn lerp_unclamped_precise(from: Self, to: Self, factor: f32) -> Self {
			RGB(Lerp::lerp_unclamped_precise(from.0, to.0, factor), Lerp::lerp_unclamped_precise(from.1, to.1, factor), Lerp::lerp_unclamped_precise(from.2, to.2, factor))
		}
		fn lerp_unclamped(from: Self, to: Self, factor: f32) -> Self {
			RGB(Lerp::lerp_unclamped(from.0, to.0, factor), Lerp::lerp_unclamped(from.1, to.1, factor), Lerp::lerp_unclamped(from.2, to.2, factor))
		}
	}

	impl From<&RGB> for image::Rgb<u8> {
		fn from(color: &RGB) -> Self {
			image::Rgb([color.0, color.1, color.2])
		}
	}

	#[test]
	fn it_from_buffer() {
		let canvas = Canvas::new(
			None,
			"red",
			Extent2::new(2u32, 2u32),
			vec![RGB(255u8, 0u8, 0u8), RGB(255u8, 0u8, 0u8), RGB(0u8, 255u8, 0u8), RGB(0u8, 255u8, 0u8)],
		);
		assert_eq!(canvas.data.len(), 4);
		let img = ImageBuffer::from_fn(2, 2, |x, y| {
			image::Rgb::from(&canvas[(x, y)])
		});

		img.save("tests/canvas.it_from_buffer.png").unwrap();
	}
}

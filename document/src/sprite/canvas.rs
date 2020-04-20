use math::{Extent2, Lerp, Mat2, Vec2};
use std::iter::FromIterator;
use std::rc::Rc;
use uuid::Uuid;

use crate::node::*;
use crate::patch::*;
use crate::sprite::*;

pub struct Canvas
{
	pub id: Uuid,
	pub name: Rc<String>,
	pub size: Rc<Extent2<u32>>,
	pub data: Rc<Vec<Pixel>>,
}

impl Canvas
{
	pub fn new(id: Option<Uuid>, name: &str, size: Extent2<u32>, data: Vec<Pixel>) -> Canvas {
		Canvas {
			id: id.or(Some(Uuid::new_v4())).unwrap(),
			name: Rc::new(name.to_owned()),
			size: Rc::new(size),
			data: Rc::new(data),
		}
	}

	fn apply_stencil(
		&self,
		offset: Vec2<u32>,
		blend_mode: BlendMode,
		stencil: Stencil,
	) -> (Patch, Patch) {
		assert_eq!(stencil.size.w + offset.x <= self.size.w, true);
		assert_eq!(stencil.size.h + offset.y <= self.size.h, true);
		(
			Patch::ApplyStencil(ApplyStencilPatch {
				target: self.id,
				offset: offset,
				blend_mode: blend_mode,
				stencil: stencil,
			}),
			Patch::Noop
			// Patch::RestoreCanvas(RestoreCanvasPatch::<Pixel> {
			// 	target: self.id,
			// 	name: (*self.name).to_owned(),
			// 	size: (*self.size).clone(),
			// 	data: (*self.data).to_owned(),
			// }),
		)
	}
}

impl std::ops::Index<(u32, u32)> for Canvas
{
	type Output = Pixel;

	fn index(&self, (x, y): (u32, u32)) -> &Pixel {
		let i = (y * self.size.w + x) as usize;
		&self.data[i]
	}
}

impl Node for Canvas
{
	fn id(&self) -> Uuid {
		self.id
	}
}

impl Layer for Canvas
{
	fn crop(&self, offset: Vec2<u32>, size: Extent2<u32>) -> (Patch, Patch) {
		assert_eq!(size.w + offset.x <= self.size.w, true);
		assert_eq!(size.h + offset.y <= self.size.h, true);
		(
			Patch::CropLayer(CropLayerPatch {
				target: self.id,
				offset: offset,
				size: size,
			}),
			Patch::Noop
			// Patch::RestoreCanvas(RestoreCanvasPatch::<T> {
			// 	target: self.id,
			// 	name: (*self.name).to_owned(),
			// 	size: (*self.size).clone(),
			// 	data: (*self.data).to_owned(),
			// }),
		)
	}

	fn resize(
		&self,
		size: Extent2<u32>,
		interpolation: Interpolation,
	) -> (Patch, Patch) {
		(
			Patch::ResizeLayer(ResizeLayerPatch {
				target: self.id,
				size: size,
				interpolation: interpolation,
			}),
			Patch::Noop
			// Patch::RestoreCanvas(RestoreCanvasPatch::<T> {
			// 	target: self.id,
			// 	name: (*self.name).to_owned(),
			// 	size: (*self.size).clone(),
			// 	data: (*self.data).to_owned(),
			// }),
		)
	}
}

impl<'a> Renamable<'a> for Canvas
{
	fn rename(&self, new_name: &'a str) -> (Patch, Patch) {
		(
			Patch::Rename(RenamePatch {
				target: self.id,
				name: new_name.to_owned(),
			}),
			Patch::Rename(RenamePatch {
				target: self.id,
				name: (*self.name).to_owned(),
			}),
		)
	}
}

impl Patchable for Canvas
{
	fn patch(&self, patch: &Patch) -> Option<Box<Self>> {
		if patch.target() == self.id {
			return match patch {
				Patch::Rename(patch) => Some(Box::new(Canvas {
					id: self.id,
					name: Rc::new(patch.name.clone()),
					size: self.size.clone(),
					data: self.data.clone(),
				})),
				// Patch::RestoreCanvasPatch(patch) => Some(Box::new(Canvas::<T> {
				// 	id: self.id,
				// 	name: Rc::new(patch.name.to_owned()),
				// 	size: Rc::new(patch.size),
				// 	data: Rc::new(patch.data.to_owned()),
				// })),
				Patch::CropLayer(patch) => {
					assert_eq!(patch.size.w + patch.offset.x <= self.size.w, true);
					assert_eq!(patch.size.h + patch.offset.y <= self.size.h, true);
					let mut data = vec![Default::default(); (patch.size.w * patch.size.h) as usize];
					for i in 0..data.len() {
						let x = patch.offset.x + ((i as u32) % patch.size.w);
						let y = patch.offset.y + ((i as u32) / patch.size.w);
						data[i] = self[(x, y)];
					}
					Some(Box::new(Canvas {
						id: self.id,
						name: self.name.clone(),
						size: Rc::new(patch.size),
						data: Rc::new(data),
					}))
				},
				Patch::ResizeLayer(patch) => {
					let mut data = vec![Default::default(); (patch.size.w * patch.size.h) as usize];
					patch.interpolation.interpolate(
						&self.size,
						&self.data,
						&patch.size,
						&mut data,
						Mat2::scaling_2d(Vec2::new(
							((self.size.w - 1) as f32) / (patch.size.w as f32),
							((self.size.h - 1) as f32) / (patch.size.h as f32),
						)),
					);
					Some(Box::new(Canvas {
						id: self.id,
						name: self.name.clone(),
						size: Rc::new(patch.size),
						data: Rc::new(data),
					}))
				},
				Patch::ApplyStencil(patch) => {
					let mut data: Vec<Pixel> = Vec::from_iter(self.data.iter().cloned());
					for (x, y, d) in patch.stencil.iter() {
						let x = x + patch.offset.x;
						let y = y + patch.offset.y;
						let i = (x * self.size.h + y) as usize;
						data[i] = Blend::blend(&self.data[i], &d, &patch.blend_mode);
					}
					Some(Box::new(Canvas {
						id: self.id,
						name: self.name.clone(),
						size: self.size.clone(),
						data: Rc::new(data),
					}))
				},
				_ => None
			};
		}
		return None;
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use collections::bitvec;
	use math::{Extent2, Vec2};

	impl Blend for u8 {
		type Output = u8;

		fn blend(from: &u8, to: &u8, mode: &BlendMode) -> u8 {
			match mode {
				BlendMode::Normal => *to,
				BlendMode::Add => from + to,
				BlendMode::Subtract => from - to,
				BlendMode::Multiply => from * to,
				BlendMode::Divide => from / to,
				_ => *to,
			}
		}
	}

	#[test]
	fn from_buffer() {
		let c1 = Canvas::new(
			None,
			"canvas",
			Extent2::new(2u32, 2u32),
			vec![Pixel::I(255), Pixel::I(128), Pixel::I(64), Pixel::I(32)],
		);

		assert_eq!(*c1.data, vec![Pixel::I(255), Pixel::I(128), Pixel::I(64), Pixel::I(32)]);
	}

	#[test]
	fn it_crops() {
		let c1 = Canvas::new(
			None,
			"canvas",
			Extent2::new(2u32, 2u32),
			vec![Pixel::I(255), Pixel::I(128), Pixel::I(64), Pixel::I(32)],
		);

		let (patch, _) = c1.crop(Vec2::new(1, 0), Extent2::new(1, 2));
		let c2 = c1.patch(&patch).unwrap();

		assert_eq!(*c2.data, vec![Pixel::I(128), Pixel::I(32)]);
	}

	#[test]
	fn it_resizes() {
		let c1 = Canvas::new(
			None,
			"canvas",
			Extent2::new(2u32, 2u32),
			vec![Pixel::I(255), Pixel::I(128), Pixel::I(64), Pixel::I(32)],
		);

		let (patch, _) = c1.resize(Extent2::new(4, 4), Interpolation::Nearest);
		let c2 = c1.patch(&patch).unwrap();

		assert_eq!(
			*c2.data,
			vec![Pixel::I(255), Pixel::I(255), Pixel::I(128), Pixel::I(128), Pixel::I(255), Pixel::I(255), Pixel::I(128), Pixel::I(128), Pixel::I(64), Pixel::I(64), Pixel::I(32), Pixel::I(32), Pixel::I(64), Pixel::I(64), Pixel::I(32), Pixel::I(32)]
		);

		let (patch, _) = c1.resize(Extent2::new(4, 4), Interpolation::Bilinear);
		let c2 = c1.patch(&patch).unwrap();
		assert_eq!(
			*c2.data,
			vec![Pixel::I(255), Pixel::I(223), Pixel::I(192), Pixel::I(160), Pixel::I(207), Pixel::I(181), Pixel::I(156), Pixel::I(130), Pixel::I(160), Pixel::I(140), Pixel::I(120), Pixel::I(100), Pixel::I(112), Pixel::I(98), Pixel::I(84), Pixel::I(70)]
		);

		let (patch, _) = c1.resize(Extent2::new(2, 1), Interpolation::Nearest);
		let c2 = c1.patch(&patch).unwrap();

		assert_eq!(*c2.data, vec![Pixel::I(255), Pixel::I(64)]);
	}

	#[test]
	fn it_apply_patch() {
		let c1 = Canvas::new(
			None,
			"canvas",
			Extent2::new(2u32, 2u32),
			vec![Pixel::I(196), Pixel::I(128), Pixel::I(64), Pixel::I(32)],
		);

		let (patch, _) = c1.apply_stencil(
			Vec2::new(0, 0),
			BlendMode::Normal,
			Stencil::from_buffer(Extent2::new(2, 2), &[Pixel::I(255), Pixel::I(255), Pixel::I(255), Pixel::I(255)]),
		);
		let c2 = c1.patch(&patch).unwrap();
		assert_eq!(*c2.data, vec![Pixel::I(255), Pixel::I(255), Pixel::I(255), Pixel::I(255)]);

		let (patch, _) = c1.apply_stencil(
			Vec2::new(0, 0),
			BlendMode::Normal,
			Stencil {
				size: Extent2::new(2, 2),
				mask: bitvec![1, 0, 0, 1],
				data: vec![Pixel::I(255), Pixel::I(255)],
			},
		);
		let c2 = c1.patch(&patch).unwrap();
		assert_eq!(*c2.data, vec![Pixel::I(255), Pixel::I(128), Pixel::I(64), Pixel::I(255)]);
	}
}

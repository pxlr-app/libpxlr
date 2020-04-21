use math::{Extent2, Mat2, Vec2};
use std::iter::FromIterator;
use std::ops::Index;
use std::rc::Rc;
use uuid::Uuid;

use crate::node::Node;
use crate::patch::*;
use crate::sprite::color::*;
use crate::sprite::*;

macro_rules! impl_canvas {
	($name:ident $color:ident $stencil:ident $stencilpatch:ident $patchstencilpatch:ident $restorepatch:ident $patchrestorepatch:ident) => {
		pub struct $name {
			pub id: Uuid,
			pub name: Rc<String>,
			pub size: Rc<Extent2<u32>>,
			pub data: Rc<Vec<$color>>,
		}

		impl $name {
			pub fn new(
				id: Option<Uuid>,
				name: &str,
				size: Extent2<u32>,
				data: Vec<$color>,
			) -> $name {
				$name {
					id: id.or(Some(Uuid::new_v4())).unwrap(),
					name: Rc::new(name.to_owned()),
					size: Rc::new(size),
					data: Rc::new(data),
				}
			}

			pub fn apply_stencil(
				&self,
				offset: Vec2<u32>,
				blend_mode: BlendMode,
				stencil: $stencil,
			) -> (Patch, Patch) {
				assert_eq!(stencil.size.w + offset.x <= self.size.w, true);
				assert_eq!(stencil.size.h + offset.y <= self.size.h, true);
				(
					Patch::$patchstencilpatch($stencilpatch {
						target: self.id,
						offset: offset,
						blend_mode: blend_mode,
						stencil: stencil,
					}),
					Patch::$patchrestorepatch($restorepatch {
						target: self.id,
						name: (*self.name).to_owned(),
						size: (*self.size).clone(),
						data: (*self.data).to_owned(),
					}),
				)
			}
		}

		impl Index<(u32, u32)> for $name {
			type Output = $color;

			fn index(&self, (x, y): (u32, u32)) -> &$color {
				let i = (y * self.size.w + x) as usize;
				&self.data[i]
			}
		}

		impl Node for $name {
			fn id(&self) -> Uuid {
				self.id
			}
		}

		impl Layer for $name {
			fn crop(&self, offset: Vec2<u32>, size: Extent2<u32>) -> (Patch, Patch) {
				assert_eq!(size.w + offset.x <= self.size.w, true);
				assert_eq!(size.h + offset.y <= self.size.h, true);
				(
					Patch::CropLayer(CropLayerPatch {
						target: self.id,
						offset: offset,
						size: size,
					}),
					Patch::$patchrestorepatch($restorepatch {
						target: self.id,
						name: (*self.name).to_owned(),
						size: (*self.size).clone(),
						data: (*self.data).to_owned(),
					}),
				)
			}

			fn resize(&self, size: Extent2<u32>, interpolation: Interpolation) -> (Patch, Patch) {
				(
					Patch::ResizeLayer(ResizeLayerPatch {
						target: self.id,
						size: size,
						interpolation: interpolation,
					}),
					Patch::$patchrestorepatch($restorepatch {
						target: self.id,
						name: (*self.name).to_owned(),
						size: (*self.size).clone(),
						data: (*self.data).to_owned(),
					}),
				)
			}
		}

		impl<'a> Renamable<'a> for $name {
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

		impl Patchable for $name {
			fn patch(&self, patch: &Patch) -> Option<Box<Self>> {
				if patch.target() == self.id {
					return match patch {
						Patch::Rename(patch) => Some(Box::new($name {
							id: self.id,
							name: Rc::new(patch.name.clone()),
							size: self.size.clone(),
							data: self.data.clone(),
						})),
						Patch::CropLayer(patch) => {
							assert_eq!(patch.size.w + patch.offset.x <= self.size.w, true);
							assert_eq!(patch.size.h + patch.offset.y <= self.size.h, true);
							let mut data =
								vec![Default::default(); (patch.size.w * patch.size.h) as usize];
							for i in 0..data.len() {
								let x = patch.offset.x + ((i as u32) % patch.size.w);
								let y = patch.offset.y + ((i as u32) / patch.size.w);
								data[i] = self[(x, y)];
							}
							Some(Box::new($name {
								id: self.id,
								name: self.name.clone(),
								size: Rc::new(patch.size),
								data: Rc::new(data),
							}))
						}
						Patch::$patchrestorepatch(patch) => Some(Box::new($name {
							id: self.id,
							name: Rc::new(patch.name.to_owned()),
							size: Rc::new(patch.size),
							data: Rc::new(patch.data.to_owned()),
						})),
						Patch::ResizeLayer(patch) => {
							let mut data =
								vec![Default::default(); (patch.size.w * patch.size.h) as usize];
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
							Some(Box::new($name {
								id: self.id,
								name: self.name.clone(),
								size: Rc::new(patch.size),
								data: Rc::new(data),
							}))
						}
						Patch::$patchstencilpatch(patch) => {
							let mut data: Vec<$color> = Vec::from_iter(self.data.iter().cloned());
							for (x, y, d) in patch.stencil.iter() {
								let x = x + patch.offset.x;
								let y = y + patch.offset.y;
								let i = (x * self.size.h + y) as usize;
								data[i] = Blend::blend(&self.data[i], &d, &patch.blend_mode);
							}
							Some(Box::new($name {
								id: self.id,
								name: self.name.clone(),
								size: self.size.clone(),
								data: Rc::new(data),
							}))
						}
						_ => None,
					};
				}
				return None;
			}
		}
	};
}

impl_canvas!(CanvasI I StencilI ApplyStencilIPatch ApplyStencilI RestoreLayerCanvasIPatch RestoreLayerCanvasI);
impl_canvas!(CanvasUV UV StencilUV ApplyStencilUVPatch ApplyStencilUV RestoreLayerCanvasUVPatch RestoreLayerCanvasUV);
impl_canvas!(CanvasRGB RGB StencilRGB ApplyStencilRGBPatch ApplyStencilRGB RestoreLayerCanvasRGBPatch RestoreLayerCanvasRGB);
impl_canvas!(CanvasRGBA RGBA StencilRGBA ApplyStencilRGBAPatch ApplyStencilRGBA RestoreLayerCanvasRGBAPatch RestoreLayerCanvasRGBA);
impl_canvas!(CanvasRGBAXYZ RGBAXYZ StencilRGBAXYZ ApplyStencilRGBAXYZPatch ApplyStencilRGBAXYZ RestoreLayerCanvasRGBAXYZPatch RestoreLayerCanvasRGBAXYZ);

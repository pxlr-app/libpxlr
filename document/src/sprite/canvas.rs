use crate::color::*;
use crate::patch::*;
use crate::sprite::*;
use crate::INode;
use math::blend::*;
use math::interpolation::*;
use math::{Extent2, Mat2, Vec2};
use serde::{Deserialize, Serialize};
use std::iter::FromIterator;
use std::ops::Index;
use std::sync::Arc;
use uuid::Uuid;

pub trait ICanvas {
	type Color: IColor;
	type Stencil: IStencil;
}

macro_rules! define_canvas {
	($name:ident, $color:ty, $stencil:ty, $stencilpatch:ident, $patchstencilpatch:ident, $restorepatch:ident, $patchrestorepatch:ident) => {
		#[derive(Debug, Serialize, Deserialize)]
		pub struct $name {
			pub id: Uuid,
			pub is_visible: bool,
			pub is_locked: bool,
			pub name: Arc<String>,
			pub size: Arc<Extent2<u32>>,
			pub albedo: Arc<Vec<$color>>,
			pub alpha: Arc<Vec<Alpha>>,
			pub normal: Arc<Vec<Normal>>,
		}

		impl $name {
			pub fn new(
				id: Option<Uuid>,
				name: &str,
				size: Extent2<u32>,
				albedo: Vec<$color>,
				alpha: Vec<Alpha>,
				normal: Vec<Normal>,
			) -> $name {
				$name {
					id: id.or(Some(Uuid::new_v4())).unwrap(),
					is_visible: true,
					is_locked: false,
					name: Arc::new(name.to_owned()),
					size: Arc::new(size),
					albedo: Arc::new(albedo),
					alpha: Arc::new(alpha),
					normal: Arc::new(normal),
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
						albedo: (*self.albedo).to_owned(),
						alpha: (*self.alpha).to_owned(),
						normal: (*self.normal).to_owned(),
					}),
				)
			}

			pub fn apply_alpha_stencil(
				&self,
				offset: Vec2<u32>,
				blend_mode: BlendMode,
				stencil: StencilAlpha,
			) -> (Patch, Patch) {
				assert_eq!(stencil.size.w + offset.x <= self.size.w, true);
				assert_eq!(stencil.size.h + offset.y <= self.size.h, true);
				(
					Patch::ApplyStencilAlpha(ApplyStencilPatch {
						target: self.id,
						offset: offset,
						blend_mode: blend_mode,
						stencil: stencil,
					}),
					Patch::$patchrestorepatch($restorepatch {
						target: self.id,
						name: (*self.name).to_owned(),
						size: (*self.size).clone(),
						albedo: (*self.albedo).to_owned(),
						alpha: (*self.alpha).to_owned(),
						normal: (*self.normal).to_owned(),
					}),
				)
			}

			pub fn apply_normal_stencil(
				&self,
				offset: Vec2<u32>,
				blend_mode: BlendMode,
				stencil: StencilNormal,
			) -> (Patch, Patch) {
				assert_eq!(stencil.size.w + offset.x <= self.size.w, true);
				assert_eq!(stencil.size.h + offset.y <= self.size.h, true);
				(
					Patch::ApplyStencilNormal(ApplyStencilPatch {
						target: self.id,
						offset: offset,
						blend_mode: blend_mode,
						stencil: stencil,
					}),
					Patch::$patchrestorepatch($restorepatch {
						target: self.id,
						name: (*self.name).to_owned(),
						size: (*self.size).clone(),
						albedo: (*self.albedo).to_owned(),
						alpha: (*self.alpha).to_owned(),
						normal: (*self.normal).to_owned(),
					}),
				)
			}
		}

		impl ICanvas for $name {
			type Color = $color;
			type Stencil = $stencil;
		}

		// impl Index<(u32, u32)> for $name {
		// 	type Output = ($color, Option<Alpha>, Option<Normal>);

		// 	fn index(&self, (x, y): (u32, u32)) -> (&$color, Option<&Alpha>, Option<&Normal>) {
		// 		let i = (y * self.size.w + x) as usize;
		// 		(&self.albedo[i], &self.alpha[i], &self.normal[i])
		// 	}
		// }

		impl ILayer for $name {
			fn crop(
				&self,
				offset: Vec2<u32>,
				size: Extent2<u32>,
			) -> Result<(Patch, Patch), CropLayerError> {
				if size.w == 0 || size.h == 0 {
					Err(CropLayerError::InvalidSize)
				} else if size.w + offset.x > self.size.w || size.h + offset.y > self.size.h {
					Err(CropLayerError::OutsideRegion)
				} else {
					Ok((
						Patch::CropLayer(CropLayerPatch {
							target: self.id,
							offset: offset,
							size: size,
						}),
						Patch::$patchrestorepatch($restorepatch {
							target: self.id,
							name: (*self.name).to_owned(),
							size: (*self.size).clone(),
							albedo: (*self.albedo).to_owned(),
							alpha: (*self.alpha).to_owned(),
							normal: (*self.normal).to_owned(),
						}),
					))
				}
			}

			fn resize(
				&self,
				size: Extent2<u32>,
				interpolation: Interpolation,
			) -> Result<(Patch, Patch), ResizeLayerError> {
				if size.w == 0 || size.h == 0 {
					Err(ResizeLayerError::InvalidSize)
				} else {
					Ok((
						Patch::ResizeLayer(ResizeLayerPatch {
							target: self.id,
							size: size,
							interpolation: interpolation,
						}),
						Patch::$patchrestorepatch($restorepatch {
							target: self.id,
							name: (*self.name).to_owned(),
							size: (*self.size).clone(),
							albedo: (*self.albedo).to_owned(),
							alpha: (*self.alpha).to_owned(),
							normal: (*self.normal).to_owned(),
						}),
					))
				}
			}
		}

		impl INode for $name {
			fn is_visible(&self) -> bool {
				self.is_visible
			}
			fn is_locked(&self) -> bool {
				self.is_locked
			}
		}

		impl<'a> Renamable<'a> for $name {
			fn rename(&self, new_name: &'a str) -> Result<(Patch, Patch), RenameError> {
				if *self.name == new_name {
					Err(RenameError::Unchanged)
				} else {
					Ok((
						Patch::Rename(RenamePatch {
							target: self.id,
							name: new_name.to_owned(),
						}),
						Patch::Rename(RenamePatch {
							target: self.id,
							name: (*self.name).to_owned(),
						}),
					))
				}
			}
		}

		impl IVisible for $name {
			fn set_visibility(&self, visible: bool) -> Result<(Patch, Patch), SetVisibilityError> {
				if self.is_visible == visible {
					Err(SetVisibilityError::Unchanged)
				} else {
					Ok((
						Patch::SetVisibility(SetVisibilityPatch {
							target: self.id,
							visibility: visible,
						}),
						Patch::SetVisibility(SetVisibilityPatch {
							target: self.id,
							visibility: self.is_visible,
						}),
					))
				}
			}
		}
		impl ILockable for $name {
			fn set_lock(&self, lock: bool) -> Result<(Patch, Patch), SetLockError> {
				if self.is_locked == lock {
					Err(SetLockError::Unchanged)
				} else {
					Ok((
						Patch::SetLock(SetLockPatch {
							target: self.id,
							lock: lock,
						}),
						Patch::SetLock(SetLockPatch {
							target: self.id,
							lock: self.is_locked,
						}),
					))
				}
			}
		}

		impl IPatchable for $name {
			fn patch(&self, patch: &Patch) -> Option<Box<Self>> {
				if patch.target() == self.id {
					return match patch {
						Patch::Rename(patch) => Some(Box::new($name {
							id: self.id,
							is_visible: self.is_visible,
							is_locked: self.is_locked,
							name: Arc::new(patch.name.clone()),
							size: self.size.clone(),
							albedo: self.albedo.clone(),
							alpha: self.alpha.clone(),
							normal: self.normal.clone(),
						})),
						Patch::SetVisibility(patch) => Some(Box::new($name {
							id: self.id,
							is_visible: patch.visibility,
							is_locked: self.is_locked,
							name: self.name.clone(),
							size: self.size.clone(),
							albedo: self.albedo.clone(),
							alpha: self.alpha.clone(),
							normal: self.normal.clone(),
						})),
						Patch::SetLock(patch) => Some(Box::new($name {
							id: self.id,
							is_visible: self.is_visible,
							is_locked: patch.lock,
							name: self.name.clone(),
							size: self.size.clone(),
							albedo: self.albedo.clone(),
							alpha: self.alpha.clone(),
							normal: self.normal.clone(),
						})),
						// Patch::CropLayer(patch) => {
						// 	assert_eq!(patch.size.w + patch.offset.x <= self.size.w, true);
						// 	assert_eq!(patch.size.h + patch.offset.y <= self.size.h, true);
						// 	let mut data =
						// 		vec![Default::default(); (patch.size.w * patch.size.h) as usize];
						// 	for i in 0..data.len() {
						// 		let x = patch.offset.x + ((i as u32) % patch.size.w);
						// 		let y = patch.offset.y + ((i as u32) / patch.size.w);
						// 		data[i] = self[(x, y)];
						// 	}
						// 	Some(Box::new($name {
						// 		id: self.id,
						// 		is_visible: self.is_visible,
						// 		is_locked: self.is_locked,
						// 		name: self.name.clone(),
						// 		size: Arc::new(patch.size),
						// 		data: Arc::new(data),
						// 	}))
						// },
						Patch::$patchrestorepatch(patch) => Some(Box::new($name {
							id: self.id,
							is_visible: self.is_visible,
							is_locked: self.is_locked,
							name: Arc::new(patch.name.to_owned()),
							size: Arc::new(patch.size),
							albedo: Arc::new(patch.albedo.to_owned()),
							alpha: Arc::new(patch.alpha.to_owned()),
							normal: Arc::new(patch.normal.to_owned()),
						})),
						// Patch::ResizeLayer(patch) => {
						// 	let mut data =
						// 		vec![Default::default(); (patch.size.w * patch.size.h) as usize];
						// 	patch.interpolation.interpolate(
						// 		&self.size,
						// 		&self.data,
						// 		&patch.size,
						// 		&mut data,
						// 		Mat2::scaling_2d(Vec2::new(
						// 			((self.size.w - 1) as f32) / (patch.size.w as f32),
						// 			((self.size.h - 1) as f32) / (patch.size.h as f32),
						// 		)),
						// 	);
						// 	Some(Box::new($name {
						// 		id: self.id,
						// 		is_visible: self.is_visible,
						// 		is_locked: self.is_locked,
						// 		name: self.name.clone(),
						// 		size: Arc::new(patch.size),
						// 		data: Arc::new(data),
						// 	}))
						// },
						// Patch::$patchstencilpatch(patch) => {
						// 	let mut data: Vec<$color> = Vec::from_iter(self.data.iter().cloned());
						// 	for (x, y, d) in patch.stencil.iter() {
						// 		let x = x + patch.offset.x;
						// 		let y = y + patch.offset.y;
						// 		let i = (x * self.size.h + y) as usize;
						// 		data[i] = Blend::blend(&self.data[i], &d, &patch.blend_mode);
						// 	}
						// 	Some(Box::new($name {
						// 		id: self.id,
						// 		is_visible: self.is_visible,
						// 		is_locked: self.is_locked,
						// 		name: self.name.clone(),
						// 		size: self.size.clone(),
						// 		data: Arc::new(data),
						// 	}))
						// }
						_ => None,
					};
				}
				return None;
			}
		}
	};
}

define_canvas!(CanvasPalette, Palette, StencilPalette, ApplyStencilPatch, ApplyStencilPalette, RestoreLayerCanvasPatch, RestoreLayerCanvasPalette);
define_canvas!(CanvasRGB, RGB, StencilRGB, ApplyStencilPatch, ApplyStencilRGB, RestoreLayerCanvasPatch, RestoreLayerCanvasRGB);
define_canvas!(CanvasUV, UV, StencilUV, ApplyStencilPatch, ApplyStencilUV, RestoreLayerCanvasPatch, RestoreLayerCanvasUV);

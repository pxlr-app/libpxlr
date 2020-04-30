use crate::color::*;
use crate::parser;
use crate::parser::Parser;
use crate::patch::*;
use crate::sprite::*;
use async_std::io;
use async_std::io::prelude::*;
use math::blend::*;
use math::interpolation::*;
use math::{Extent2, Mat2, Vec2};
use nom::multi::many_m_n;
use nom::IResult;
use serde::{Deserialize, Serialize};
use std::iter::FromIterator;
use std::ops::Index;
use std::rc::Rc;
use uuid::Uuid;

pub trait Canvas {
	type Color: Color;
	type Stencil: Stencil;
}

macro_rules! define_canvas {
	($name:ident $color:ident $stencil:ident $stencilpatch:ident $patchstencilpatch:ident $restorepatch:ident $patchrestorepatch:ident) => {
		#[derive(Debug, Serialize, Deserialize)]
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

		impl Canvas for $name {
			type Color = $color;
			type Stencil = $stencil;
		}

		impl Index<(u32, u32)> for $name {
			type Output = $color;

			fn index(&self, (x, y): (u32, u32)) -> &$color {
				let i = (y * self.size.w + x) as usize;
				&self.data[i]
			}
		}

		impl Layer for $name {
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
							data: (*self.data).to_owned(),
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
							data: (*self.data).to_owned(),
						}),
					))
				}
			}
		}

		impl<'a> Renamable<'a> for $name {
			fn rename(&self, new_name: &'a str) -> Result<(Patch, Patch), RenameError> {
				if *self.name == new_name {
					Err(RenameError::SameName)
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

		impl<S> parser::v0::PartitionTableParse<S> for $name
		where
			S: io::Read + io::Write + io::Seek + std::marker::Unpin,
		{
			type Output = $name;

			// TODO Due to https://github.com/dtolnay/async-trait/issues/46
			//      had to expand the macro manually. Keeping original
			//
			// async fn parse<'b>(
			// 	_index: &parser::v0::PartitionIndex,
			// 	row: &parser::v0::PartitionTableRow,
			// 	_storage: &mut dyn FileStorage,
			// 	bytes: &'b [u8],
			// ) -> IResult<&'b [u8], Self::Output> {
			// 	let (bytes, size) = Extent2::<u32>::parse(bytes)?;
			// 	let (bytes, data) = many_m_n(
			// 		(size.w as usize) * (size.h as usize),
			// 		(size.w as usize) * (size.h as usize),
			// 		<$color as crate::parser::Parser>::parse,
			// 	)(bytes)?;
			// 	Ok((
			// 		bytes,
			// 		$name {
			// 			id: row.id,
			// 			name: Rc::new(String::from(&row.name)),
			// 			size: Rc::new(size),
			// 			data: Rc::new(data),
			// 		},
			// 	))
			// }
			fn parse<'life0, 'life1, 'life2, 'life3, 'async_trait>(
				index: &'life1 parser::v0::PartitionIndex,
				row: &'life2 parser::v0::PartitionTableRow,
				_storage: &'life3 mut S,
				bytes: &'life0 [u8],
			) -> ::core::pin::Pin<
				Box<
					dyn ::core::future::Future<Output = IResult<&'life0 [u8], Self::Output>>
						+ 'async_trait,
				>,
			>
			where
				'life0: 'async_trait,
				'life1: 'async_trait,
				'life2: 'async_trait,
				'life3: 'async_trait,
				Self: 'async_trait,
			{
				async fn run<'b, S>(
					_index: &parser::v0::PartitionIndex,
					row: &parser::v0::PartitionTableRow,
					_storage: &mut S,
					bytes: &'b [u8],
				) -> IResult<&'b [u8], $name>
				where
					S: io::Read + io::Write + io::Seek + std::marker::Unpin,
				{
					let (bytes, size) = Extent2::<u32>::parse(bytes)?;
					let (bytes, data) = many_m_n(
						(size.w as usize) * (size.h as usize),
						(size.w as usize) * (size.h as usize),
						<$color as crate::parser::Parser>::parse,
					)(bytes)?;
					Ok((
						bytes,
						$name {
							id: row.id,
							name: Rc::new(String::from(&row.name)),
							size: Rc::new(size),
							data: Rc::new(data),
						},
					))
				}
				Box::pin(run(index, row, _storage, bytes))
			}

			// TODO Due to https://github.com/dtolnay/async-trait/issues/46
			//      had to expand the macro manually. Keeping original
			//
			// async fn write(
			// 	&self,
			// 	index: &mut parser::v0::PartitionIndex,
			// 	storage: &mut S,
			// ) -> io::Result<usize> {
			// 	let offset = storage.seek(io::SeekFrom::Current(0)).await?;
			// 	let size = {
			// 		let mut b: usize = 8;
			// 		self.size.write(storage).await?;
			// 		for color in self.data.iter() {
			// 			b += color.write(storage).await?;
			// 		}
			// 		b
			// 	};
			// 	if let Some(i) = index.index_uuid.get(&self.id) {
			// 		let mut row = index.rows.get_mut(*i).unwrap();
			// 		row.chunk_offset = offset as u64;
			// 		row.chunk_size = size as u32;
			// 	} else {
			// 		let row = parser::v0::PartitionTableRow {
			// 			id: self.id,
			// 			chunk_type: parser::v0::ChunkType::Note,
			// 			chunk_offset: offset as u64,
			// 			chunk_size: size as u32,
			// 			position: Vec2::new(0., 0.),
			// 			size: *self.size,
			// 			name: String::from(&*self.name),
			// 			children: Vec::new(),
			// 			preview: Vec::new(),
			// 		};
			// 		index.index_uuid.insert(row.id, index.rows.len());
			// 		index.rows.push(row);
			// 	}
			// 	Ok(size)
			// }
			fn write<'life0, 'life1, 'life2, 'async_trait>(
				&'life0 self,
				index: &'life1 mut parser::v0::PartitionIndex,
				storage: &'life2 mut S,
			) -> ::core::pin::Pin<
				Box<dyn ::core::future::Future<Output = io::Result<usize>> + 'async_trait>,
			>
			where
				'life0: 'async_trait,
				'life1: 'async_trait,
				'life2: 'async_trait,
				Self: 'async_trait,
			{
				async fn run<S>(
					_self: &$name,
					index: &mut parser::v0::PartitionIndex,
					storage: &mut S,
				) -> io::Result<usize>
				where
					S: io::Read + io::Write + io::Seek + std::marker::Unpin,
				{
					let offset = storage.seek(io::SeekFrom::Current(0)).await?;
					let size = {
						let mut b: usize = 8;
						_self.size.write(storage).await?;
						for color in _self.data.iter() {
							b += color.write(storage).await?;
						}
						b
					};
					if let Some(i) = index.index_uuid.get(&_self.id) {
						let mut row = index.rows.get_mut(*i).unwrap();
						row.chunk_offset = offset as u64;
						row.chunk_size = size as u32;
					} else {
						let row = parser::v0::PartitionTableRow {
							id: _self.id,
							chunk_type: parser::v0::ChunkType::Note,
							chunk_offset: offset as u64,
							chunk_size: size as u32,
							position: Vec2::new(0., 0.),
							size: *_self.size,
							name: String::from(&*_self.name),
							children: Vec::new(),
							preview: Vec::new(),
						};
						index.index_uuid.insert(row.id, index.rows.len());
						index.rows.push(row);
					}
					Ok(size)
				}
				Box::pin(run(self, index, storage))
			}
		}
	};
}

define_canvas!(CanvasI I StencilI ApplyStencilIPatch ApplyStencilI RestoreLayerCanvasIPatch RestoreLayerCanvasI);
define_canvas!(CanvasIXYZ IXYZ StencilIXYZ ApplyStencilIXYZPatch ApplyStencilIXYZ RestoreLayerCanvasIXYZPatch RestoreLayerCanvasIXYZ);
define_canvas!(CanvasUV UV StencilUV ApplyStencilUVPatch ApplyStencilUV RestoreLayerCanvasUVPatch RestoreLayerCanvasUV);
define_canvas!(CanvasRGB RGB StencilRGB ApplyStencilRGBPatch ApplyStencilRGB RestoreLayerCanvasRGBPatch RestoreLayerCanvasRGB);
define_canvas!(CanvasRGBA RGBA StencilRGBA ApplyStencilRGBAPatch ApplyStencilRGBA RestoreLayerCanvasRGBAPatch RestoreLayerCanvasRGBA);
define_canvas!(CanvasRGBAXYZ RGBAXYZ StencilRGBAXYZ ApplyStencilRGBAXYZPatch ApplyStencilRGBAXYZ RestoreLayerCanvasRGBAXYZPatch RestoreLayerCanvasRGBAXYZ);

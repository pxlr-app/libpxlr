use crate::parser;
use crate::parser::IParser;
use document::{
	color::{I, IXYZ, RGB, RGBA, RGBAXYZ, UV},
	sprite::{CanvasI, CanvasIXYZ, CanvasRGB, CanvasRGBA, CanvasRGBAXYZ, CanvasUV},
	INode, Node,
};
use futures::io;
use math::Extent2;
use math::Vec2;
use nom::multi::many_m_n;
use nom::IResult;
use std::sync::Arc;

macro_rules! define_canvas {
	($name:ident $color:ident $stencil:ident $stencilpatch:ident $patchstencilpatch:ident $restorepatch:ident $patchrestorepatch:ident) => {
		impl parser::v0::IParser for $name {
			type Output = $name;

			// TODO Due to https://github.com/dtolnay/async-trait/issues/46
			//		had to expand the macro manually
			//
			// fn parse<'b, S>(
			// 	_index: &parser::v0::PartitionIndex,
			// 	row: &parser::v0::PartitionTableRow,
			// 	_storage: &mut S,
			// 	bytes: &'b [u8],
			// ) -> IResult<&'b [u8], Self::Output>
			// where
			// 	S: parser::ReadAt,
			// {
			// 	let (bytes, size) = Extent2::<u32>::parse(bytes)?;
			// 	let (bytes, data) = many_m_n(
			// 		(size.w as usize) * (size.h as usize),
			// 		(size.w as usize) * (size.h as usize),
			// 		<$color as crate::parser::IParser>::parse,
			// 	)(bytes)?;
			// 	Ok((
			// 		bytes,
			// 		$name {
			// 			id: row.id,
			// 			is_visible: row.is_visible,
			// 			is_locked: row.is_locked,
			// 			name: Arc::new(String::from(&row.name)),
			// 			size: Arc::new(size),
			// 			data: Arc::new(data),
			// 		},
			// 	))
			// }
			fn parse<'a, 'b, 'c, 'async_trait>(
				row: &'b parser::v0::PartitionTableRow,
				_children: &'c mut Vec<Node>,
				bytes: &'a [u8],
			) -> ::core::pin::Pin<
				Box<
					dyn ::core::future::Future<Output = IResult<&'a [u8], Self::Output>>
						+ std::marker::Send
						+ 'async_trait,
				>,
			>
			where
				'a: 'async_trait,
				'b: 'async_trait,
				'c: 'async_trait,
				Self: std::marker::Sync + 'async_trait,
			{
				async fn run<'b>(
					row: &parser::v0::PartitionTableRow,
					bytes: &'b [u8],
				) -> IResult<&'b [u8], $name> {
					let (bytes, size) = Extent2::<u32>::parse(bytes)?;
					let (bytes, data) = many_m_n(
						(size.w as usize) * (size.h as usize),
						(size.w as usize) * (size.h as usize),
						<$color as crate::parser::IParser>::parse,
					)(bytes)?;
					Ok((
						bytes,
						$name {
							id: row.id,
							is_visible: row.is_visible,
							is_locked: row.is_locked,
							name: Arc::new(String::from(&row.name)),
							size: Arc::new(size),
							data: Arc::new(data),
						},
					))
				}
				Box::pin(run(row, bytes))
			}

			// TODO Due to https://github.com/dtolnay/async-trait/issues/46
			//		had to expand the macro manually
			//
			// fn write<S>(
			// 	&self,
			// 	index: &mut parser::v0::PartitionIndex,
			// 	storage: &mut S,
			// 	offset: u64,
			// ) -> io::Result<usize>
			// where
			// 	S: io::AsyncWriteExt,
			// {
			// 	let size = {
			// 		let mut b: usize = 8;
			// 		self.size.write(storage)?;
			// 		for color in self.data.iter() {
			// 			b += color.write(storage)?;
			// 		}
			// 		b
			// 	};
			// 	if let Some(i) = index.index_uuid.get(&self.id) {
			// 		let mut row = index.rows.get_mut(*i).unwrap();
			// 		row.chunk_offset = offset as u64;
			// 		row.chunk_size = size as u32;
			// 		row.is_visible = self.is_visible;
			// 		row.is_locked = self.is_locked;
			// 	} else {
			// 		let row = parser::v0::PartitionTableRow {
			// 			id: self.id,
			// 			chunk_type: parser::v0::ChunkType::Note,
			// 			chunk_offset: offset as u64,
			// 			chunk_size: size as u32,
			// 			is_root: false,
			// 			is_visible: self.is_visible,
			// 			is_locked: self.is_locked,
			// 			is_folded: false,
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
			fn write<'a, 'b, 'c, 'async_trait, S>(
				&'a self,
				index: &'b mut parser::v0::PartitionIndex,
				storage: &'c mut S,
				offset: u64,
			) -> ::core::pin::Pin<
				Box<
					dyn ::core::future::Future<Output = io::Result<usize>>
						+ std::marker::Send
						+ 'async_trait,
				>,
			>
			where
				'a: 'async_trait,
				'b: 'async_trait,
				'c: 'async_trait,
				Self: std::marker::Sync + 'async_trait,
				S: io::AsyncWriteExt + std::marker::Send + std::marker::Unpin,
			{
				async fn run<S>(
					canvas: &$name,
					index: &mut parser::v0::PartitionIndex,
					storage: &mut S,
					offset: u64,
				) -> io::Result<usize>
				where
					S: io::AsyncWriteExt + std::marker::Send + std::marker::Unpin,
				{
					let size = {
						let mut b: usize = 8;
						canvas.size.write(storage).await?;
						for color in canvas.data.iter() {
							b += color.write(storage).await?;
						}
						b
					};
					if let Some(i) = index.index_uuid.get(&canvas.id) {
						let mut row = index.rows.get_mut(*i).unwrap();
						row.chunk_offset = offset as u64;
						row.chunk_size = size as u32;
						row.is_visible = canvas.is_visible;
						row.is_locked = canvas.is_locked;
					} else {
						let row = parser::v0::PartitionTableRow {
							id: canvas.id,
							chunk_type: parser::v0::ChunkType::Note,
							chunk_offset: offset as u64,
							chunk_size: size as u32,
							is_root: false,
							is_visible: canvas.is_visible,
							is_locked: canvas.is_locked,
							is_folded: false,
							position: Vec2::new(0., 0.),
							size: *canvas.size,
							name: String::from(&*canvas.name),
							children: Vec::new(),
							preview: Vec::new(),
						};
						index.index_uuid.insert(row.id, index.rows.len());
						index.rows.push(row);
					}
					Ok(size)
				}
				Box::pin(run(self, index, storage, offset))
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

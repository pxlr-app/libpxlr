use crate::color;
use crate::file;
use crate::file::part;
use crate::file::writer::WriteTo;
use math::{Extent2, Vec2};
use nom::multi::many_m_n;
use std::io::SeekFrom;

#[derive(Debug, PartialEq)]
pub struct LayerGroup {
	pub position: Vec2<f32>,
}

impl crate::file::reader::v0::Reader for LayerGroup {
	fn from_bytes(bytes: &[u8]) -> nom::IResult<&[u8], LayerGroup> {
		let (bytes, position) = Vec2::<f32>::from_bytes(bytes)?;
		Ok((bytes, LayerGroup { position }))
	}
}

impl crate::file::writer::Writer for crate::sprite::LayerGroup {
	fn write<W: std::io::Write + std::io::Seek>(
		&self,
		file: &mut file::File,
		writer: &mut W,
	) -> std::io::Result<usize> {
		let offset = writer.seek(SeekFrom::Current(0))?;
		let mut size: usize = 0;
		for child in self.children.iter() {
			size += child.write(file, writer)?;
		}
		if let Some(i) = file.chunks.get(&self.id) {
			let mut row = file.rows.get_mut(*i).unwrap();
			row.chunk_offset = offset;
			row.chunk_size = 0;
		} else {
			let row = part::PartitionTableRow {
				id: self.id,
				chunk_type: part::ChunkType::Group,
				chunk_offset: offset,
				chunk_size: 0,
				position: *self.position,
				size: Extent2::new(0, 0),
				name: String::from(&*self.name),
				children: self
					.children
					.iter()
					.map(|c| *file.chunks.get(&c.id()).unwrap() as u32)
					.collect::<Vec<_>>(),
				preview: Vec::new(),
			};
			file.chunks.insert(row.id, file.rows.len());
			file.rows.push(row);
		}
		Ok(size)
	}
}

macro_rules! define_canvas {
	($name:ident, $color:path, $node:path, $type:path) => {
		#[derive(Debug, PartialEq)]
		pub struct $name {
			pub size: Extent2<u32>,
			pub data: Vec<$color>,
		}

		impl crate::file::reader::v0::Reader for $name {
			fn from_bytes(bytes: &[u8]) -> nom::IResult<&[u8], $name> {
				let (bytes, size) = Extent2::<u32>::from_bytes(bytes)?;
				let (bytes, data) = many_m_n(
					(size.w as usize) * (size.h as usize),
					(size.w as usize) * (size.h as usize),
					<$color as crate::file::reader::v0::Reader>::from_bytes,
				)(bytes)?;
				Ok((bytes, $name { size, data }))
			}
		}
		impl crate::file::writer::Writer for $node {
			fn write<W: std::io::Write + std::io::Seek>(
				&self,
				file: &mut file::File,
				writer: &mut W,
			) -> std::io::Result<usize> {
				let offset = writer.seek(SeekFrom::Current(0))?;
				let size = {
					let mut b: usize = 8;
					self.size.write_to(writer)?;
					for color in self.data.iter() {
						b += color.write_to(writer)?;
					}
					b
				};
				if let Some(i) = file.chunks.get(&self.id) {
					let mut row = file.rows.get_mut(*i).unwrap();
					row.chunk_offset = offset;
					row.chunk_size = size as u32;
				} else {
					let row = part::PartitionTableRow {
						id: self.id,
						chunk_type: $type,
						chunk_offset: offset,
						chunk_size: size as u32,
						position: Vec2::new(0., 0.),
						size: Extent2::new(0, 0),
						name: String::from(&*self.name),
						children: Vec::new(),
						preview: Vec::new(),
					};
					file.chunks.insert(row.id, file.rows.len());
					file.rows.push(row);
				}
				Ok(size)
			}
		}
	};
}

define_canvas!(
	CanvasI,
	color::I,
	crate::sprite::CanvasI,
	part::ChunkType::CanvasI
);
define_canvas!(
	CanvasIXYZ,
	color::IXYZ,
	crate::sprite::CanvasIXYZ,
	part::ChunkType::CanvasIXYZ
);
define_canvas!(
	CanvasUV,
	color::UV,
	crate::sprite::CanvasUV,
	part::ChunkType::CanvasUV
);
define_canvas!(
	CanvasRGB,
	color::RGB,
	crate::sprite::CanvasRGB,
	part::ChunkType::CanvasRGB
);
define_canvas!(
	CanvasRGBA,
	color::RGBA,
	crate::sprite::CanvasRGBA,
	part::ChunkType::CanvasRGBA
);
define_canvas!(
	CanvasRGBAXYZ,
	color::RGBAXYZ,
	crate::sprite::CanvasRGBAXYZ,
	part::ChunkType::CanvasRGBAXYZ
);

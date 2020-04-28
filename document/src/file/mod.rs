pub mod document;
mod file;
pub mod part;
pub mod reader;
pub mod sprite;
pub mod writer;
pub use self::file::*;

// #[cfg(test)]
// mod tests {
// 	use super::*;
// 	use crate::file::reader;
// 	use std::io::{BufWriter, Cursor};
// 	// use crate::color;
// 	// use math::{Extent2, Vec2};

// 	#[test]
// 	fn it_parse_header() {
// 		let header = Header { version: 1 };
// 		let mut writer = BufWriter::new(Cursor::new(Vec::new()));
// 		let len = header.write_to(&mut writer).unwrap();
// 		assert_eq!(len, 5);

// 		let buffer = writer.buffer();
// 		let (buffer, header2) = <Header as reader::v0::Reader>::from_bytes(buffer).unwrap();
// 		assert_eq!(header.version, header2.version);
// 		assert_eq!(buffer, []);
// 	}

// 	#[test]
// 	fn it_parse_partition_table_row() {
// 		let row = PartitionTableRow {
// 			id: Uuid::new_v4(),
// 			chunk_type: ChunkType::Note,
// 			chunk_offset: 0,
// 			chunk_size: 8,
// 			position: Vec2::new(10., 10.),
// 			size: Extent2::new(0, 0),
// 			name: "Foo".into(),
// 			children: vec![],
// 			preview: vec![],
// 		};
// 		let mut writer = BufWriter::new(Cursor::new(Vec::new()));
// 		let len = row.write_to(&mut writer).unwrap();
// 		assert_eq!(len, 61);

// 		let buffer = writer.buffer();
// 		let (buffer, row2) = <PartitionTableRow as reader::v0::Reader>::from_bytes(buffer).unwrap();
// 		assert_eq!(row.id, row2.id);
// 		assert_eq!(row.chunk_type, row2.chunk_type);
// 		assert_eq!(row.chunk_offset, row2.chunk_offset);
// 		assert_eq!(row.chunk_size, row2.chunk_size);
// 		assert_eq!(row.position, row2.position);
// 		assert_eq!(row.size, row2.size);
// 		assert_eq!(row.name, row2.name);
// 		assert_eq!(row.children, row2.children);
// 		assert_eq!(row.preview, row2.preview);
// 		assert_eq!(buffer, []);
// 	}

// 	#[test]
// 	fn it_parse_partition_table_rows() {
// 		let rows = vec![
// 			PartitionTableRow {
// 				id: Uuid::new_v4(),
// 				chunk_type: ChunkType::Note,
// 				chunk_offset: 0,
// 				chunk_size: 8,
// 				position: Vec2::new(10., 10.),
// 				size: Extent2::new(0, 0),
// 				name: "Foo".into(),
// 				children: vec![],
// 				preview: vec![],
// 			},
// 			PartitionTableRow {
// 				id: Uuid::new_v4(),
// 				chunk_type: ChunkType::Note,
// 				chunk_offset: 0,
// 				chunk_size: 8,
// 				position: Vec2::new(10., 20.),
// 				size: Extent2::new(0, 0),
// 				name: "Bar".into(),
// 				children: vec![],
// 				preview: vec![],
// 			},
// 		];
// 		let mut writer = BufWriter::new(Cursor::new(Vec::new()));
// 		let len = rows.write_to(&mut writer).unwrap();
// 		assert_eq!(len, 122);

// 		let buffer = writer.buffer();
// 		let (buffer, rows2) =
// 			<Vec<PartitionTableRow> as reader::v0::Reader>::from_bytes(buffer).unwrap();
// 		assert_eq!(rows, rows2);
// 		assert_eq!(buffer, []);
// 	}
// }

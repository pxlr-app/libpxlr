use crate::parser;
use async_trait::async_trait;
use document::{INode, Node, Note};
use futures::io;
use math::Extent2;
use nom::IResult;
use std::sync::Arc;

#[async_trait]
impl parser::v0::IParser for Note {
	type Output = Note;

	async fn parse<'b>(
		row: &parser::v0::PartitionTableRow,
		_children: &mut Vec<Node>,
		bytes: &'b [u8],
	) -> IResult<&'b [u8], Self::Output> {
		Ok((
			bytes,
			Note {
				id: row.id,
				is_visible: row.is_visible,
				is_locked: row.is_locked,
				note: Arc::new(String::from(&row.name)),
				position: Arc::new(row.position),
			},
		))
	}
}

#[async_trait]
impl parser::v0::IWriter for Note {
	async fn write<S>(
		&self,
		index: &mut parser::v0::PartitionIndex,
		_storage: &mut S,
		offset: u64,
	) -> io::Result<usize>
	where
		S: io::AsyncWriteExt + std::marker::Send + std::marker::Unpin,
	{
		if let Some(i) = index.index_uuid.get(&self.id) {
			let mut row = index.rows.get_mut(*i).unwrap();
			row.chunk_offset = offset;
			row.chunk_size = 0;
			row.is_visible = self.is_visible();
			row.is_locked = self.is_locked();
			row.position = *self.position;
			row.name = String::from(&*self.note);
		} else {
			let row = parser::v0::PartitionTableRow {
				id: self.id,
				chunk_type: parser::v0::ChunkType::Note,
				chunk_offset: offset,
				chunk_size: 0,
				is_root: false,
				is_visible: self.is_visible(),
				is_locked: self.is_locked(),
				is_folded: false,
				position: *self.position,
				size: Extent2::new(0, 0),
				name: String::from(&*self.note),
				children: Vec::new(),
				preview: Vec::new(),
			};
			index.index_uuid.insert(row.id, index.rows.len());
			index.rows.push(row);
		}
		Ok(0)
	}
}

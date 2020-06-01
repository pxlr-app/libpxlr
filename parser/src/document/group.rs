use crate::parser;
use async_trait::async_trait;
use document::{DocumentNode, Group, Node};
use futures::io;
use math::Extent2;
use nom::IResult;
use std::sync::Arc;

#[async_trait]
impl parser::v0::IParser for Group {
	type Output = Group;

	async fn parse<'b>(
		row: &parser::v0::PartitionTableRow,
		children: &mut Vec<Node>,
		bytes: &'b [u8],
	) -> IResult<&'b [u8], Self::Output> {
		use std::convert::TryInto;
		let children: Vec<Arc<DocumentNode>> = children
			.drain(..)
			.map(|node| Arc::new(node.try_into().expect("Node is not a valid DocumentNode.")))
			.collect();
		Ok((
			bytes,
			Group {
				id: row.id,
				is_visible: row.is_visible,
				is_locked: row.is_locked,
				is_folded: row.is_folded,
				name: Arc::new(String::from(&row.name)),
				position: Arc::new(row.position),
				children: Arc::new(children),
			},
		))
	}

	async fn write<S>(
		&self,
		index: &mut parser::v0::PartitionIndex,
		storage: &mut S,
		offset: u64,
	) -> io::Result<usize>
	where
		S: io::AsyncWriteExt + std::marker::Send + std::marker::Unpin,
	{
		let mut size: usize = 0;
		for child in self.children.iter() {
			size += child.write(index, storage, offset + (size as u64)).await?;
		}
		let children = self
			.children
			.iter()
			.map(|c| *index.index_uuid.get(&c.id()).unwrap() as u32)
			.collect::<Vec<_>>();
		if let Some(i) = index.index_uuid.get(&self.id) {
			let mut row = index.rows.get_mut(*i).unwrap();
			row.chunk_offset = offset;
			row.chunk_size = 0;
			row.is_visible = self.is_visible;
			row.is_locked = self.is_locked;
			row.is_folded = self.is_folded;
			row.position = *self.position;
			row.name = String::from(&*self.name);
			row.children = children;
		} else {
			let row = parser::v0::PartitionTableRow {
				id: self.id,
				chunk_type: parser::v0::ChunkType::Group,
				chunk_offset: offset,
				chunk_size: 0,
				is_root: false,
				is_visible: self.is_visible,
				is_locked: self.is_locked,
				is_folded: self.is_folded,
				position: *self.position,
				size: Extent2::new(0, 0),
				name: String::from(&*self.name),
				children: children,
				preview: Vec::new(),
			};
			index.index_uuid.insert(row.id, index.rows.len());
			index.rows.push(row);
		}
		Ok(size)
	}
}

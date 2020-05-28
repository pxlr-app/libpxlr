use crate::parser;
use crate::parser::IParser;
use async_trait::async_trait;
use document::{
	color::ColorMode,
	sprite::{LayerGroup, LayerNode},
	INode, Node,
};
use futures::io;
use math::Extent2;
use nom::IResult;
use std::sync::Arc;

#[async_trait]
impl parser::v0::IParser for LayerGroup {
	type Output = LayerGroup;

	async fn parse<'b>(
		row: &parser::v0::PartitionTableRow,
		children: &mut Vec<Node>,
		bytes: &'b [u8],
	) -> IResult<&'b [u8], Self::Output> {
		use std::convert::TryInto;
		let (bytes, color_mode) = <ColorMode as parser::IParser>::parse(bytes)?;
		let children: Vec<Arc<LayerNode>> = children
			.drain(..)
			.map(|node| Arc::new(node.try_into().expect("Node is not a valid LayerNode.")))
			.collect();
		Ok((
			bytes,
			LayerGroup {
				id: row.id,
				is_visible: row.is_visible,
				is_locked: row.is_locked,
				is_folded: row.is_folded,
				name: Arc::new(String::from(&row.name)),
				color_mode: color_mode,
				children: Arc::new(children),
				position: Arc::new(row.position),
				size: Arc::new(row.size),
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
		self.color_mode.write(storage).await?;
		let mut size: usize = 2;
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
			row.chunk_offset = offset as u64;
			row.chunk_size = 2;
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
				chunk_offset: offset as u64,
				chunk_size: 2,
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

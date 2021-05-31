use document_core::NodeType;
use document_file::{File, FileError};
use std::sync::Arc;

pub trait Serialize {
	type Error;

	fn serialize(&self) -> Result<Vec<u8>, Self::Error>;
}

pub trait Deserialize {
	type Output;
	type Error;

	fn deserialize(data: &[u8]) -> Result<Self::Output, Self::Error>;
}

impl Serialize for Arc<NodeType> {
	type Error = FileError;

	fn serialize(&self) -> Result<Vec<u8>, FileError> {
		let mut file = File::default();
		let mut buffer = async_std::io::Cursor::new(Vec::new());
		file.set_root_node(self.clone());
		async_std::task::block_on(file.append(&mut buffer, "", ""))?;
		Ok(buffer.into_inner())
	}
}

impl Deserialize for Arc<NodeType> {
	type Output = Arc<NodeType>;
	type Error = FileError;

	fn deserialize(data: &[u8]) -> Result<Arc<NodeType>, FileError> {
		let mut buffer = async_std::io::Cursor::new(data);
		let file = async_std::task::block_on(File::read(&mut buffer))?;
		let root = async_std::task::block_on(file.get_root_node(&mut buffer, false))?;
		Ok(root)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use document_core::*;

	#[test]
	fn serialize_note() {
		let note = Arc::new(NodeType::Note(Note::default()));
		let data = note.clone().serialize().unwrap();

		let note2 = Arc::<NodeType>::deserialize(&data).unwrap();
		assert_eq!(*note2, *note);
	}
}

use document_core::NodeType;
use document_file::{File, FileError};
use std::sync::Arc;

/// Serialize NodeType
pub fn serialize(node: Arc<NodeType>) -> Result<Vec<u8>, FileError> {
	let mut file = File::default();
	let mut buffer = std::io::Cursor::new(Vec::new());
	file.set_root_node(node);
	file.append(&mut buffer)?;
	Ok(buffer.into_inner())
}

/// Deserialize NodeType
pub fn deserialize(data: &[u8]) -> Result<Arc<NodeType>, FileError> {
	let mut buffer = std::io::Cursor::new(data);
	let file = File::read(&mut buffer)?;
	let root = file.get_root_node(&mut buffer)?;
	Ok(root)
}

#[cfg(test)]
mod tests {
	use super::*;
	use document_core::*;

	#[test]
	fn serialize_note() {
		let note = Arc::new(NodeType::Note(Note::default()));
		let data = serialize(note.clone()).unwrap();
		
		let note2 = deserialize(&data).unwrap();
		assert_eq!(*note2, *note);
	}
}
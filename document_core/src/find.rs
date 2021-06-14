use crate::{walk, Node, NodeType, VisitorOps};
use std::sync::Arc;
use uuid::Uuid;

pub fn find(entry: &Arc<NodeType>, id: &Uuid) -> Option<Arc<NodeType>> {
	let mut result: Option<Arc<NodeType>> = None;
	walk(
		entry,
		&mut |node| {
			if node.id() == id {
				result.replace(node.clone());
				VisitorOps::BREAK
			} else {
				VisitorOps::CONTINUE
			}
		},
		&mut |_| {},
	);

	result
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{Group, Node, Note};

	#[test]
	fn find_by_id() {
		let note = Note::new("NoteA", (0, 0), "");
		let note_id = *note.id();
		let group2 = Group::new("GroupB", (0, 0), vec![Arc::new(NodeType::Note(note))]);
		let group2_id = *group2.id();
		let group1 = Group::new("GroupA", (0, 0), vec![Arc::new(NodeType::Group(group2))]);
		let group1_id = *group1.id();
		let root = Arc::new(NodeType::Group(group1));

		find(&root, &note_id).expect("Could not find note_id");
		find(&root, &group2_id).expect("Could not find group2_id");
		find(&root, &group1_id).expect("Could not find group1_id");
	}
}

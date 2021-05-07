use crate::{HasChildren, NodeType};
use std::{convert::TryInto, sync::Arc};

#[derive(Debug, Clone)]
pub enum VisitorOps {
	BREAK,
	CONTINUE,
	SKIP,
}

pub fn walk<
	'node,
	EnterFn: FnMut(&'node Arc<NodeType>) -> VisitorOps,
	LeaveFn: FnMut(&'node Arc<NodeType>),
>(
	node: &'node Arc<NodeType>,
	enter: &mut EnterFn,
	leave: &mut LeaveFn,
) -> VisitorOps {
	let ops = enter(node);
	// Continue deep down
	if let VisitorOps::CONTINUE = ops {
		let group: Result<&dyn HasChildren, ()> = (&**node).try_into();
		match group {
			Ok(group) => {
				for child in group.children().iter() {
					if let VisitorOps::BREAK = walk(child, enter, leave) {
						// Break
						break;
					}
				}
			}
			_ => {}
		}
		leave(node);
	}
	ops
}

#[cfg(test)]
mod tests {
	use crate::{walk, Group, Node, NodeType, Note, VisitorOps};
	use std::sync::Arc;

	#[test]
	fn walk_tree() {
		let note = Note::new("NoteA", (0, 0), "");
		let group2 = Group::new("GroupB", (0, 0), vec![Arc::new(NodeType::Note(note))]);
		let group1 = Group::new("GroupA", (0, 0), vec![Arc::new(NodeType::Group(group2))]);
		let root = Arc::new(NodeType::Group(group1));

		let mut names_enter: Vec<&str> = vec![];
		let mut names_leave: Vec<&str> = vec![];
		walk(
			&root,
			&mut |node| {
				names_enter.push(node.name());
				VisitorOps::CONTINUE
			},
			&mut |node| {
				names_leave.push(node.name());
			},
		);

		assert_eq!(names_enter, vec!["GroupA", "GroupB", "NoteA"]);
		assert_eq!(names_leave, vec!["NoteA", "GroupB", "GroupA"]);
	}
}

pub use document_derive::*;
pub mod any;
pub mod color;
pub mod node;
pub mod patch;

pub mod prelude {
	pub use super::{any::*, color::*, node::*, patch};
	pub use document_derive::*;
	pub use math::{Extent2, Vec2};
	pub use serde::{Deserialize, Serialize};
	pub use std::rc::Rc;
	pub use uuid::Uuid;
}

#[doc(hidden)]
pub use typetag;

#[doc(hidden)]
pub use lazy_static;

#[cfg(test)]
mod tests {
	use super::prelude::*;
	use crate as document;

	#[derive(Debug, DocumentNode, Serialize, Deserialize)]
	struct Empty {
		pub id: Uuid,
		pub name: Rc<String>,
	}

	impl Name for Empty {
		fn name(&self) -> String {
			(*self.name).clone()
		}
	}
	impl Position for Empty {}
	impl Size for Empty {}
	impl Visible for Empty {}
	impl Locked for Empty {}
	impl Folded for Empty {}

	impl Patchable for Empty {
		fn patch(&mut self, patch: &dyn Patch) -> bool {
			if let Some(patch) = patch.downcast::<Rename>() {
				self.name = Rc::new(patch.name.clone());
				true
			} else {
				false
			}
		}
	}

	#[derive(Debug, Patch, Serialize, Deserialize)]
	struct Rename {
		pub target: Uuid,
		pub name: String,
	}

	#[test]
	fn test_cast() {
		let empty = Empty {
			id: Uuid::new_v4(),
			name: Rc::new("Foo".into()),
		};
		let node: &dyn Node = &empty;
		let documentnode = node.as_documentnode();
		assert_eq!(documentnode.is_some(), true);
		let note2 = documentnode.unwrap().downcast::<Empty>();
		assert_eq!(note2.is_some(), true);
	}

	#[test]
	fn test_patch() {
		let mut empty = Empty {
			id: Uuid::new_v4(),
			name: Rc::new("Foo".into()),
		};
		let patch = Rename {
			target: empty.id,
			name: "Bar".into(),
		};
		let node: &mut dyn Node = &mut empty;
		node.patch(&patch);
		assert_eq!(*empty.name, "Bar");
	}
}

use crate::{
	any::{Any, Downcast},
	patch,
};
use math::{Extent2, Vec2};
use std::fmt::Debug;
use uuid::Uuid;

#[typetag::serde(tag = "patch", content = "props")]
pub trait Patch: Any + Debug {
	fn target(&self) -> Uuid;
}
impl Downcast for dyn Patch {}

pub trait Patchable {
	fn patch(&mut self, patch: &dyn Patch) -> bool;
}

#[typetag::serde(tag = "node", content = "props")]
pub trait Node: Any + Debug + Patchable {
	fn id(&self) -> Uuid;
	fn as_documentnode(&self) -> Option<&dyn DocumentNode> {
		None
	}
}
impl Downcast for dyn Node {}

pub trait Name {
	fn name(&self) -> String {
		"".into()
	}
	fn rename(&self, _name: String) -> Option<(patch::Rename, patch::Rename)> {
		None
	}
}
pub trait Position {
	fn position(&self) -> Vec2<u32> {
		Vec2::new(0, 0)
	}
	fn translate(&self, _target: Vec2<u32>) -> Option<(patch::Translate, patch::Translate)> {
		None
	}
}
pub trait Size {
	fn size(&self) -> Extent2<u32> {
		Extent2::new(0, 0)
	}
	fn resize(&self, _target: Extent2<u32>) -> Option<(patch::Resize, patch::Resize)> {
		None
	}
}
pub trait Visible {
	fn visible(&self) -> bool {
		true
	}
	fn set_visibility(&self, _visible: bool) -> Option<(patch::SetVisible, patch::SetVisible)> {
		None
	}
}
pub trait Locked {
	fn locked(&self) -> bool {
		false
	}
	fn set_lock(&self, _locked: bool) -> Option<(patch::SetLock, patch::SetLock)> {
		None
	}
}
pub trait Folded {
	fn folded(&self) -> bool {
		false
	}
	fn set_fold(&self, _folded: bool) -> Option<(patch::SetFold, patch::SetFold)> {
		None
	}
}

pub trait DocumentNode: Node + Name + Position + Size + Visible + Locked + Folded {
	fn as_node(&self) -> &dyn Node;
}
impl Downcast for dyn DocumentNode {}

mod group;
mod note;
pub use group::*;
pub use note::*;

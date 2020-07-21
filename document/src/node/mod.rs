use crate::{
	any::{Any, Downcast},
	parser, patch,
};
use math::{Extent2, Vec2};
use serde::ser::{Serialize, SerializeMap, Serializer};
use std::{collections::BTreeMap, fmt::Debug, sync::Arc};
use uuid::Uuid;
mod group;
mod note;
pub use group::*;
pub use note::*;

pub trait Node: Any + Debug + patch::Patchable + erased_serde::Serialize {
	fn id(&self) -> Uuid;
	fn node_type(&self) -> &'static str;
	fn as_documentnode(&self) -> Option<&dyn DocumentNode> {
		None
	}
}
impl Downcast for dyn Node {}

pub type NodeRef = Arc<Box<dyn Node>>;
pub type NodeList = Vec<NodeRef>;

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

pub type DeserializeFn<T> = fn(&mut dyn erased_serde::Deserializer) -> erased_serde::Result<Box<T>>;

pub struct NodeRegistryEntry<'b, T: ?Sized> {
	pub v0_parse: parser::v0::ParseFn<'b>,
	pub deserialize: DeserializeFn<T>,
}

pub type NodeRegistry<'b, T: ?Sized> = BTreeMap<&'static str, NodeRegistryEntry<'b, T>>;

static mut NODES: Option<NodeRegistry<dyn Node>> = None;

impl dyn Node {
	pub fn from(node: Box<dyn Node>) -> NodeRef {
		Arc::new(node)
	}

	pub(crate) fn registry<'b>() -> &'static Option<NodeRegistry<'b, dyn Node>> {
		unsafe {
			std::mem::transmute::<
				&Option<NodeRegistry<'static, dyn Node>>,
				&Option<NodeRegistry<'b, dyn Node>>,
			>(&NODES)
		}
	}
	pub(crate) fn init_registry<'b>(map: NodeRegistry<'b, dyn Node>) {
		unsafe {
			NODES = Some(std::mem::transmute::<
				NodeRegistry<'b, dyn Node>,
				NodeRegistry<'static, dyn Node>,
			>(map));
		}
	}
}

impl Serialize for dyn Node {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		let node_name = self.node_type();
		let mut map = serializer.serialize_map(Some(2))?;
		map.serialize_entry("node", node_name)?;
		//map.serialize_entry("props", self)?;
		map.end()
	}
}

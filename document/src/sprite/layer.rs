use crate::node::INode;
use crate::patch::*;
use crate::sprite::Group;

pub enum Layer {
	Group(Group)
}

pub trait ILayer: INode {
	fn patch(&self, patch: &Patch) -> Option<Layer>;
}
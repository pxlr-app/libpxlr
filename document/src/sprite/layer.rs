use std::any::Any;
use math::{Extent2, Vec2};

use uuid::Uuid;

use crate::patch::*;
use crate::node::Node;

pub trait Layer: Node {
	fn crop(&self, offset: Vec2<u32>, size: Extent2<u32>) -> (CropPatch, Box<dyn PatchImpl>);
}

pub trait LayerImpl: Layer {
	fn as_any(&self) -> &dyn Any;
}

impl<T> LayerImpl for T
where
	T: Layer + Any,
{
	fn as_any(&self) -> &dyn Any {
		self
	}
}

pub struct CropPatch {
	pub target: Uuid,
	pub offset: Vec2<u32>,
	pub size: Extent2<u32>,
}

impl Patch for CropPatch {
	fn target(&self) -> Uuid { self.target }
}
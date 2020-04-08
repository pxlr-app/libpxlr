use std::any::Any;
use math::{Extent2, Vec2};

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
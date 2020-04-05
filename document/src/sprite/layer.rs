use std::any::Any;

use crate::node::Node;

pub trait Layer: Node {}

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
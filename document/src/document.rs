use std::any::Any;

use crate::node::Node;

use math::Vec2;

pub trait Document: Node {
	fn position(&self) -> Vec2<f32>;
}

pub trait DocumentImpl: Document {
	fn as_any(&self) -> &dyn Any;
}

impl<T> DocumentImpl for T
where
	T: Document + Any,
{
	fn as_any(&self) -> &dyn Any {
		self
	}
}

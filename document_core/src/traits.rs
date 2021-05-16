use crate::NodeType;
use color::{Channel, Rgba};
use std::sync::Arc;
use uuid::Uuid;
use vek::{geom::repr_c::Rect, vec::repr_c::vec2::Vec2};

pub trait Node {
	fn id(&self) -> &Uuid;
	fn set_id(&mut self, id: Uuid);
	fn name(&self) -> &str;
	fn set_name(&mut self, name: String);
}

pub trait HasChildren {
	fn is_child_valid(&self, _node: &NodeType) -> bool;
	fn children(&self) -> &Vec<Arc<NodeType>>;
	fn set_children(&mut self, children: Vec<Arc<NodeType>>);
}

pub trait HasBounds {
	fn bounds(&self) -> Rect<i32, i32>;
	fn set_position(&mut self, position: Vec2<i32>);
}

pub trait HasContent {
	fn content(&self) -> &str;
	fn set_content(&mut self, content: String);
}

pub trait HasChannel {
	fn channel(&self) -> Channel;
	fn set_channel(&mut self, channel: Channel);
}

pub trait HasColors {
	fn colors(&self) -> &Vec<Rgba>;
	fn set_colors(&mut self, colors: Vec<Rgba>);
}

use math::Vec2;
use uuid::Uuid;

use crate::sprite::{BlendMode, Stencil};

pub struct ApplyStencilPatch
{
	pub target: Uuid,
	pub stencil: Stencil,
	pub offset: Vec2<u32>,
	pub blend_mode: BlendMode,
}
use crate::patch::{IPatch, PatchMode};
use crate::sprite::*;
use math::blend::BlendMode;
use math::Vec2;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplyStencilPatch<S>
where
	S: IStencil,
{
	pub target: Uuid,
	pub stencil: S,
	pub offset: Vec2<u32>,
	pub blend_mode: BlendMode,
}

impl<S> IPatch for ApplyStencilPatch<S>
where
	S: IStencil,
{
	fn mode(&self) -> PatchMode {
		PatchMode::DATA
	}
}

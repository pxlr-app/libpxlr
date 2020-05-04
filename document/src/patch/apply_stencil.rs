use crate::patch::{IPatch, PatchMode};
use crate::sprite::{StencilI, StencilIXYZ, StencilRGB, StencilRGBA, StencilRGBAXYZ, StencilUV};
use math::blend::BlendMode;
use math::Vec2;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

macro_rules! define_stencil_patch {
	($name:ident $stencil:ident) => {
		#[derive(Debug, Serialize, Deserialize)]
		pub struct $name {
			pub target: Uuid,
			pub stencil: $stencil,
			pub offset: Vec2<u32>,
			pub blend_mode: BlendMode,
		}

		impl IPatch for $name {
			fn mode(&self) -> PatchMode {
				PatchMode::DATA
			}
		}
	};
}

define_stencil_patch!(ApplyStencilIPatch StencilI);
define_stencil_patch!(ApplyStencilIXYZPatch StencilIXYZ);
define_stencil_patch!(ApplyStencilUVPatch StencilUV);
define_stencil_patch!(ApplyStencilRGBPatch StencilRGB);
define_stencil_patch!(ApplyStencilRGBAPatch StencilRGBA);
define_stencil_patch!(ApplyStencilRGBAXYZPatch StencilRGBAXYZ);

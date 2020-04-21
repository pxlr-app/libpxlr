use math::Vec2;
use uuid::Uuid;

use crate::sprite::{BlendMode, StencilI, StencilRGB, StencilRGBA, StencilRGBAXYZ, StencilUV};

macro_rules! impl_stencil_patch {
	($name:ident $stencil:ident) => {
		pub struct $name {
			pub target: Uuid,
			pub stencil: $stencil,
			pub offset: Vec2<u32>,
			pub blend_mode: BlendMode,
		}
	};
}

impl_stencil_patch!(ApplyStencilIPatch StencilI);
impl_stencil_patch!(ApplyStencilUVPatch StencilUV);
impl_stencil_patch!(ApplyStencilRGBPatch StencilRGB);
impl_stencil_patch!(ApplyStencilRGBAPatch StencilRGBA);
impl_stencil_patch!(ApplyStencilRGBAXYZPatch StencilRGBAXYZ);

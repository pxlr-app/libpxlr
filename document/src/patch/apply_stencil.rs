use crate::sprite::{StencilI, StencilIXYZ, StencilRGB, StencilRGBA, StencilRGBAXYZ, StencilUV};
use math::blend::BlendMode;
use math::Vec2;
use uuid::Uuid;

macro_rules! define_stencil_patch {
	($name:ident $stencil:ident) => {
		pub struct $name {
			pub target: Uuid,
			pub stencil: $stencil,
			pub offset: Vec2<u32>,
			pub blend_mode: BlendMode,
		}
	};
}

define_stencil_patch!(ApplyStencilIPatch StencilI);
define_stencil_patch!(ApplyStencilIXYZPatch StencilIXYZ);
define_stencil_patch!(ApplyStencilUVPatch StencilUV);
define_stencil_patch!(ApplyStencilRGBPatch StencilRGB);
define_stencil_patch!(ApplyStencilRGBAPatch StencilRGBA);
define_stencil_patch!(ApplyStencilRGBAXYZPatch StencilRGBAXYZ);

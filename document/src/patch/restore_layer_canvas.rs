use math::Extent2;
use uuid::Uuid;

use crate::sprite::color::*;

macro_rules! impl_restore_canvas_patch {
	($name:ident $color:ident) => {
		pub struct $name {
			pub target: Uuid,
			pub name: String,
			pub size: Extent2<u32>,
			pub data: Vec<$color>,
		}
	};
}

impl_restore_canvas_patch!(RestoreLayerCanvasIPatch I);
impl_restore_canvas_patch!(RestoreLayerCanvasUVPatch UV);
impl_restore_canvas_patch!(RestoreLayerCanvasRGBPatch RGB);
impl_restore_canvas_patch!(RestoreLayerCanvasRGBAPatch RGBA);
impl_restore_canvas_patch!(RestoreLayerCanvasRGBAXYZPatch RGBAXYZ);

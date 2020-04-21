use math::Extent2;
use uuid::Uuid;

use crate::sprite::color::*;

macro_rules! define_restore_canvas_patch {
	($name:ident $color:ident) => {
		pub struct $name {
			pub target: Uuid,
			pub name: String,
			pub size: Extent2<u32>,
			pub data: Vec<$color>,
		}
	};
}

define_restore_canvas_patch!(RestoreLayerCanvasIPatch I);
define_restore_canvas_patch!(RestoreLayerCanvasUVPatch UV);
define_restore_canvas_patch!(RestoreLayerCanvasRGBPatch RGB);
define_restore_canvas_patch!(RestoreLayerCanvasRGBAPatch RGBA);
define_restore_canvas_patch!(RestoreLayerCanvasRGBAXYZPatch RGBAXYZ);

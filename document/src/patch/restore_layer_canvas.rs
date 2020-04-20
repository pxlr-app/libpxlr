use math::Extent2;
use uuid::Uuid;

use crate::sprite::Pixel;

pub struct RestoreLayerCanvasPatch
{
	pub target: Uuid,
	pub name: String,
	pub size: Extent2<u32>,
	pub data: Vec<Pixel>,
}
pub struct Document {
	pub root: Group,
}

pub trait Node {}

pub struct Group {
	pub name: String,
	pub children: Vec<Box<dyn Node>>,
}

impl Node for Group {}

pub struct Label {
	pub name: String,
}

impl Node for Label {}

pub struct Sprite {
	pub name: String,
	pub layers: Vec<Box<dyn SpriteLayer>>,
	pub width: i32,
	pub height: i32,
}

impl Node for Sprite {}

pub trait SpriteLayer {}

pub struct SpriteLayerGroup {
	pub name: String,
	pub children: Vec<Box<dyn SpriteLayer>>,
}

impl SpriteLayer for SpriteLayerGroup {}

pub struct SpriteLayerDataRGBA {
	pub name: String,
	pub width: i32,
	pub height: i32,
	pub data: Vec<i32>,
}

impl SpriteLayer for SpriteLayerDataRGBA {}

pub struct SpriteLayerDataI {
	pub name: String,
	pub width: i32,
	pub height: i32,
	pub data: Vec<i8>,
}

impl SpriteLayer for SpriteLayerDataI {}

pub struct SpriteLayerDataUV {
	pub name: String,
	pub width: i32,
	pub height: i32,
	pub data: Vec<i32>,
}

impl SpriteLayer for SpriteLayerDataUV {}
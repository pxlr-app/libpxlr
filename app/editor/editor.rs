use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ViewportBounds {
	pub top: f32,
	pub right: f32,
	pub bottom: f32,
	pub left: f32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum ViewportOptions {
	Blank,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Viewport {
	pub key: String,
	pub bounds: ViewportBounds,
	pub options: ViewportOptions,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "cmd")]
pub enum Command {
	Init,
	Ping,
	AddViewport { viewport: Viewport },
	RemoveViewport { key: String },
	UpdateViewport { viewport: Viewport },
	Resize { width: i32, height: i32 },
	Draw,
}

use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum BlendMode {
	Normal,
	Multiply,
	Divide,
	Add,
	Subtract,
	Difference,
	Screen,
	Darken,
	Lighten,
}

pub trait Blend {
	type Output;

	fn blend(from: &Self, to: &Self, mode: &BlendMode) -> Self::Output;
}

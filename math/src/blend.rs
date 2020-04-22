#[derive(Copy, Clone, Debug)]
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

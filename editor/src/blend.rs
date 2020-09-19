#[derive(Copy, Clone, Debug)]
pub enum Blend {
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

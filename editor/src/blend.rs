// https://www.w3.org/TR/compositing-1/

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

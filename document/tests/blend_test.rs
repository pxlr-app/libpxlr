use document::sprite::{Blend, BlendMode};

#[derive(Debug, PartialEq)]
struct Color(u32);

impl Blend for Color {
	type Output = Color;
	fn blend(from: &Color, to: &Color, mode: &BlendMode) -> Color {
		match mode {
			BlendMode::Normal => Color(to.0),
			BlendMode::Add => Color(from.0 + to.0),
			BlendMode::Subtract => Color(from.0 - to.0),
			BlendMode::Multiply => Color(from.0 * to.0),
			BlendMode::Divide => Color(from.0 / to.0),
			_ => Color(to.0),
		}
	}
}

#[test]
fn it_blends() {
	assert_eq!(
		Blend::blend(&Color(128), &Color(32), &BlendMode::Normal),
		Color(32)
	);
	assert_eq!(
		Blend::blend(&Color(128), &Color(32), &BlendMode::Add),
		Color(160)
	);
	assert_eq!(
		Blend::blend(&Color(128), &Color(32), &BlendMode::Subtract),
		Color(96)
	);
	assert_eq!(
		Blend::blend(&Color(128), &Color(32), &BlendMode::Multiply),
		Color(4096)
	);
	assert_eq!(
		Blend::blend(&Color(128), &Color(32), &BlendMode::Divide),
		Color(4)
	);
}

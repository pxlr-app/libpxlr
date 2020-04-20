use math::Lerp;
use std::default::Default;
use std::ops::{Add, Sub, Mul, Div};

use crate::sprite::{Blend, BlendMode};

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Pixel {
	Nil,
	I(u8),
	UV {
		u: f32,
		v: f32
	},
	RGB {
		r: u8,
		g: u8,
		b: u8
	},
	RGBA {
		r: u8,
		g: u8,
		b: u8,
		a: u8
	},
	RGBAXYZ {
		r: u8,
		g: u8,
		b: u8,
		a: u8,
		x: f32,
		y: f32,
		z: f32
	}
}

impl Default for Pixel {
	fn default() -> Self { Pixel::Nil }
}

impl Add for Pixel {
	type Output = Pixel;

	fn add(self, other: Self) -> Self {
		Pixel::Nil
	}
}

impl Sub for Pixel {
	type Output = Pixel;

	fn sub(self, other: Self) -> Self {
		Pixel::Nil
	}
}

impl Mul for Pixel {
	type Output = Pixel;

	fn mul(self, other: Self) -> Self {
		Pixel::Nil
	}
}

impl Div for Pixel {
	type Output = Pixel;

	fn div(self, other: Self) -> Self {
		Pixel::Nil
	}
}

impl Blend for Pixel {
	type Output = Pixel;
	fn blend(from: &Self, to: &Self, mode: &BlendMode) -> Self {
		match mode {
			BlendMode::Normal => *to,
			BlendMode::Add => *from + *to,
			BlendMode::Subtract => *from - *to,
			BlendMode::Multiply => *from * *to,
			BlendMode::Divide => *from / *to,
			_ => *to,
		}
	}
}

impl Lerp<f32> for Pixel {
	type Output = Pixel;

	fn lerp_unclamped(from: Self, to: Self, factor: f32) -> Self::Output {
		Pixel::Nil
	}
}
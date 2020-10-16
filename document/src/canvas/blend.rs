use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// Based on [SVG specs](https://www.w3.org/TR/compositing-1/#porterduffcompositingoperators)
#[derive(Debug, Display, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum Blend {
	Normal,
	Multiply,
	Screen,
	Overlay,
	Darken,
	Lighten,
	ColorDodge,
	ColorBurn,
	HardLight,
	SoftLight,
	Difference,
	Exclusion,
}

impl Blend {
	#[inline(always)]
	pub fn blend(&self, a: f32, b: f32) -> f32 {
		match self {
			Blend::Normal => normal(a, b),
			Blend::Multiply => multiply(a, b),
			Blend::Screen => screen(a, b),
			Blend::Overlay => overlay(a, b),
			Blend::Darken => darken(a, b),
			Blend::Lighten => lighten(a, b),
			Blend::ColorDodge => colordodge(a, b),
			Blend::ColorBurn => colorburn(a, b),
			Blend::HardLight => hardlight(a, b),
			Blend::SoftLight => softlight(a, b),
			Blend::Difference => difference(a, b),
			Blend::Exclusion => exclusion(a, b),
		}
	}
}

impl Default for Blend {
	fn default() -> Self {
		Blend::Normal
	}
}

/// Based on [SVG specs](https://www.w3.org/TR/compositing-1/#porterduffcompositingoperators)
#[derive(Debug, Display, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum Compose {
	Clear,
	Copy,
	Destination,
	SourceOver,
	DestinationOver,
	SourceIn,
	DestinationIn,
	SourceOut,
	DestinationOut,
	SourceAtop,
	DestinationAtop,
	XOR,
	Lighter,
}

impl Compose {
	pub fn compose(&self, fa: f32, ba: f32) -> (f32, f32) {
		match self {
			Compose::Clear => (0., 0.),
			Compose::Copy => (1., 0.),
			Compose::Destination => (0., 1.),
			Compose::SourceOver => (1., 1. - fa),
			Compose::DestinationOver => (1. - ba, 1.),
			Compose::SourceIn => (ba, 0.),
			Compose::DestinationIn => (0., fa),
			Compose::SourceOut => (1. - ba, 0.),
			Compose::DestinationOut => (0., 1. - fa),
			Compose::SourceAtop => (ba, 1. - fa),
			Compose::DestinationAtop => (1. - ba, fa),
			Compose::XOR => (1. - ba, 1. - fa),
			Compose::Lighter => (1., 1.),
		}
	}
}

impl Default for Compose {
	fn default() -> Self {
		Compose::Lighter
	}
}

#[inline(always)]
fn normal(_: f32, cf: f32) -> f32 {
	cf
}

#[inline(always)]
fn multiply(cb: f32, cf: f32) -> f32 {
	cb * cf
}

#[inline(always)]
fn screen(cb: f32, cf: f32) -> f32 {
	cb + cf - cb * cf
}

#[inline(always)]
fn overlay(cb: f32, cf: f32) -> f32 {
	hardlight(cf, cb)
}

#[inline(always)]
fn darken(cb: f32, cf: f32) -> f32 {
	cb.min(cf)
}

#[inline(always)]
fn lighten(cb: f32, cf: f32) -> f32 {
	cb.max(cf)
}

#[inline(always)]
fn colordodge(cb: f32, cf: f32) -> f32 {
	if cb == 0. {
		0.
	} else if cf == 1. {
		1.
	} else {
		(cb / (1. - cf)).min(1.)
	}
}

#[inline(always)]
fn colorburn(cb: f32, cf: f32) -> f32 {
	if cb == 1. {
		1.
	} else if cf == 0. {
		0.
	} else {
		1. - ((1. - cb) / cf).min(1.)
	}
}

#[inline(always)]
fn hardlight(cb: f32, cf: f32) -> f32 {
	if cf <= 0.5 {
		multiply(cb, cf)
	} else {
		screen(cb, cf)
	}
}

#[inline(always)]
fn softlight(cb: f32, cf: f32) -> f32 {
	if cf <= 0.5 {
		cb - (1. - 2. * cf) * cb * (1. - cb)
	} else {
		let d = if cb <= 0.25 {
			((16. * cb - 12.) * cb + 4.) * cb
		} else {
			cb.sqrt()
		};
		cb + (2. * cf - 1.) * (d - cb)
	}
}

#[inline(always)]
fn difference(cb: f32, cf: f32) -> f32 {
	cb - cf
}

#[inline(always)]
fn exclusion(cb: f32, cf: f32) -> f32 {
	cb + cf - 2. * cb * cf
}

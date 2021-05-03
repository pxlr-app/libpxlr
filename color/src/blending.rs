use serde::{Deserialize, Serialize};

/// Based on [SVG specs](https://www.w3.org/TR/compositing-1/#porterduffcompositingoperators)
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum Blending {
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

impl Blending {
	#[inline(always)]
	pub fn blend(&self, a: f32, b: f32) -> f32 {
		match self {
			Blending::Normal => normal(a, b),
			Blending::Multiply => multiply(a, b),
			Blending::Screen => screen(a, b),
			Blending::Overlay => overlay(a, b),
			Blending::Darken => darken(a, b),
			Blending::Lighten => lighten(a, b),
			Blending::ColorDodge => colordodge(a, b),
			Blending::ColorBurn => colorburn(a, b),
			Blending::HardLight => hardlight(a, b),
			Blending::SoftLight => softlight(a, b),
			Blending::Difference => difference(a, b),
			Blending::Exclusion => exclusion(a, b),
		}
	}
}

impl Default for Blending {
	fn default() -> Self {
		Blending::Normal
	}
}

/// Based on [SVG specs](https://www.w3.org/TR/compositing-1/#porterduffcompositingoperators)
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum Compositing {
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

impl Compositing {
	pub fn compose(&self, fa: f32, ba: f32) -> (f32, f32) {
		match self {
			Compositing::Clear => (0., 0.),
			Compositing::Copy => (1., 0.),
			Compositing::Destination => (0., 1.),
			Compositing::SourceOver => (1., 1. - fa),
			Compositing::DestinationOver => (1. - ba, 1.),
			Compositing::SourceIn => (ba, 0.),
			Compositing::DestinationIn => (0., fa),
			Compositing::SourceOut => (1. - ba, 0.),
			Compositing::DestinationOut => (0., 1. - fa),
			Compositing::SourceAtop => (ba, 1. - fa),
			Compositing::DestinationAtop => (1. - ba, fa),
			Compositing::XOR => (1. - ba, 1. - fa),
			Compositing::Lighter => (1., 1.),
		}
	}
}

impl Default for Compositing {
	fn default() -> Self {
		Compositing::Lighter
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

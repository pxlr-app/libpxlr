use std::any::Any;
use std::ops::{Add, Sub, Mul, Div};
use math::{Extent2, Lerp, Vec2};

use uuid::Uuid;

use crate::node::Node;
use crate::patch::*;

pub trait Layer: Node {
	fn crop(&self, offset: Vec2<u32>, size: Extent2<u32>) -> (CropPatch, Box<dyn PatchImpl>);
	fn resize(
		&self,
		size: Extent2<u32>,
		interpolation: Interpolation,
	) -> (ResizePatch, Box<dyn PatchImpl>);
}

pub trait LayerImpl: Layer {
	fn as_any(&self) -> &dyn Any;
}

impl<T> LayerImpl for T
where
	T: Layer + Any,
{
	fn as_any(&self) -> &dyn Any {
		self
	}
}

pub struct CropPatch {
	pub target: Uuid,
	pub offset: Vec2<u32>,
	pub size: Extent2<u32>,
}

impl Patch for CropPatch {
	fn target(&self) -> Uuid {
		self.target
	}
}

#[derive(Copy, Clone)]
pub enum Interpolation {
	Nearest,
	Linear,
	Bilinear,
	Cubic,
	Bicubic,
	// Sinc,
	// Lanczos,
	// Box,
}

#[inline]
fn blerp<T>(c00: T, c10: T, c01: T, c11: T, factor_x: f32, factor_y: f32) -> T
where
	T: Lerp<f32, Output = T>,
{
	Lerp::lerp_precise(Lerp::lerp_precise(c00, c10, factor_x), Lerp::lerp_precise(c01, c11, factor_x), factor_y)
}

#[inline]
fn p2i(x: u32, y: u32, w: u32) -> usize {
	(x * w + y) as usize
}

impl Interpolation {
	pub fn sample<T>(&self, src_size: &Extent2<u32>, src_data: &Vec<T>, dst_size: &Extent2<u32>, dst_data: &mut Vec<T>)
	where
		T: Default + Copy + Lerp<f32, Output = T>,
	{
		let sw = src_size.w;
		let sh = src_size.h;
		let dw = dst_size.w;
		let dh = dst_size.h;
		match self {
			Interpolation::Bilinear => {
				for x in 0..dst_size.w {
					for y in 0..dst_size.h {
						let gx = (x as f32) / (dw * (sw - 1)) as f32;
						let gy = (y as f32) / (dh * (sh - 1)) as f32;
						let gxi = gx as u32;
						let gyi = gy as u32;

						let c00 = src_data[p2i(gxi + 0, gxi + 0, sw)];
						let c10 = src_data[p2i(gxi + 1, gxi + 0, sw)];
						let c01 = src_data[p2i(gxi + 0, gxi + 1, sw)];
						let c11 = src_data[p2i(gxi + 1, gxi + 1, sw)];

						let fx = gx - (gxi as f32);
						let fy = gy - (gyi as f32);

						dst_data[p2i(x, y, dw)] = blerp(c00, c10, c01, c11, fx, fy);
					}
				}
			},
			_ => ()
		}
	}
}

pub struct ResizePatch {
	pub target: Uuid,
	pub size: Extent2<u32>,
	pub interpolation: Interpolation,
}

impl Patch for ResizePatch {
	fn target(&self) -> Uuid {
		self.target
	}
}

use crate::{Extent2, Lerp, Mat2, Vec2};

#[derive(Copy, Clone)]
pub enum Interpolation {
	Nearest,
	Bilinear,
}

#[inline]
fn p2i(x: u32, y: u32, w: u32) -> usize {
	(x * w + y) as usize
}

#[inline]
fn clamp(x: f32, min: f32, max: f32) -> f32 {
	if x >= max {
		max
	} else if x <= min {
		min
	} else {
		x
	}
}

impl Interpolation {
	pub fn interpolate<T>(
		&self,
		src_size: &Extent2<u32>,
		src_data: &Vec<T>,
		dst_size: &Extent2<u32>,
		dst_data: &mut Vec<T>,
		transform: Mat2<f32>,
	) where
		T: Default + Copy + Lerp<f32, Output = T>,
	{
		let sw = src_size.w;
		let sh = src_size.h;
		let dw = dst_size.w;
		let dh = dst_size.h;
		match self {
			Interpolation::Nearest => {
				for x in 0..dst_size.w {
					for y in 0..dst_size.h {
						let pf = Vec2::new(x as f32, y as f32);
						let pf = pf * transform;
						let pu = pf.map(|x| x.round() as u32);
						dst_data[p2i(x, y, dh)] = src_data[p2i(pu.x, pu.y, sh)];
					}
				}
			}
			Interpolation::Bilinear => {
				for x in 0..dst_size.w {
					for y in 0..dst_size.h {
						let pf = Vec2::new(x as f32, y as f32);
						let pf = pf * transform;
						let pu = pf.map(|x| x.floor() as u32);

						let c00 = src_data[p2i(pu.x + 0, pu.y + 0, sw)];
						let c10 = src_data[p2i(pu.x + 1, pu.y + 0, sw)];
						let c01 = src_data[p2i(pu.x + 0, pu.y + 1, sw)];
						let c11 = src_data[p2i(pu.x + 1, pu.y + 1, sw)];
						let f = pf - pu.map(|x| x as f32);

						dst_data[p2i(x, y, dw)] = Lerp::lerp_precise(
							Lerp::lerp_precise(c00, c10, f.x),
							Lerp::lerp_precise(c01, c11, f.x),
							f.y,
						);
					}
				}
			}
		}
	}
}

use crate::prelude::*;

#[inline(always)]
fn lerp_pixel(channel: Channel, from: &Pixel, to: &Pixel, weight: f32, dst: &mut Pixel) {
	use math::Lerp;
	let size = channel.size();
	assert!(from.len() == size);
	assert!(to.len() == size);
	assert!(dst.len() == size);

	if channel & Channel::I == Channel::I {
		unsafe {
			let from = channel.unsafe_i(from);
			let to = channel.unsafe_i(to);
			*channel.unsafe_i_mut(dst) = Lerp::lerp(from, to, weight);
		}
	}
	if channel & Channel::RGB == Channel::RGB {
		unsafe {
			let from = channel.unsafe_rgb(from);
			let to = channel.unsafe_rgb(to);
			*channel.unsafe_rgb_mut(dst) = Lerp::lerp(from, to, weight);
		}
	}
	if channel & Channel::A == Channel::A {
		unsafe {
			let from = channel.unsafe_a(from);
			let to = channel.unsafe_a(to);
			*channel.unsafe_a_mut(dst) = Lerp::lerp(from, to, weight);
		}
	}
	if channel & Channel::UV == Channel::UV {
		unsafe {
			let from = channel.unsafe_uv(from);
			let to = channel.unsafe_uv(to);
			*channel.unsafe_uv_mut(dst) = Lerp::lerp(from, to, weight);
		}
	}
	if channel & Channel::XYZ == Channel::XYZ {
		unsafe {
			let from = channel.unsafe_xyz(from);
			let to = channel.unsafe_xyz(to);
			*channel.unsafe_xyz_mut(dst) = Lerp::lerp(from, to, weight);
		}
	}
}

// #[inline(always)]
// fn blend_cubic(v0: f32, v1: f32, v2: f32, v3: f32, factor: f32) -> f32 {
// 	#[rustfmt::skip]
// 	let r = v1 + 0.5 * factor * (v2 - v0 + factor * (2.0 * v0 - 5.0 * v1 + 4.0 * v2 - v3 + factor * (3.0 * (v1 - v2) + v3 - v0)));
// 	r
// }

// #[inline(always)]
// fn cubic_pixel(
// 	channel: Channel,
// 	p0: &Pixel,
// 	p1: &Pixel,
// 	p2: &Pixel,
// 	p3: &Pixel,
// 	weight: f32,
// 	dst: &mut Pixel,
// ) {
// 	use math::Lerp;
// 	let size = channel.size();
// 	assert!(p0.len() == size);
// 	assert!(p1.len() == size);
// 	assert!(p2.len() == size);
// 	assert!(p3.len() == size);
// 	assert!(dst.len() == size);

// 	if channel & Channel::I == Channel::I {
// 		unsafe {
// 			let p1 = channel.unsafe_i(p1);
// 			let p2 = channel.unsafe_i(p2);
// 			*channel.unsafe_i_mut(dst) = Lerp::lerp(p1, p2, weight);
// 		}
// 	}
// 	if channel & Channel::RGB == Channel::RGB {
// 		unsafe {
// 			let p0 = channel.unsafe_rgb(p0);
// 			let p1 = channel.unsafe_rgb(p1);
// 			let p2 = channel.unsafe_rgb(p2);
// 			let p3 = channel.unsafe_rgb(p3);
// 			let r = blend_cubic(p0.r as f32, p1.r as f32, p2.r as f32, p3.r as f32, weight);
// 			let g = blend_cubic(p0.g as f32, p1.g as f32, p2.g as f32, p3.g as f32, weight);
// 			let b = blend_cubic(p0.b as f32, p1.b as f32, p2.b as f32, p3.b as f32, weight);
// 			*channel.unsafe_rgb_mut(dst) = RGB::new(r as u8, g as u8, b as u8);
// 		}
// 	}
// 	if channel & Channel::A == Channel::A {
// 		unsafe {
// 			let p0 = channel.unsafe_a(p0);
// 			let p1 = channel.unsafe_a(p1);
// 			let p2 = channel.unsafe_a(p2);
// 			let p3 = channel.unsafe_a(p3);
// 			let a = blend_cubic(p0.a as f32, p1.a as f32, p2.a as f32, p3.a as f32, weight);
// 			*channel.unsafe_a_mut(dst) = A::new(a as u8);
// 		}
// 	}
// 	if channel & Channel::UV == Channel::UV {
// 		unsafe {
// 			let p0 = channel.unsafe_uv(p0);
// 			let p1 = channel.unsafe_uv(p1);
// 			let p2 = channel.unsafe_uv(p2);
// 			let p3 = channel.unsafe_uv(p3);
// 			let u = blend_cubic(p0.u as f32, p1.u as f32, p2.u as f32, p3.u as f32, weight);
// 			let v = blend_cubic(p0.v as f32, p1.v as f32, p2.v as f32, p3.v as f32, weight);
// 			*channel.unsafe_uv_mut(dst) = UV::new(u, v);
// 		}
// 	}
// 	if channel & Channel::XYZ == Channel::XYZ {
// 		unsafe {
// 			let p0 = channel.unsafe_xyz(p0);
// 			let p1 = channel.unsafe_xyz(p1);
// 			let p2 = channel.unsafe_xyz(p2);
// 			let p3 = channel.unsafe_xyz(p3);
// 			let x = blend_cubic(p0.x as f32, p1.x as f32, p2.x as f32, p3.x as f32, weight);
// 			let y = blend_cubic(p0.y as f32, p1.y as f32, p2.y as f32, p3.y as f32, weight);
// 			let z = blend_cubic(p0.z as f32, p1.z as f32, p2.z as f32, p3.z as f32, weight);
// 			*channel.unsafe_xyz_mut(dst) = XYZ::new(x, y, z);
// 		}
// 	}
// }

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Interpolation {
	Nearest,
	Bilinear,
	// Bicubic,
}

impl Interpolation {
	#[inline(always)]
	pub fn interpolate_into<'src, 'dst>(
		&self,
		pos: &Vec2<f32>,
		channels: Channel,
		size: &Extent2<u32>,
		src: &'src Pixels,
		dst: &'dst mut Pixel,
	) {
		#[inline(always)]
		fn get_pixel(stride: usize, data: &Pixels, x: f32, y: f32, w: u32) -> &Pixel {
			let index = (x as u32 + w * y as u32) as usize;
			return &data[(index * stride)..((index + 1) * stride)];
		}

		let stride = channels.size();
		assert!(src.len() == size.w as usize * size.h as usize * stride);
		assert!(dst.len() == stride);
		match self {
			Interpolation::Nearest => {
				let x = pos.x.round();
				let y = pos.y.round();

				if x >= 0f32 && x < size.w as f32 && y >= 0f32 && y < size.h as f32 {
					dst.copy_from_slice(get_pixel(stride, &src, x, y, size.w));
				}
			}
			Interpolation::Bilinear => {
				let l = pos.x.floor();
				let r = l + 1f32;
				let t = pos.y.floor();
				let b = t + 1f32;
				// println!("Bilinear {},{},{},{}", l, r, t, b);

				if l >= 0f32 && r < size.w as f32 && t >= 0f32 && b < size.h as f32 {
					let hw = pos.x - l;
					let vw = pos.y - t;
					let mut tmp = vec![0u8; stride * 2];
					let (tl, tr, bl, br) = (
						get_pixel(stride, &src, l, t, size.w),
						get_pixel(stride, &src, r, t, size.w),
						get_pixel(stride, &src, l, b, size.w),
						get_pixel(stride, &src, r, b, size.w),
					);
					lerp_pixel(channels, tl, tr, hw, &mut tmp[0..stride]);
					lerp_pixel(channels, bl, br, hw, &mut tmp[stride..]);
					lerp_pixel(channels, &tmp[0..stride], &tmp[stride..], vw, dst);
				}
			} // Interpolation::Bicubic => {
			  // 	let l = pos.x.floor() - 1f32;
			  // 	let r = l + 4f32;
			  // 	let t = pos.y.floor() - 1f32;
			  // 	let b = t + 4f32;
			  // 	// println!("Bicubic {},{},{},{}", l, r, t, b);

			  // 	if l >= 0f32 && r < size.w as f32 && t >= 0f32 && b < size.h as f32 {
			  // 		let hw = pos.x - (l + 1f32);
			  // 		let vw = pos.y - (t + 1f32);
			  // 		let mut cols = vec![0u8; stride * 4];

			  // 		for row in (t as usize)..(b as usize) {
			  // 			let i = row as usize - t as usize;
			  // 			let (p0, p1, p2, p3) = (
			  // 				get_pixel(stride, &src, l + 0f32, row as f32, size.w),
			  // 				get_pixel(stride, &src, l + 1f32, row as f32, size.w),
			  // 				get_pixel(stride, &src, l + 2f32, row as f32, size.w),
			  // 				get_pixel(stride, &src, l + 3f32, row as f32, size.w),
			  // 			);
			  // 			cubic_pixel(
			  // 				channels,
			  // 				p0,
			  // 				p1,
			  // 				p2,
			  // 				p3,
			  // 				hw,
			  // 				&mut cols[(i * stride)..((i + 1) * stride)],
			  // 			);
			  // 		}
			  // 		// println!("cols={:?}, vw={}", cols, vw);
			  // 		cubic_pixel(
			  // 			channels,
			  // 			&cols[(0 * stride)..(1 * stride)],
			  // 			&cols[(1 * stride)..(2 * stride)],
			  // 			&cols[(2 * stride)..(3 * stride)],
			  // 			&cols[(3 * stride)..(4 * stride)],
			  // 			vw,
			  // 			dst,
			  // 		);
			  // 	}
			  // }
		};
	}
}

#[cfg(te2st)]
mod tests {
	use super::*;

	#[test]
	fn test_interpolate_nearest() {
		use Interpolation::Nearest;
		let channel = Channel::A;
		let size = Extent2::new(2, 2);
		let src = vec![5u8, 10, 15, 20];
		let mut dst = vec![0u8];

		Nearest.interpolate_into(&Vec2::new(0., 0.), channel, &size, &src, &mut dst);
		assert_eq!(dst, &[5u8][..]);
		Nearest.interpolate_into(&Vec2::new(1., 0.), channel, &size, &src, &mut dst);
		assert_eq!(dst, &[10u8][..]);
		Nearest.interpolate_into(&Vec2::new(0., 1.), channel, &size, &src, &mut dst);
		assert_eq!(dst, &[15u8][..]);
		Nearest.interpolate_into(&Vec2::new(1., 1.), channel, &size, &src, &mut dst);
		assert_eq!(dst, &[20u8][..]);
		Nearest.interpolate_into(&Vec2::new(0.5, 0.), channel, &size, &src, &mut dst);
		assert_eq!(dst, &[10u8][..]);
		Nearest.interpolate_into(&Vec2::new(0.5, 0.5), channel, &size, &src, &mut dst);
		assert_eq!(dst, &[20u8][..]);
	}

	#[test]
	fn test_interpolate_bilinear() {
		use Interpolation::Bilinear;
		let channel = Channel::A;
		let size = Extent2::new(2, 2);
		let src = vec![5u8, 10, 15, 20];
		let mut dst = vec![0u8];

		Bilinear.interpolate_into(&Vec2::new(0., 0.), channel, &size, &src, &mut dst);
		assert_eq!(dst, &[5u8][..]);
		Bilinear.interpolate_into(&Vec2::new(1., 0.), channel, &size, &src, &mut dst);
		assert_eq!(dst, &[10u8][..]);
		Bilinear.interpolate_into(&Vec2::new(0., 1.), channel, &size, &src, &mut dst);
		assert_eq!(dst, &[15u8][..]);
		Bilinear.interpolate_into(&Vec2::new(1., 1.), channel, &size, &src, &mut dst);
		assert_eq!(dst, &[20u8][..]);
		Bilinear.interpolate_into(&Vec2::new(0.5, 0.), channel, &size, &src, &mut dst);
		assert_eq!(dst, &[8u8][..]);
		Bilinear.interpolate_into(&Vec2::new(0.5, 0.5), channel, &size, &src, &mut dst);
		assert_eq!(dst, &[13u8][..]);
	}

	#[test]
	fn test_interpolate_cubic() {
		// use Interpolation::Bicubic;
		// let channel = Channel::A;
		// let size = Extent2::new(4, 4);
		// let src = vec![
		// 	5u8, 10, 15, 20, 25, 30, 35, 40, 45, 50, 55, 60, 65, 70, 75, 80,
		// ];
		// let mut dst = vec![0u8];
		// Bicubic.interpolate_into(&Vec2::new(1., 1.), channel, &size, &src, &mut dst);
		// assert_eq!(dst, &[30u8][..]);
		// Bicubic.interpolate_into(&Vec2::new(2., 1.), channel, &size, &src, &mut dst);
		// assert_eq!(dst, &[30u8][..]);
		// Bicubic.interpolate_into(&Vec2::new(1., 3.), channel, &size, &src, &mut dst);
		// assert_eq!(dst, &[3u8][..]);
		// Bicubic.interpolate_into(&Vec2::new(3., 3.), channel, &size, &src, &mut dst);
		// assert_eq!(dst, &[0u8][..]);
		// Bicubic.interpolate_into(&Vec2::new(2.5, 1.5), channel, &size, &src, &mut dst);
		// assert_eq!(dst, &[0u8][..]);
		// Bicubic.interpolate_into(&Vec2::new(2.5, 2.5), channel, &size, &src, &mut dst);
		// assert_eq!(dst, &[0u8][..]);
	}
}

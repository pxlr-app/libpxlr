use crate::{blend::Blend, interpolation::Interpolation};
use document::prelude::*;
#[cfg(feature = "rayon")]
use rayon::prelude::*;

// https://github.com/image-rs/imageproc/blob/master/src/geometric_transformations.rs

pub fn transform_into<'src, 'dst>(
	transform: &Mat3<f32>,
	interpolation: &Interpolation,
	size: &Extent2<u32>,
	channels: Channel,
	src: &'src Pixels,
	dst: &'dst mut Pixels,
) {
	let stride = channels.size();
	assert!(src.len() == size.w as usize * size.h as usize * stride);
	assert!(dst.len() == size.w as usize * size.h as usize * stride);

	let pitch = size.w as usize * stride;

	#[cfg(feature = "rayon")]
	let chunks = dst.par_chunks_mut(pitch);
	#[cfg(not(feature = "rayon"))]
	let chunks = dst.chunks_mut(pitch);

	chunks.enumerate().for_each(|(y, row)| {
		use math::Vec3;
		for (x, slice) in row.chunks_mut(stride).enumerate() {
			let pos = *transform * Vec3::new(x as f32, y as f32, 0.);
			interpolation.interpolate_into(&pos.xy(), channels, &size, src, slice);
		}
	});
}

// pub fn blend_into<
// 	'srca,
// 	'srcb,
// 	'dest,
// >(
// 	_size: Extent2<u32>,
// 	_blend_mode: &Blend,
// 	_channels: Channel,
// 	_source_a: &'srca Pixels,
// 	_source_b: &'srcb Pixels,
// 	_dest: &'dst mut Pixel,
// ) {
// }

#[cfg(test)]
mod tests {
	use super::*;
}

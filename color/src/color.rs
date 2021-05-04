use vek::ops::Lerp;

pub trait Color: Default {
	const SIZE: usize;

	fn to_slice(&self) -> &[u8];
	fn to_slice_mut(&mut self) -> &mut [u8];
	fn from_slice(slice: &[u8]) -> &Self;
	fn from_slice_mut(slice: &mut [u8]) -> &mut Self;
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Alpha<C: Color> {
	pub color: C,
	pub alpha: u8,
}

impl<C: Color> Alpha<C> {
	pub fn new(color: C, alpha: u8) -> Self {
		Self { color, alpha }
	}
}

impl<C: Color> Color for Alpha<C> {
	const SIZE: usize = 1 + std::mem::size_of::<C>();

	fn to_slice(&self) -> &[u8] {
		unsafe {
			std::slice::from_raw_parts(
				self as *const Self as *const u8,
				std::mem::size_of::<Self>(),
			)
		}
	}

	fn to_slice_mut(&mut self) -> &mut [u8] {
		unsafe {
			std::slice::from_raw_parts_mut(
				self as *mut Self as *mut u8,
				std::mem::size_of::<Self>(),
			)
		}
	}

	fn from_slice(slice: &[u8]) -> &Self {
		assert_eq!(slice.len(), std::mem::size_of::<Self>());
		unsafe { &*(slice.as_ptr() as *const Self) }
	}

	fn from_slice_mut(slice: &mut [u8]) -> &mut Self {
		assert_eq!(slice.len(), std::mem::size_of::<Self>());
		unsafe { &mut *(slice.as_ptr() as *mut Self) }
	}
}

impl<C: Color> Default for Alpha<C> {
	fn default() -> Self {
		Self {
			color: Default::default(),
			alpha: 255,
		}
	}
}

impl<C: Color> std::ops::Deref for Alpha<C> {
	type Target = C;

	fn deref(&self) -> &C {
		&self.color
	}
}

impl<C: Color> std::ops::DerefMut for Alpha<C> {
	fn deref_mut(&mut self) -> &mut C {
		&mut self.color
	}
}

macro_rules! define_color {
	($name:ident, ($($comp:ident),+), $type:ty) => {
		#[repr(C)]
		#[derive(Debug, Default, Clone, Copy, PartialEq)]
		pub struct $name {
			$(pub $comp: $type),+
		}

		impl $name {
			pub fn new($($comp: $type),+) -> Self {
				Self { $($comp: $comp,)+ }
			}
		}

		impl Color for $name {
			const SIZE: usize = std::mem::size_of::<Self>();

			fn to_slice(&self) -> &[u8] {
				unsafe { std::slice::from_raw_parts(self as *const Self as *const u8, std::mem::size_of::<Self>()) }
			}

			fn to_slice_mut(&mut self) -> &mut [u8] {
				unsafe { std::slice::from_raw_parts_mut(self as *mut Self as *mut u8, std::mem::size_of::<Self>()) }
			}

			fn from_slice(slice: &[u8]) -> &Self {
				assert_eq!(slice.len(), std::mem::size_of::<Self>());
				unsafe { &*(slice.as_ptr() as *const Self) }
			}

			fn from_slice_mut(slice: &mut [u8]) -> &mut Self {
				assert_eq!(slice.len(), std::mem::size_of::<Self>());
				unsafe { &mut *(slice.as_ptr() as *mut Self) }
			}
		}
	};
}

define_color!(Luma, (luma), u8);
pub type Lumaa = Alpha<Luma>;
define_color!(Rgb, (red, green, blue), u8);
pub type Rgba = Alpha<Rgb>;
define_color!(Uv, (u, v), f32);
define_color!(Normal, (x, y, z), f32);

impl From<Luma> for Rgb {
	fn from(value: Luma) -> Self {
		Rgb::new(value.luma, value.luma, value.luma)
	}
}
impl From<Rgb> for Luma {
	fn from(value: Rgb) -> Self {
		use palette::IntoColor;
		let luma: palette::LinLuma<palette::white_point::D65, f32> = palette::rgb::LinSrgb::new(
			(value.red as f32) / 255f32,
			(value.green as f32) / 255f32,
			(value.blue as f32) / 255f32,
		)
		.into_luma();
		Luma::new((luma.luma * 255f32) as u8)
	}
}
impl From<Uv> for Rgb {
	fn from(value: Uv) -> Self {
		Rgb::new((value.u * 255f32) as u8, (value.v * 255f32) as u8, 0)
	}
}
impl From<Rgb> for Uv {
	fn from(value: Rgb) -> Self {
		Uv::new((value.red as f32) / 255f32, (value.green as f32) / 255f32)
	}
}
impl From<Normal> for Rgb {
	fn from(value: Normal) -> Self {
		Rgb::new(
			(value.x * 255f32) as u8,
			(value.y * 255f32) as u8,
			(value.z * 255f32) as u8,
		)
	}
}
impl From<Rgb> for Normal {
	fn from(value: Rgb) -> Self {
		Normal::new(
			(value.red as f32) / 255f32,
			(value.green as f32) / 255f32,
			(value.blue as f32) / 255f32,
		)
	}
}
impl<C: Color> From<C> for Alpha<C> {
	fn from(value: C) -> Self {
		Alpha::new(value, 255)
	}
}
impl From<Alpha<Luma>> for Alpha<Rgb> {
	fn from(value: Alpha<Luma>) -> Self {
		Alpha::new(value.color.into(), value.alpha)
	}
}
impl From<Alpha<Rgb>> for Alpha<Luma> {
	fn from(value: Alpha<Rgb>) -> Self {
		Alpha::new(value.color.into(), value.alpha)
	}
}
impl From<Alpha<Uv>> for Alpha<Rgb> {
	fn from(value: Alpha<Uv>) -> Self {
		Alpha::new(value.color.into(), value.alpha)
	}
}
impl From<Alpha<Normal>> for Alpha<Rgb> {
	fn from(value: Alpha<Normal>) -> Self {
		Alpha::new(value.color.into(), value.alpha)
	}
}
impl Lerp<f32> for Luma {
	type Output = Luma;

	fn lerp_unclamped(from: Self, to: Self, factor: f32) -> Self::Output {
		Luma {
			luma: Lerp::lerp_unclamped(&from.luma, &to.luma, factor),
		}
	}
}
impl Lerp<f32> for Rgb {
	type Output = Rgb;

	fn lerp_unclamped(from: Self, to: Self, factor: f32) -> Self::Output {
		use palette::{LinSrgb, Mix};
		let a = LinSrgb::new(
			from.red as f32 / 255f32,
			from.green as f32 / 255f32,
			from.blue as f32 / 255f32,
		);
		let b = LinSrgb::new(
			to.red as f32 / 255f32,
			to.green as f32 / 255f32,
			to.blue as f32 / 255f32,
		);
		let c = a.mix(&b, factor);
		Rgb::new(
			(c.red * 255f32) as u8,
			(c.green * 255f32) as u8,
			(c.blue * 255f32) as u8,
		)
	}
}
impl Lerp<f32> for Uv {
	type Output = Uv;

	fn lerp_unclamped(from: Self, to: Self, factor: f32) -> Self::Output {
		use vek::vec::repr_c::vec2::Vec2;
		let vr = Vec2::lerp(Vec2::new(from.u, from.v), Vec2::new(to.u, to.v), factor);
		Uv::new(vr.x, vr.y)
	}
}
impl Lerp<f32> for Normal {
	type Output = Normal;

	fn lerp_unclamped(from: Self, to: Self, factor: f32) -> Self::Output {
		use vek::vec::repr_c::vec3::Vec3;
		let vr = Vec3::slerp(
			Vec3::new(from.x, from.y, from.z),
			Vec3::new(to.x, to.y, to.z),
			factor,
		);
		Normal::new(vr.x, vr.y, vr.z)
	}
}
impl<C: Lerp<f32, Output = C> + Copy + Color> Lerp<f32> for Alpha<C> {
	type Output = Alpha<<C as Lerp>::Output>;

	fn lerp_unclamped(from: Self, to: Self, factor: f32) -> Self::Output {
		Alpha {
			color: Lerp::lerp_unclamped(from.color, to.color, factor),
			alpha: Lerp::lerp_unclamped(&from.alpha, &to.alpha, factor),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn color_sizes() {
		assert_eq!(std::mem::size_of::<Luma>(), 1);
		assert_eq!(std::mem::size_of::<Lumaa>(), 2);
		assert_eq!(std::mem::size_of::<Rgb>(), 3);
		assert_eq!(std::mem::size_of::<Rgba>(), 4);
		assert_eq!(std::mem::size_of::<Uv>(), 8);
		assert_eq!(std::mem::size_of::<Normal>(), 12);
	}

	#[test]
	fn color_conversion() {
		let luma_to_rgb: Rgb = Luma::new(128).into();
		assert_eq!(luma_to_rgb, Rgb::new(128, 128, 128));
		let luma_to_lumaa: Lumaa = Luma::new(128).into();
		assert_eq!(luma_to_lumaa, Alpha::new(Luma::new(128), 255));
		let lumaa_to_luma: Luma = *Alpha::new(Luma::new(128), 255);
		assert_eq!(lumaa_to_luma, Luma::new(128));
		let rgb_to_luma: Luma = Rgb::new(128, 128, 128).into();
		assert_eq!(rgb_to_luma, Luma::new(128));
		let rgb_to_rgba: Rgba = Rgb::new(1, 2, 3).into();
		assert_eq!(rgb_to_rgba, Alpha::new(Rgb::new(1, 2, 3), 255));
		let rgba_to_rgb: Rgb = *Alpha::new(Rgb::new(1, 2, 3), 255);
		assert_eq!(rgba_to_rgb, Rgb::new(1, 2, 3));
		let uv_to_rgb: Rgb = Uv::new(0.5, 0.5).into();
		assert_eq!(uv_to_rgb, Rgb::new(127, 127, 0));
		let rgb_to_uv: Uv = Rgb::new(127, 127, 0).into();
		assert_eq!(rgb_to_uv, Uv::new(0.49803922, 0.49803922));
		let nor_to_rgb: Rgb = Normal::new(0.5, 0.5, 0.5).into();
		assert_eq!(nor_to_rgb, Rgb::new(127, 127, 127));
		let rgb_to_nor: Normal = Rgb::new(120, 240, 90).into();
		assert_eq!(rgb_to_nor, Normal::new(0.47058824, 0.9411765, 0.3529412));
		let lumaa_to_rgba: Rgba = Alpha::new(Luma::new(128), 128).into();
		assert_eq!(lumaa_to_rgba, Alpha::new(Rgb::new(128, 128, 128), 128));
	}
}

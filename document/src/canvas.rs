use crate::prelude::*;
use bytes::Bytes;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct Canvas {
	pub size: Extent2<u32>,
	pub channels: Channel,
	pub data: Bytes,
	pub stencils: VecDeque<Arc<PositionnedStencil>>,
}

impl Canvas {
	pub fn new(size: Extent2<u32>, channels: Channel, data: Vec<u8>) -> Self {
		Canvas {
			size,
			channels,
			data: Bytes::from(data),
			stencils: VecDeque::new(),
		}
	}

	pub fn apply_stencil(&self, position: Vec2<u32>, stencil: Stencil) -> Canvas {
		let mut cloned = self.clone();
		cloned
			.stencils
			.push_front(Arc::new(PositionnedStencil { position, stencil }));
		cloned
	}
}

impl std::ops::Index<(&u32, &u32)> for Canvas {
	type Output = [u8];

	fn index(&self, index: (&u32, &u32)) -> &Self::Output {
		let index = (index.1 + self.size.w * index.0) as usize;
		self.index(index)
	}
}

impl std::ops::Index<usize> for Canvas {
	type Output = [u8];

	fn index(&self, index: usize) -> &Self::Output {
		let stride = self.channels.len() as usize;
		let x = index as u32 % self.size.w;
		let y = index as u32 / self.size.w;
		for stencil in self.stencils.iter() {
			if x >= stencil.position.x
				&& x <= stencil.position.x + stencil.stencil.size.w
				&& y >= stencil.position.y
				&& y <= stencil.position.y + stencil.stencil.size.h
			{
				let x = x - stencil.position.x;
				let y = y - stencil.position.y;
				if let Ok(data) = stencil.stencil.try_get((&x, &y)) {
					return data;
				}
			}
		}
		&self.data[(index * stride)..((index + 1) * stride)]
	}
}

impl std::ops::Add<Stencil> for &Canvas {
	type Output = Canvas;

	fn add(self, other: Stencil) -> Self::Output {
		self.apply_stencil(Vec2::new(0, 0), other)
	}
}

#[cfg(test)]
mod tests {
	use crate::prelude::*;
	use collections::bitvec;

	#[test]
	fn test_canvas_index() {
		let a = Canvas::new(Extent2::new(2, 2), Channel::A, vec![1u8, 2, 3, 4]);
		assert_eq!(&a[0], &[1u8][..]);
		assert_eq!(&a[1], &[2u8][..]);
		assert_eq!(&a[2], &[3u8][..]);
		assert_eq!(&a[3], &[4u8][..]);

		let stencil = Stencil {
			size: Extent2::new(2, 2),
			mask: bitvec![Lsb0, u8; 1, 0, 0, 1],
			channels: Channel::A,
			data: vec![8u8, 9],
		};

		let b = a.apply_stencil(Vec2::new(0, 0), stencil);
		assert_eq!(&b[0], &[8u8][..]);
		assert_eq!(&b[1], &[2u8][..]);
		assert_eq!(&b[2], &[3u8][..]);
		assert_eq!(&b[3], &[9u8][..]);

		let stencil = Stencil {
			size: Extent2::new(2, 2),
			mask: bitvec![Lsb0, u8; 0, 1, 1, 0],
			channels: Channel::A,
			data: vec![11u8, 12],
		};

		let c = a.apply_stencil(Vec2::new(0, 0), stencil);
		assert_eq!(&c[0], &[1u8][..]);
		assert_eq!(&c[1], &[11u8][..]);
		assert_eq!(&c[2], &[12u8][..]);
		assert_eq!(&c[3], &[4u8][..]);
	}
}

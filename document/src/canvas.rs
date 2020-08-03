use crate::prelude::*;
use collections::{Bytes, BytesChain};

pub struct Canvas {
	pub size: Extent2<u32>,
	pub region: Extent2<u32>,
	pub channels: Channel,
	pub data: BytesChain,
}

impl Canvas {
	/// Reorder data to form continuous 2d tiles in memory.
	/// When applying stencil, only affected tiles should
	/// be copied to a new memory location. Canvas will retain
	/// immutability and use less memory over multiple mutation.
	///
	/// Group into 2x2 tiles
	///
	/// initial                 canvas 1               canvas 2
	///     |                     |                      |
	///     v                     v                      v
	///    abc -> reordered   -> ab.e -> mutate a    -> jk.e
	///    def    in 2x2 grid    cd.f    abcd copied    lm.f
	///    ghi                   ....    to jklm        ....
	///                          gh.i                   gh.i
	///
	/// Region abcd will be kept in memory as long as canvas 1
	/// is referenced to (undo history, etc).
	///
	pub fn new(size: Extent2<u32>, channels: Channel, data: Vec<u8>) -> Self {
		let data = {
			let mut chain = BytesChain::new();
			chain.push(Bytes::from(data));
			chain
		};
		Canvas {
			size,
			region: size,
			channels,
			data,
		}
	}
}
#[allow(non_snake_case, unused_variables)]
#[cfg(test)]
mod tests {
	// use math::Extent2;

	/// 2x2 tiles
	///
	/// abcdefghijklmnopqrstuvwxy
	/// abfgcdhiejklpqmnrsotuvwxy
	///
	/// a = 0  -> 0  ~ 0,0 -> 0,0
	/// b = 1  -> 1  ~ 1,0 -> 1,0
	/// c = 2  -> 4  ~ 2,0 -> 4,0
	/// d = 3  -> 5  ~ 3,0 -> 0,1
	/// e = 4  -> 8  ~ 4,0 -> 3,1
	///
	/// f = 5  -> 2  ~ 0,1 -> 2,0
	/// g = 6  -> 3  ~ 1,1 -> 3,0
	/// h = 7  -> 6  ~ 2,1 -> 1,1
	/// i = 8  -> 7  ~ 3,1 -> 2,1
	/// j = 9  -> 9  ~ 4,1 -> 4,1
	///
	/// k = 10 -> 10 ~ 0,2 -> 0,2
	/// l = 11 -> 11 ~ 1,2 -> 1,2
	/// m = 12 -> 14 ~ 2,2 -> 4,2
	/// n = 13 -> 15 ~ 3,2 -> 0,3
	/// o = 14 -> 18 ~ 4,2 -> 3,3
	///
	/// p = 15 -> 12 ~ 0,3 -> 2,2
	/// q = 16 -> 13 ~ 1,3 -> 3,2
	/// r = 17 -> 16 ~ 2,3 -> 1,3
	/// s = 18 -> 17 ~ 3,3 -> 2,3
	/// t = 19 -> 19 ~ 4,3 -> 4,3
	///
	/// u = 20 -> 20 ~ 0,4 -> 0,4
	/// v = 21 -> 21 ~ 1,4 -> 1,4
	/// w = 22 -> 22 ~ 2,4 -> 2,4
	/// x = 23 -> 23 ~ 3,4 -> 3,4
	/// y = 24 -> 24 ~ 4,4 -> 4,4
	///

	// https://pdfs.semanticscholar.org/3030/22b1c442f543d6794a2171e3dfcd9ff149cb.pdf
	// https://en.wikipedia.org/wiki/Z-order_curve
	fn Srm(M: usize, N: usize, x: usize, y: usize) -> usize {
		x + M * y
	}
	fn Scm(M: usize, N: usize, x: usize, y: usize) -> usize {
		N * x + y
	}
	// fn Sbrm(P: usize, Q: usize, M: usize, N: usize, x: usize, y: usize) -> usize {
	// 	(P * Q) * Srm(M / P, N / Q, x / P, y / Q) + Srm(P, Q, x % P, y % Q)
	// }
	fn Sbrm(P: usize, Q: usize, M: usize, N: usize, x: usize, y: usize) -> usize {
		assert!(P <= M);
		assert!(Q <= N);
		let W = (M as f64 / P as f64).ceil() as usize * P;
		let H = (N as f64 / Q as f64).ceil() as usize * Q;
		let A = if x / P == M / P {
			M % P
		} else {
			P
		};
		let B = if y / Q == N / Q {
			N % Q
		} else {
			Q
		};
		// #[rustfmt::skip]
		// println!(
		// 	"{},{} in {}x{} ({}x{}) | ({}x{}) | {} x ({}x{}) | -> {} * {} + {} = {:<2} | {} * {} + {} = {:<2} | {} * {} + {} = {:<2} | {} * {} + {} - {} * {} - {} * {} = {:<2}",
		// 	 x, y,    W, H,  M, N,
		// 	 A, B,
		// 	 y, P - (M % P), Q,

		// 	(P * Q), Srm(M / P, N / Q, x / P, y / Q), Srm(P, Q, x % P, y % Q),
		// 	(P * Q) * Srm(M / P, N / Q, x / P, y / Q) + Srm(P, Q, x % P, y % Q),

		// 	(P * Q), Srm(M / P, N / Q, x / P, y / Q), Srm(A, B, x % P, y % Q),
		// 	(P * Q) * Srm(M / P, N / Q, x / P, y / Q) + Srm(A, B, x % P, y % Q),

		// 	(P * Q), Srm(W / P, H / Q, x / P, y / Q), Srm(A, B, x % P, y % Q),
		// 	(P * Q) * Srm(W / P, H / Q, x / P, y / Q) + Srm(A, B, x % P, y % Q),

		// 	// fonctionne à 3x3
		// 	// (P * Q), Srm(W / P, H / Q, x / P, y / Q), Srm(A, B, x % P, y % Q), (y / Q), (P - (M % P)) * Q, (y / Q) * (x / P), (Q - (N % Q)) * P,
		// 	// (P * Q) * Srm(W / P, H / Q, x / P, y / Q) + Srm(A, B, x % P, y % Q) - (y / Q) * ((P - (M % P)) * Q) - (y / Q) * (x / P) * ((Q - (N % Q)) * P)
		// 	(P * Q), Srm(W / P, H / Q, x / P, y / Q), Srm(A, B, x % P, y % Q), (y / Q), (P - (M % P)) * Q, (y / (Q * (N / Q))) * (x / P), (Q - (N % Q)) * P,
		// 	(P * Q) * Srm(W / P, H / Q, x / P, y / Q) + Srm(A, B, x % P, y % Q) - (y / Q) * (P - (M % P)) * Q - (y / (Q * (N / Q))) * (x / P) * (Q - (N % Q)) * P
		// );
		(P * Q) * Srm(W / P, H / Q, x / P, y / Q) + Srm(A, B, x % P, y % Q) - (y / Q) * (P - (M % P)) * Q - (y / (Q * (N / Q))) * (x / P) * (Q - (N % Q)) * P
	}

	#[test]
	fn test_map() {
		#[rustfmt::skip]
		let a = vec![
			'a', 'b', 'c', 'd', 'e', 
			'f', 'g', 'h', 'i', 'j', 
			'k', 'l', 'm', 'n', 'o', 
			'p', 'q', 'r', 's', 't', 
			'u', 'v', 'w', 'x', 'y',
			'z', '0', '1', '2', '3',
			'4', '5', '6', '7', '8', '9'
		];
		#[rustfmt::skip]
		let b2x2 = vec![
			'a', 'b', 'e', 'f', 'i',
			'c', 'd', 'g', 'h', 'j',
			'k', 'l', 'o', 'p', 's',
			'm', 'n', 'q', 'r', 't', 
			'u', 'v', 'w', 'x', 'y',
		];
		#[rustfmt::skip]
		let b3x3 = vec![
			'a', 'b', 'c', 'j', 'k', //
			'd', 'e', 'f', 'l', 'm', //
			'g', 'h', 'i', 'n', 'o', // 
			'p', 'q', 'r', 'v', 'w', //
			's', 't', 'u', 'x', 'y',
		];

		
		let (M, N) = (5usize, 5usize);
		let (P, Q) = (2usize, 2usize);
		let mut b = vec![' '; M * N];
		
		// println!("{}x{} with tile {}x{}", M, N, P, Q);
		for i in 0..(M * N) {
			let x = i % M;
			let y = i / M;
			// println!(
			// 	"                                                       {:<2} => {:<2},{:<2} => Srm = {}, Scm = {}, Sbrm = {}, Sbrm2 = {}",
			// 	i,
			// 	x,
			// 	y,
			// 	a[Srm(M, N, x, y)],
			// 	a[Scm(P, Q, x, y)],
			// 	a[Sbrm(P, Q, M, N, x, y)], // works if MxN is multiple of PxQ
			// 	a[Sbrm2(P, Q, M, N, x, y)],
			// );
			// Sbrm2(P, Q, M, N, x, y);
			b[i] = a[Sbrm(P, Q, M, N, x, y)];
		}

		assert_eq!(b, b2x2);

		let (M, N) = (5usize, 5usize);
		let (P, Q) = (3usize, 3usize);
		let mut b = vec![' '; M * N];
		// println!("{}x{} with tile {}x{}", M, N, P, Q);
		for i in 0..M * N {
			let x = i % M;
			let y = i / M;
			// println!(
			// 	"                                                       {:<2} => {:<2},{:<2} => Srm = {}, Scm = {}, Sbrm = {}, Sbrm2 = {}",
			// 	i,
			// 	x,
			// 	y,
			// 	a[Srm(M, N, x, y)],
			// 	a[Scm(P, Q, x, y)],
			// 	a[Sbrm(P, Q, M, N, x, y)], // works if MxN is multiple of PxQ
			// 	a[Sbrm2(P, Q, M, N, x, y)],
			// );
			// Sbrm2(P, Q, M, N, x, y);
			b[i] = a[Sbrm(P, Q, M, N, x, y)];
		}
		assert_eq!(b, b3x3);

		let (M, N) = (5usize, 5usize);
		let (P, Q) = (5usize, 5usize);
		let mut b = vec![' '; M * N];
		for i in 0..M * N {
			let x = i % M;
			let y = i / M;
			b[i] = a[Sbrm(P, Q, M, N, x, y)];
		}
		assert_eq!(b[..], a[0..M*N]);
	}
}

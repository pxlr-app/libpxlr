#[inline]
fn row_major(m: usize, x: usize, y: usize) -> usize {
	x + m * y
}

/// Map row-major to blocked-row-major index.
///
/// Given an 2d array flattened with row-major indexes,
/// map each index to a PxQ sub-grid of blocked row-major.
///
/// Given an array 5x5 linear data
///   abcdefghijklmnopqrstuvwxy
/// remap to form blocks of 2x2 in linear data
///   abeficdghjklopsmnqrtuvwxy
///
pub fn map_rm_to_brm(mut p: usize, mut q: usize, m: usize, n: usize, i: usize) -> usize {
	assert!(m > 0);
	assert!(n > 0);
	assert!(p > 0);
	assert!(q > 0);
	p = p.min(m);
	q = q.min(n);
	let x = i % m;
	let y = i / m;
	let w = (m as f64 / p as f64).ceil() as usize * p;
	let a = if x / p == m / p { m % p } else { p };
	let k = if p == m { 0 } else { p - (m % p) };
	let l = if q == n { 0 } else { q - (n % q) };
	#[rustfmt::skip]
	let index = (p * q) * row_major(w / p, x / p, y / q) // nb block before current block times block size
				+ row_major(a, x % p, y % q)             // + row-major index within current block
				- (y / q) * k * q                        // - incomplete (y-axis) block times missing cols before current block
				- (y / (q * (n / q))) * (x / p) * l * p; // - incomplete (x axis) block times missing rows before current block
	index
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_map_rm_to_brm() {
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

		let (m, n) = (5usize, 5usize);
		let (p, q) = (2usize, 2usize);
		let mut b = vec![' '; m * n];
		#[rustfmt::skip]
		let c = vec![
			'a', 'b', 'e', 'f', 'i',
			'c', 'd', 'g', 'h', 'j',
			'k', 'l', 'o', 'p', 's',
			'm', 'n', 'q', 'r', 't', 
			'u', 'v', 'w', 'x', 'y',
		];
		for i in 0..(m * n) {
			b[i] = a[map_rm_to_brm(p, q, m, n, i)];
		}
		assert_eq!(b, c);

		let (m, n) = (5usize, 5usize);
		let (p, q) = (3usize, 3usize);
		let mut b = vec![' '; m * n];
		#[rustfmt::skip]
		let c = vec![
			'a', 'b', 'c', 'j', 'k',
			'd', 'e', 'f', 'l', 'm',
			'g', 'h', 'i', 'n', 'o',
			'p', 'q', 'r', 'v', 'w',
			's', 't', 'u', 'x', 'y',
		];
		for i in 0..m * n {
			b[i] = a[map_rm_to_brm(p, q, m, n, i)];
		}
		assert_eq!(b, c);

		let (m, n) = (5usize, 5usize);
		let (p, q) = (5usize, 5usize);
		let mut b = vec![' '; m * n];
		for i in 0..m * n {
			b[i] = a[map_rm_to_brm(p, q, m, n, i)];
		}
		assert_eq!(b[..], a[0..m * n]);

		let (m, n) = (5usize, 5usize);
		let (p, q) = (4usize, 2usize);
		let mut b = vec![' '; m * n];
		#[rustfmt::skip]
		let c = vec![
			'a', 'b', 'c', 'd', 'i',
			'e', 'f', 'g', 'h', 'j',
			'k', 'l', 'm', 'n', 's',
			'o', 'p', 'q', 'r', 't',
			'u', 'v', 'w', 'x', 'y',
		];
		for i in 0..m * n {
			b[i] = a[map_rm_to_brm(p, q, m, n, i)];
		}
		assert_eq!(b, c);

		let (m, n) = (5usize, 5usize);
		let (p, q) = (2usize, 5usize);
		let mut b = vec![' '; m * n];
		#[rustfmt::skip]
		let c = vec![
			'a', 'b', 'k', 'l', 'u',
			'c', 'd', 'm', 'n', 'v',
			'e', 'f', 'o', 'p', 'w',
			'g', 'h', 'q', 'r', 'x',
			'i', 'j', 's', 't', 'y',
		];
		for i in 0..m * n {
			b[i] = a[map_rm_to_brm(p, q, m, n, i)];
		}
		assert_eq!(b, c);

		let (m, n) = (5usize, 5usize);
		let (p, q) = (5usize, 2usize);
		let mut b = vec![' '; m * n];
		for i in 0..m * n {
			b[i] = a[map_rm_to_brm(p, q, m, n, i)];
		}
		assert_eq!(b[..], a[0..m * n]);

		let (m, n) = (5usize, 5usize);
		let (p, q) = (10usize, 10usize);
		let mut b = vec![' '; m * n];
		for i in 0..m * n {
			b[i] = a[map_rm_to_brm(p, q, m, n, i)];
		}
		assert_eq!(b[..], a[0..m * n]);

		let (m, n) = (5usize, 5usize);
		let (p, q) = (10usize, 2usize);
		let mut b = vec![' '; m * n];
		for i in 0..m * n {
			b[i] = a[map_rm_to_brm(p, q, m, n, i)];
		}
		assert_eq!(b[..], a[0..m * n]);

		let (m, n) = (5usize, 5usize);
		let (p, q) = (2usize, 10usize);
		let mut b = vec![' '; m * n];
		#[rustfmt::skip]
		let c = vec![
			'a', 'b', 'k', 'l', 'u',
			'c', 'd', 'm', 'n', 'v',
			'e', 'f', 'o', 'p', 'w',
			'g', 'h', 'q', 'r', 'x',
			'i', 'j', 's', 't', 'y',
		];
		for i in 0..m * n {
			b[i] = a[map_rm_to_brm(p, q, m, n, i)];
		}
		assert_eq!(b, c);
	}
}
